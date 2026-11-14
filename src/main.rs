// Some backend items (temperature changes, probes) aren't used by the applet yet.
#[allow(dead_code)]
mod gamma;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use cosmic::app::{Core, Task};
use cosmic::cosmic_config::{Config, ConfigGet, ConfigSet};
use cosmic::iced::core::window;
use cosmic::iced::futures::channel::oneshot;
use cosmic::iced::window::Id;
use cosmic::surface::action::{app_popup, destroy_popup};
use cosmic::widget::{list_column, mouse_area, settings, toggler};
use cosmic::Element;

const APP_ID: &str = "io.github.cosmic_ext.Nightlight";
const ICON_ON: &str = "io.github.cosmic_ext.Nightlight-on-symbolic";
const ICON_OFF: &str = "io.github.cosmic_ext.Nightlight-off-symbolic";
const CONFIG_VERSION: u64 = 1;
const CONFIG_KEY_ENABLED: &str = "enabled";

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<Nightlight>(())
}

type Reply = oneshot::Sender<Result<(), String>>;

/// Handle to a worker thread that owns the (blocking) Wayland gamma backend,
/// so the UI never waits on the compositor. When the applet exits the process
/// ends and the Wayland connection closes, which makes the compositor reset
/// gamma to neutral even though the detached thread's backend isn't dropped.
struct Worker {
    tx: mpsc::Sender<(bool, Reply)>,
}

/// How often the idle worker drains compositor events.
const POLL_INTERVAL: Duration = Duration::from_millis(500);

impl Worker {
    fn spawn() -> Self {
        let (tx, rx) = mpsc::channel::<(bool, Reply)>();
        thread::spawn(move || {
            let mut backend: Option<gamma::GammaBackend> = None;
            loop {
                let (enable, reply) = match rx.recv_timeout(POLL_INTERVAL) {
                    Ok(req) => req,
                    Err(mpsc::RecvTimeoutError::Timeout) => {
                        // Keep reading events while idle so the socket never
                        // fills; on a broken connection start afresh next time.
                        if let Some(b) = backend.as_mut() {
                            if let Err(e) = b.dispatch() {
                                eprintln!("nightlight: {e}");
                                backend = None;
                            }
                        }
                        continue;
                    }
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                };
                let result = (|| {
                    if backend.is_none() {
                        backend = Some(gamma::GammaBackend::new().map_err(|e| e.to_string())?);
                    }
                    let b = backend.as_mut().unwrap();
                    if enable {
                        if let Err(e) = b.enable(gamma::DEFAULT_TEMPERATURE) {
                            // Partway failure: don't leave some outputs warm.
                            let _ = b.disable();
                            return Err(e.to_string());
                        }
                        if b.active_outputs() == 0 {
                            let _ = b.disable();
                            return Err("no output accepted gamma control".to_string());
                        }
                        Ok(())
                    } else {
                        b.disable().map_err(|e| e.to_string())
                    }
                })();
                let _ = reply.send(result);
            }
        });
        Self { tx }
    }

    fn request(&self, enable: bool) -> Task<Message> {
        let (reply, rx) = oneshot::channel();
        let sent = self.tx.send((enable, reply)).is_ok();
        Task::perform(
            async move {
                if !sent {
                    return Err("gamma worker stopped".to_string());
                }
                rx.await
                    .unwrap_or_else(|_| Err("gamma worker stopped".to_string()))
            },
            move |r| cosmic::Action::App(Message::Applied(enable, r)),
        )
    }
}

/// What a toggle press asks for: `None` while a request is in flight.
fn toggle_request(enabled: bool, busy: bool) -> Option<bool> {
    (!busy).then_some(!enabled)
}

/// New real state and the value to save after a backend reply to a request
/// for `want`. Success follows the request and is saved. A failure leaves the
/// state off after a failed enable (the icon shows reality) and saves nothing:
/// a failed startup restore deliberately keeps the saved flag set, so the next
/// start retries (e.g. the compositor wasn't ready at login) instead of
/// silently forgetting the user's choice.
fn applied(enabled: bool, want: bool, ok: bool) -> (bool, Option<bool>) {
    if ok {
        (want, Some(want))
    } else if want {
        (false, None)
    } else {
        (enabled, None)
    }
}

