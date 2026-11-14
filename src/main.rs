// Some backend items (temperature changes, probes) aren't used by the applet yet.
#[allow(dead_code)]
mod gamma;
mod location;
mod requests;
mod saver;
mod solar;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use chrono::{Datelike, Timelike};
use cosmic::app::{Core, Task};
use cosmic::cosmic_config::{Config, ConfigGet, ConfigSet};
use cosmic::iced::core::window;
use cosmic::iced::futures::channel::oneshot;
use cosmic::iced::window::Id;
use cosmic::iced::Subscription;
use cosmic::surface::action::{app_popup, destroy_popup};
use cosmic::widget::{list_column, mouse_area, settings, slider, text, toggler};
use cosmic::Element;
use requests::{Requests, Target};
use saver::{Save, Saver};

const APP_ID: &str = "io.github.cosmic_ext.Nightlight";
const ICON_ON: &str = "io.github.cosmic_ext.Nightlight-on-symbolic";
const ICON_OFF: &str = "io.github.cosmic_ext.Nightlight-off-symbolic";
const CONFIG_VERSION: u64 = 1;
const CONFIG_KEY_ENABLED: &str = "enabled";
const CONFIG_KEY_TEMPERATURE: &str = "temperature";
// Keys from older versions, no longer read: `schedule_on`, `schedule_start`
// and `schedule_end` (the custom-times schedule, removed). They are left in
// place and ignored. `location_custom` is read only while
// `location_from_timezone` hasn't been saved yet (see `Settings::load`).
const CONFIG_KEY_AUTO_ON: &str = "auto_on";
const CONFIG_KEY_LOCATION_FROM_TIMEZONE: &str = "location_from_timezone";
const CONFIG_KEY_LOCATION_CUSTOM: &str = "location_custom";
const CONFIG_KEY_LOCATION_LAT: &str = "location_lat";
const CONFIG_KEY_LOCATION_LON: &str = "location_lon";
/// How often the auto ramp is checked. Also bounds how late it catches up
/// after the machine resumes from sleep.
const TICK: Duration = Duration::from_secs(20);

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<Nightlight>(())
}

type Reply = oneshot::Sender<Result<(), String>>;

/// Handle to a worker thread that owns the (blocking) Wayland gamma backend,
/// so the UI never waits on the compositor. When the applet exits the process
/// ends and the Wayland connection closes, which makes the compositor reset
/// gamma to neutral even though the detached thread's backend isn't dropped.
struct Worker {
    /// `Some(kelvin)` turns night light on (or retunes it); `None` turns it off.
    tx: mpsc::Sender<(Option<u32>, Reply)>,
}

/// How often the idle worker drains compositor events.
const POLL_INTERVAL: Duration = Duration::from_millis(500);

