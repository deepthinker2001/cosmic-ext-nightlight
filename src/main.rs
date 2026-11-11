#![allow(dead_code)]
mod gamma;

use std::sync::mpsc;
use std::thread;

use cosmic::app::{Core, Task};
use cosmic::cosmic_config::{Config, ConfigGet, ConfigSet};
use cosmic::iced::futures::channel::oneshot;
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
/// so the UI never waits on the compositor.
struct Worker {
    tx: mpsc::Sender<(bool, Reply)>,
}

impl Worker {
    fn spawn() -> Self {
        let (tx, rx) = mpsc::channel::<(bool, Reply)>();
        thread::spawn(move || {
            let mut backend: Option<gamma::GammaBackend> = None;
            while let Ok((enable, reply)) = rx.recv() {
                let result = (|| {
                    if backend.is_none() {
                        backend = Some(gamma::GammaBackend::new().map_err(|e| e.to_string())?);
                    }
                    let b = backend.as_mut().unwrap();
                    if enable {
                        b.enable(gamma::DEFAULT_TEMPERATURE).map_err(|e| e.to_string())?;
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
                rx.await.unwrap_or_else(|_| Err("gamma worker stopped".to_string()))
            },
            move |r| cosmic::Action::App(Message::Applied(enable, r)),
        )
    }
}

fn config() -> Option<Config> {
    Config::new(APP_ID, CONFIG_VERSION).ok()
}

struct Nightlight {
    core: Core,
    /// Real state: true only once the backend confirmed night light is on.
    enabled: bool,
    /// A backend request is in flight.
    busy: bool,
    worker: Worker,
}

#[derive(Clone, Debug)]
enum Message {
    Toggle,
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
        let saved = config()
            .and_then(|c| c.get::<bool>(CONFIG_KEY_ENABLED).ok())
            .unwrap_or(false);
        let worker = Worker::spawn();
        let task = if saved { worker.request(true) } else { Task::none() };
        (Self { core, enabled: false, busy: saved, worker }, task)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Toggle => {
                if self.busy {
                    return Task::none();
                }
                self.busy = true;
                return self.worker.request(!self.enabled);
            }
            Message::Applied(want, result) => {
                self.busy = false;
                match result {
                    Ok(()) => {
                        self.enabled = want;
                        if let Some(c) = config() {
                            if let Err(e) = c.set(CONFIG_KEY_ENABLED, want) {
                                eprintln!("nightlight: cannot save state: {e}");
                            }
                        }
                    }
                    Err(e) => {
                        // Icon follows the real state: the request failed, so
                        // the state is unchanged (off after a failed enable).
                        eprintln!("nightlight: {e}");
                        if want {
                            self.enabled = false;
                        }
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let icon = if self.enabled { ICON_ON } else { ICON_OFF };
        self.core
            .applet
            .icon_button(icon)
            .on_press(Message::Toggle)
            .into()
    }
}