/// The open popup after the compositor closed popup `closed`: only the
/// popup we track is forgotten.
fn popup_after_close(popup: Option<Id>, closed: Id) -> Option<Id> {
    popup.filter(|id| *id != closed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closing_matching_popup_clears_it() {
        let id = Id::unique();
        assert_eq!(popup_after_close(Some(id), id), None);
    }

    #[test]
    fn closing_other_popup_keeps_it() {
        let (a, b) = (Id::unique(), Id::unique());
        assert_eq!(popup_after_close(Some(a), b), Some(a));
    }

    #[test]
    fn closing_with_none_open_is_noop() {
        assert_eq!(popup_after_close(None, Id::unique()), None);
    }

    #[test]
    fn toggle_flips_and_ignores_while_busy() {
        assert_eq!(toggle_request(false, false), Some(true));
        assert_eq!(toggle_request(true, false), Some(false));
        assert_eq!(toggle_request(false, true), None);
        assert_eq!(toggle_request(true, true), None);
    }

    #[test]
    fn success_follows_request_and_saves() {
        assert_eq!(applied(false, true, true), (true, Some(true)));
        assert_eq!(applied(true, false, true), (false, Some(false)));
    }

    #[test]
    fn failure_keeps_real_state_and_saves_nothing() {
        assert_eq!(applied(false, true, false), (false, None));
        assert_eq!(applied(true, false, false), (true, None));
    }
}

fn config() -> Option<Config> {
    Config::new(APP_ID, CONFIG_VERSION).ok()
}

/// Persisted settings, stored as one cosmic_config key per field. Adding a
/// field with a default needs no version bump: a missing key reads as its
/// default. Bump `CONFIG_VERSION` only when an existing key changes meaning.
#[derive(Clone, Debug, Default, PartialEq)]
struct Settings {
    /// The user's last choice: night light on.
    enabled: bool,
}

impl Settings {
    fn load(config: &Config) -> Self {
        Self {
            enabled: config.get::<bool>(CONFIG_KEY_ENABLED).unwrap_or(false),
        }
    }

    fn save_enabled(config: &Config, enabled: bool) -> Result<(), cosmic::cosmic_config::Error> {
        config.set(CONFIG_KEY_ENABLED, enabled)
    }
}

struct Nightlight {
    core: Core,
    /// Real state: true only once the backend confirmed night light is on.
    enabled: bool,
    /// A backend request is in flight.
    busy: bool,
    worker: Worker,
    /// The settings popup, while it is open.
    popup: Option<Id>,
}

#[derive(Clone, Debug)]
enum Message {
    Toggle,
    /// Right click on the icon: open or close the settings popup.
    ToggleSettings,
    PopupClosed(Id),
    Applied(bool, Result<(), String>),
}

impl cosmic::Application for Nightlight {
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Message>) {
        let saved = config().map(|c| Settings::load(&c)).unwrap_or_default().enabled;
        let worker = Worker::spawn();
        let task = if saved {
            worker.request(true)
        } else {
            Task::none()
        };
        (
            Self {
                core,
                enabled: false,
                busy: saved,
                worker,
                popup: None,
            },
            task,
        )
    }

    fn on_close_requested(&self, id: window::Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Toggle => {
                let Some(want) = toggle_request(self.enabled, self.busy) else {
                    return Task::none();
                };
                self.busy = true;
                return self.worker.request(want);
            }
            Message::ToggleSettings => {
                let action = match self.popup {
                    Some(id) => destroy_popup(id),
                    None => app_popup::<Nightlight>(
                        |_| Default::default(),
                        |state: &mut Nightlight| {
                            let id = Id::unique();
                            state.popup = Some(id);
                            state.core.applet.get_popup_settings(
                                state.core.main_window_id().unwrap(),
                                id,
                                None,
                                None,
                                None,
                            )
                        },
                        Some(Box::new(|state: &Nightlight| {
                            let content = list_column().add(settings::item(
                                "Night light",
                                toggler(state.enabled).on_toggle(|_| Message::Toggle),
                            ));
                            Element::from(state.core.applet.popup_container(content))
                                .map(cosmic::Action::App)
                        })),
                    ),
                };
                return cosmic::task::message(cosmic::Action::Surface(action));
            }
            Message::PopupClosed(id) => {
                self.popup = popup_after_close(self.popup, id);
            }
            Message::Applied(want, result) => {
                self.busy = false;
                if let Err(e) = &result {
                    eprintln!("nightlight: {e}");
                }
                let (enabled, save) = applied(self.enabled, want, result.is_ok());
                self.enabled = enabled;
                if let (Some(v), Some(c)) = (save, config()) {
                    if let Err(e) = Settings::save_enabled(&c, v) {
                        eprintln!("nightlight: cannot save state: {e}");
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let icon = if self.enabled { ICON_ON } else { ICON_OFF };
        // Left click toggles; right click opens the settings popup. The popup
        // button has no right-click hook, so a mouse area wraps it.
        let button = self
            .core
            .applet
            .icon_button(icon)
            .on_press(Message::Toggle);
        mouse_area(button)
            .on_right_press(Message::ToggleSettings)
            .into()
    }
}