impl Worker {
    fn spawn() -> Self {
        let (tx, rx) = mpsc::channel::<(Option<u32>, Reply)>();
        thread::spawn(move || {
            let mut backend: Option<gamma::GammaBackend> = None;
            loop {
                let (target, reply) = match rx.recv_timeout(POLL_INTERVAL) {
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
                let _ = reply.send(handle_request(&mut backend, target));
            }
        });
        Self { tx }
    }

    /// Send `target` to the worker; the reply comes back as `Applied`.
    fn request(&self, target: Target) -> Task<Message> {
        let (reply, rx) = oneshot::channel();
        let sent = self.tx.send((target, reply)).is_ok();
        Task::perform(
            async move {
                if !sent {
                    return Err("gamma worker stopped".to_string());
                }
                rx.await
                    .unwrap_or_else(|_| Err("gamma worker stopped".to_string()))
            },
            move |r| cosmic::Action::App(Message::Applied(r)),
        )
    }

    /// Send `target` when there is one, else nothing.
    fn send(&self, target: Option<Target>) -> Task<Message> {
        target.map_or_else(Task::none, |t| self.request(t))
    }
}

/// Serve one worker request, connecting the backend on first use (or after
/// a broken connection). Kept out of the worker's loop body so the loop only
/// receives and replies. Not a hot path: `GammaBackend::new` runs only while
/// the backend is `None`, i.e. once per connection (and its `Vec::new` does
/// not allocate until an output is pushed); requests on a live connection
/// reuse the cached backend.
fn handle_request(
    backend: &mut Option<gamma::GammaBackend>,
    target: Option<u32>,
) -> Result<(), String> {
    if backend.is_none() {
        *backend = Some(gamma::GammaBackend::new().map_err(|e| e.to_string())?);
    }
    let Some(b) = backend.as_mut() else {
        return Err("gamma backend unavailable".to_string());
    };
    if let Some(kelvin) = target {
        if let Err(e) = b.enable(kelvin) {
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
}

/// The open popup after the compositor closed popup `closed`: only the
/// popup we track is forgotten.
fn popup_after_close(popup: Option<Id>, closed: Id) -> Option<Id> {
    popup.filter(|id| *id != closed)
}

/// Short hint for where a temperature sits between warm and cool.
fn temperature_hint(kelvin: u32) -> &'static str {
    match kelvin {
        ..=2699 => "Very warm",
        2700..=3999 => "Warm",
        4000..=5199 => "Neutral warm",
        _ => "Cool",
    }
}

/// The ramp percent right now at `coords`, from the local date and time.
fn auto_percent_now(coords: location::Coords) -> u8 {
    let now = chrono::Local::now();
    let sun = solar::sun_times(
        coords.0,
        coords.1,
        now.ordinal(),
        now.offset().local_minus_utc() / 60,
    );
    let minutes = f64::from(now.hour() * 60 + now.minute()) + f64::from(now.second()) / 60.0;
    solar::ramp_percent(sun, minutes, solar::RAMP_WINDOW_MINUTES)
}

/// What to ask the backend for to reach ramp `percent`: `Some((true, K))` to
/// set a temperature, `Some((false, _))` to go neutral, `None` when already
/// neutral.
fn auto_action(percent: u8, target: u32, enabled: bool) -> Option<(bool, u32)> {
    if percent == 0 {
        enabled.then_some((false, gamma::NEUTRAL_TEMPERATURE))
    } else {
        Some((
            true,
            solar::kelvin_for_percent(percent, target, gamma::NEUTRAL_TEMPERATURE),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_action_ramps_and_goes_neutral() {
        assert_eq!(auto_action(0, 3500, false), None);
        assert_eq!(auto_action(0, 3500, true), Some((false, 6500)));
        assert_eq!(auto_action(100, 3500, false), Some((true, 3500)));
        assert_eq!(auto_action(50, 3500, true), Some((true, 5000)));
    }

    #[test]
    fn new_target_is_used_at_once() {
        // A slider move forgets the last step, so the same percent is applied
        // again, now with the new target.
        let (last, apply) = solar::auto_step(50, None);
        assert_eq!(auto_action(apply.unwrap(), 3500, true), Some((true, 5000)));
        assert_eq!(solar::auto_step(50, last).1, None);
        let (_, apply) = solar::auto_step(50, None);
        assert_eq!(auto_action(apply.unwrap(), 2500, true), Some((true, 4500)));
    }

    #[test]
    fn hints_cover_the_range() {
        assert_eq!(temperature_hint(1900), "Very warm");
        assert_eq!(temperature_hint(3500), "Warm");
        assert_eq!(temperature_hint(4500), "Neutral warm");
        assert_eq!(temperature_hint(6500), "Cool");
    }

    #[test]
    fn settings_default_temperature() {
        assert_eq!(Settings::default().temperature, gamma::DEFAULT_TEMPERATURE);
    }

    #[test]
    fn settings_load_defaults_then_roundtrips_enabled() {
        // A private temp folder, removed on drop: the user's settings are never touched.
        let dir = tempfile::tempdir().expect("temp dir");
        let config = Config::with_custom_path(APP_ID, CONFIG_VERSION, dir.path().to_path_buf())
            .expect("config in temp dir");
        assert_eq!(Settings::load(&config), Settings::default());
        Settings::save_enabled(&config, true).expect("save enabled");
        assert!(Settings::load(&config).enabled);
    }

    fn temp_config() -> (tempfile::TempDir, Config) {
        // A private temp folder, removed on drop: the user's settings are never touched.
        let dir = tempfile::tempdir().expect("temp dir");
        let config = Config::with_custom_path(APP_ID, CONFIG_VERSION, dir.path().to_path_buf())
            .expect("config in temp dir");
        (dir, config)
    }

    #[test]
    fn from_timezone_migrates_from_location_custom() {
        // Nothing saved: on.
        assert!(from_timezone_setting(None, None));
        // Older versions: typed coordinates in use means the toggle is off.
        assert!(!from_timezone_setting(None, Some(true)));
        assert!(from_timezone_setting(None, Some(false)));
        // Once saved, the new key wins over the old one.
        assert!(from_timezone_setting(Some(true), Some(true)));
        assert!(!from_timezone_setting(Some(false), Some(false)));
    }

    #[test]
    fn old_custom_location_loads_with_the_toggle_off() {
        let (_dir, config) = temp_config();
        config.set(CONFIG_KEY_LOCATION_CUSTOM, true).unwrap();
        config.set(CONFIG_KEY_LOCATION_LAT, 48.85_f64).unwrap();
        config.set(CONFIG_KEY_LOCATION_LON, 2.35_f64).unwrap();
        let s = Settings::load(&config);
        assert_eq!(
            s.location,
            location::Setting {
                from_timezone: false,
                custom: Some((48.85, 2.35)),
            }
        );
        // Saving the toggle on keeps the coordinates for later.
        Settings::save_location(
            &config,
            location::Setting {
                from_timezone: true,
                ..s.location
            },
        )
        .unwrap();
        let s = Settings::load(&config);
        assert!(s.location.from_timezone);
        assert_eq!(s.location.custom, Some((48.85, 2.35)));
    }

    #[test]
    fn old_timezone_choice_and_bad_coordinates_load_with_the_toggle_on() {
        let (_dir, config) = temp_config();
        config.set(CONFIG_KEY_LOCATION_CUSTOM, false).unwrap();
        config.set(CONFIG_KEY_LOCATION_LAT, 123.0_f64).unwrap();
        config.set(CONFIG_KEY_LOCATION_LON, 2.35_f64).unwrap();
        assert_eq!(
            Settings::load(&config).location,
            location::Setting::default()
        );
    }

    #[test]
    fn old_schedule_keys_are_ignored() {
        let (_dir, config) = temp_config();
        config.set("schedule_on", true).unwrap();
        config.set("schedule_start", 21_u16 * 60).unwrap();
        config.set("schedule_end", 7_u16 * 60).unwrap();
        config.set(CONFIG_KEY_AUTO_ON, true).unwrap();
        let s = Settings::load(&config);
        assert_eq!(
            s,
            Settings {
                auto_on: true,
                ..Settings::default()
            }
        );
    }

    #[test]
    fn location_notes() {
        use location::{Source, Timezone};
        let tz = Timezone {
            zone: "America/New_York".to_string(),
            coords: (40.7142, -74.0064),
        };
        assert_eq!(
            location_note(Source::Timezone(&tz)).as_deref(),
            Some("New York (America/New_York): 40.71, -74.01")
        );
        for unresolved in [None, Some((1.0, 2.0))] {
            let note = location_note(Source::TimezoneUnresolved(unresolved)).unwrap();
            assert!(note.contains("wasn't found"), "{note}");
        }
        assert!(location_note(Source::Custom(None)).is_some());
        assert_eq!(location_note(Source::Custom(Some((1.0, 2.0)))), None);
    }

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
}

fn config() -> Option<Config> {
    Config::new(APP_ID, CONFIG_VERSION).ok()
}

/// Persisted settings, stored as one cosmic_config key per field. Adding a
/// field with a default needs no version bump: a missing key reads as its
/// default. Bump `CONFIG_VERSION` only when an existing key changes meaning.
#[derive(Clone, Debug, PartialEq)]
struct Settings {
    /// The user's last choice: night light on.
    enabled: bool,
    /// Colour temperature in Kelvin.
    temperature: u32,
    /// Automatic sunrise/sunset ramp on.
    auto_on: bool,
    /// Where the location comes from, and the coordinates typed in.
    location: location::Setting,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: false,
            temperature: gamma::DEFAULT_TEMPERATURE,
            auto_on: false,
            location: location::Setting::default(),
        }
    }
}

/// Whether the location comes from the timezone, from the saved
/// `location_from_timezone` and, before that key existed, the older
/// `location_custom` (typed coordinates in use means the toggle is off).
/// Neither saved: on.
fn from_timezone_setting(saved: Option<bool>, old_custom: Option<bool>) -> bool {
    saved.or(old_custom.map(|custom| !custom)).unwrap_or(true)
}

impl Settings {
    fn load(config: &Config) -> Self {
        Self {
            enabled: config.get::<bool>(CONFIG_KEY_ENABLED).unwrap_or(false),
            temperature: gamma::clamp_temperature(
                config
                    .get::<u32>(CONFIG_KEY_TEMPERATURE)
                    .unwrap_or(gamma::DEFAULT_TEMPERATURE),
            ),
            auto_on: config.get::<bool>(CONFIG_KEY_AUTO_ON).unwrap_or(false),
            location: location::Setting {
                from_timezone: from_timezone_setting(
                    config.get::<bool>(CONFIG_KEY_LOCATION_FROM_TIMEZONE).ok(),
                    config.get::<bool>(CONFIG_KEY_LOCATION_CUSTOM).ok(),
                ),
                // Loaded even while the timezone is used, so switching the
                // toggle off brings back what was typed.
                custom: config
                    .get::<f64>(CONFIG_KEY_LOCATION_LAT)
                    .ok()
                    .zip(config.get::<f64>(CONFIG_KEY_LOCATION_LON).ok())
                    .filter(|&(lat, lon)| location::valid(lat, lon)),
            },
        }
    }

    fn save_auto(config: &Config, on: bool) -> Result<(), cosmic::cosmic_config::Error> {
        config.set(CONFIG_KEY_AUTO_ON, on)
    }

    fn save_location(
        config: &Config,
        setting: location::Setting,
    ) -> Result<(), cosmic::cosmic_config::Error> {
        config.set(CONFIG_KEY_LOCATION_FROM_TIMEZONE, setting.from_timezone)?;
        if let Some((lat, lon)) = setting.custom {
            config.set(CONFIG_KEY_LOCATION_LAT, lat)?;
            config.set(CONFIG_KEY_LOCATION_LON, lon)?;
        }
        Ok(())
    }

    fn save_temperature(config: &Config, kelvin: u32) -> Result<(), cosmic::cosmic_config::Error> {
        config.set(CONFIG_KEY_TEMPERATURE, kelvin)
    }

    fn save_enabled(config: &Config, enabled: bool) -> Result<(), cosmic::cosmic_config::Error> {
        config.set(CONFIG_KEY_ENABLED, enabled)
    }
}

struct Nightlight {
    core: Core,
    /// Wanted, in-flight and applied gamma targets (see `requests`).
    requests: Requests,
    worker: Worker,
    /// Writes settings off the UI thread.
    saver: Saver,
    /// The settings popup, while it is open.
    popup: Option<Id>,
    /// Chosen colour temperature in Kelvin.
    temperature: u32,
    auto: AutoState,
}

/// State of the sunrise/sunset mode.
struct AutoState {
    on: bool,
    /// The "From time zone" toggle and the coordinates typed in.
    location: location::Setting,
    /// The system timezone, read once at start-up.
    timezone: Option<location::Timezone>,
    /// Text in the latitude and longitude boxes.
    lat_input: String,
    lon_input: String,
    /// Ramp percent applied last (see `solar::auto_step`).
    last: Option<u8>,
}

impl AutoState {
    /// Where the location in use comes from.
    fn source(&self) -> location::Source<'_> {
        location::choose(&self.location, self.timezone.as_ref())
    }
}

impl Nightlight {
    /// The location in use, if any.
    fn location(&self) -> Option<location::Coords> {
        self.auto.source().coords()
    }

    fn save_auto(&self) {
        self.saver.save(Save::Auto(self.auto.on));
    }

    /// Ask for `target`; sends it now unless a request is in flight (then
    /// its reply sends the latest target).
    fn want(&mut self, target: Target) -> Task<Message> {
        let send = self.requests.want(target);
        self.worker.send(send)
    }

    /// Check the ramp against the clock and ask for its step if the whole
    /// percent changed.
    fn check_auto(&mut self) -> Task<Message> {
        let Some(coords) = self.location().filter(|_| self.auto.on) else {
            return Task::none();
        };
        let (last, apply) = solar::auto_step(auto_percent_now(coords), self.auto.last);
        self.auto.last = last;
        match apply.and_then(|p| auto_action(p, self.temperature, self.requests.is_on())) {
            Some((on, kelvin)) => self.want(on.then_some(kelvin)),
            None => Task::none(),
        }
    }

    fn save_location(&self) {
        self.saver.save(Save::Location(self.auto.location));
    }

    /// The text boxes changed: once both hold valid degrees they become the
    /// typed location. Half-typed or out-of-range text changes nothing.
    fn location_edited(&mut self) -> Task<Message> {
        let coords = location::parse_degrees(&self.auto.lat_input)
            .zip(location::parse_degrees(&self.auto.lon_input))
            .filter(|&(lat, lon)| location::valid(lat, lon));
        match coords {
            Some(c) if self.auto.location.custom != Some(c) => {
                self.auto.location.custom = Some(c);
                self.save_location();
                self.location_changed()
            }
            _ => Task::none(),
        }
    }

    /// The "From time zone" toggle changed.
    fn timezone_toggled(&mut self, on: bool) -> Task<Message> {
        if on == self.auto.location.from_timezone {
            return Task::none();
        }
        self.auto.location.from_timezone = on;
        self.save_location();
        self.location_changed()
    }

    /// The location in use may have changed: recompute at once, or switch
    /// auto off when there is none left.
    fn location_changed(&mut self) -> Task<Message> {
        if self.auto.on && self.location().is_none() {
            self.auto.on = false;
            self.save_auto();
        }
        self.auto.last = None;
        self.check_auto()
    }

    /// Kelvin for a manual switch-on: the current ramp step while auto is on.
    /// At 0% (daytime) the ramp itself is neutral, but a person pressing the
    /// button wants the warm tint, so the full target is used; the next ramp
    /// step then takes over (documented in the README).
    fn manual_kelvin(&self) -> u32 {
        match self.location().filter(|_| self.auto.on) {
            Some(coords) => match auto_percent_now(coords) {
                0 => self.temperature,
                p => solar::kelvin_for_percent(p, self.temperature, gamma::NEUTRAL_TEMPERATURE),
            },
            None => self.temperature,
        }
    }

    /// The slider moved: remember and save the new target, and retune at
    /// once if night light is on (coalesced while a request is in flight).
    fn temperature_changed(&mut self, kelvin: u32) -> Task<Message> {
        let kelvin = gamma::clamp_temperature(kelvin);
        if kelvin == self.temperature {
            return Task::none();
        }
        self.temperature = kelvin;
        self.saver.save(Save::Temperature(kelvin));
        if self.auto.on {
            // The ramp uses the new target at once: forget the last step so
            // the current one is applied again.
            self.auto.last = None;
            return self.check_auto();
        }
        if self.requests.is_on() {
            return self.want(Some(kelvin));
        }
        Task::none()
    }

    /// The gamma worker answered the request in flight: record it, save the
    /// state on success, and send what was asked for meanwhile.
    fn reply_received(&mut self, result: &Result<(), String>) -> Task<Message> {
        let ok = result.is_ok();
        if let Err(e) = result {
            eprintln!("nightlight: {e}");
        }
        let replied = self.requests.replied(ok);
        if let Some(sent) = replied.sent {
            // Only a success is saved: a failed startup restore deliberately
            // keeps the saved flag set, so the next start retries (e.g. the
            // compositor wasn't ready at login) instead of forgetting it.
            if ok {
                self.saver.save(Save::Enabled(sent.is_some()));
            }
        }
        if !ok {
            // Forget the step so the next tick applies it again.
            self.auto.last = None;
        }
        self.worker.send(replied.send)
    }
}

#[derive(Clone, Debug)]
enum Message {
    Toggle,
    /// Right click on the icon: open or close the settings popup.
    ToggleSettings,
    PopupClosed(Id),
    /// The gamma worker answered the request in flight.
    Applied(Result<(), String>),
    /// The temperature slider moved.
    Temperature(u32),
    /// Auto (sunrise/sunset) was switched on or off.
    AutoOn(bool),
    /// The "From time zone" toggle changed.
    FromTimezone(bool),
    LatInput(String),
    LonInput(String),
    /// Time to check the auto ramp.
    Tick,
}

/// The popup's line about the location in use, if it needs one.
fn location_note(source: location::Source<'_>) -> Option<String> {
    use location::Source;
    match source {
        Source::Timezone(tz) => {
            let (lat, lon) = tz.coords;
            Some(format!(
                "{} ({}): {lat:.2}, {lon:.2}",
                location::place_name(&tz.zone),
                tz.zone
            ))
        }
        Source::TimezoneUnresolved(Some(_)) => Some(
            "The system time zone's location wasn't found: using the \
             latitude and longitude below."
                .to_string(),
        ),
        Source::TimezoneUnresolved(None) => Some(
            "The system time zone's location wasn't found: enter latitude \
             and longitude to use Auto."
                .to_string(),
        ),
        Source::Custom(None) => Some("Enter latitude and longitude to use Auto.".to_string()),
        Source::Custom(Some(_)) => None,
    }
}

/// The settings popup's content.
fn popup_content(state: &Nightlight) -> Element<'_, cosmic::Action<Message>> {
    let kelvin = state.temperature;
    let mut content = list_column()
        .add(settings::item(
            "Night light",
            toggler(state.requests.is_on()).on_toggle(|_| Message::Toggle),
        ))
        .add(settings::item(
            format!("Colour temperature: {kelvin} K"),
            text(temperature_hint(kelvin)),
        ))
        .add(
            slider(
                gamma::MIN_TEMPERATURE..=gamma::MAX_TEMPERATURE,
                kelvin,
                Message::Temperature,
            )
            .step(100_u32),
        )
        .add(settings::item(
            "Auto (sunrise/sunset)",
            toggler(state.auto.on).on_toggle(Message::AutoOn),
        ));
    if state.auto.on {
        content = content.add(text(format!(
            "Ramps over {} min centred on sunrise and sunset, to the \
             temperature above.",
            solar::RAMP_WINDOW_MINUTES
        )));
    }
    content = content.add(settings::item(
        "From time zone",
        toggler(state.auto.location.from_timezone).on_toggle(Message::FromTimezone),
    ));
    let source = state.auto.source();
    if let Some(note) = location_note(source) {
        content = content.add(text(note));
    }
    if source.shows_entry() {
        content = content
            .add(settings::item(
                "Latitude",
                cosmic::widget::text_input("e.g. 48.85", &state.auto.lat_input)
                    .on_input(Message::LatInput),
            ))
            .add(settings::item(
                "Longitude",
                cosmic::widget::text_input("e.g. 2.35", &state.auto.lon_input)
                    .on_input(Message::LonInput),
            ));
    }
    Element::from(state.core.applet.popup_container(content)).map(cosmic::Action::App)
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
        let settings = config().map(|c| Settings::load(&c)).unwrap_or_default();
        let timezone = location::timezone_location();
        let coords = location::choose(&settings.location, timezone.as_ref()).coords();
        // Auto needs a location; without one the saved choice is restored.
        let auto_coords = coords.filter(|_| settings.auto_on);
        let (auto_last, auto_apply) = match auto_coords {
            Some(c) => solar::auto_step(auto_percent_now(c), None),
            None => (None, None),
        };
        let start = if auto_coords.is_some() {
            auto_apply.and_then(|p| auto_action(p, settings.temperature, false))
        } else {
            settings.enabled.then_some((true, settings.temperature))
        };
        let worker = Worker::spawn();
        let mut requests = Requests::default();
        let task = worker.send(start.and_then(|(on, kelvin)| requests.want(on.then_some(kelvin))));
        let custom = settings.location.custom;
        let fmt = |v: f64| format!("{v}");
        (
            Self {
                core,
                requests,
                saver: Saver::spawn(),
                auto: AutoState {
                    on: auto_coords.is_some(),
                    location: settings.location,
                    timezone,
                    lat_input: custom.map(|c| fmt(c.0)).unwrap_or_default(),
                    lon_input: custom.map(|c| fmt(c.1)).unwrap_or_default(),
                    last: auto_last,
                },
                worker,
                popup: None,
                temperature: settings.temperature,
            },
            task,
        )
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.auto.on {
            cosmic::iced::time::every(TICK).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn on_close_requested(&self, id: window::Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Toggle => {
                // Never ignored: shown at once, sent now or after the reply
                // to the request in flight.
                let target = (!self.requests.is_on()).then(|| self.manual_kelvin());
                return self.want(target);
            }
            Message::Temperature(kelvin) => return self.temperature_changed(kelvin),
            Message::AutoOn(on) => {
                // No location: leave auto off (the popup shows a note).
                let on = on && self.location().is_some();
                self.auto.on = on;
                self.auto.last = None;
                self.save_auto();
                return self.check_auto();
            }
            Message::FromTimezone(on) => return self.timezone_toggled(on),
            Message::LatInput(text) => {
                self.auto.lat_input = text;
                return self.location_edited();
            }
            Message::LonInput(text) => {
                self.auto.lon_input = text;
                return self.location_edited();
            }
            Message::Tick => return self.check_auto(),
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
                        Some(Box::new(popup_content)),
                    ),
                };
                return cosmic::task::message(cosmic::Action::Surface(action));
            }
            Message::PopupClosed(id) => {
                self.popup = popup_after_close(self.popup, id);
            }
            Message::Applied(result) => return self.reply_received(&result),
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let icon = if self.requests.is_on() {
            ICON_ON
        } else {
            ICON_OFF
        };
        // Left click toggles; right click opens the settings popup. The popup
        // button has no right-click hook, so a mouse area wraps it. The toggle
        // fires on press, as libcosmic's own applet buttons do: `on_press`
        // fires only on a release over a button still marked pressed.
        let button = self
            .core
            .applet
            .icon_button(icon)
            .on_press_down(Message::Toggle);
        mouse_area(button)
            .on_right_press(Message::ToggleSettings)
            .into()
    }
}
