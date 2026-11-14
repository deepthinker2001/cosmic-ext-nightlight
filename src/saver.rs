//! Saves settings on a long-lived thread, so the UI never waits on the disk.
//!
//! Each `cosmic_config` write creates the config folder, writes a temporary
//! file, fsyncs it, renames it and fsyncs the folder. Done on the UI thread
//! for every slider step, that made the applet lag. Here the UI only sends a
//! message; the thread waits until no change has come for `QUIET`, keeps
//! only the latest value of each kind, and writes those.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use cosmic::cosmic_config::Config;

use crate::{location, Settings, TimeOfDay};

/// How long the settings must stay unchanged before they are written.
const QUIET: Duration = Duration::from_millis(250);

/// One setting to save.
#[derive(Clone, Debug, PartialEq)]
pub enum Save {
    Enabled(bool),
    Temperature(u32),
    Schedule {
        on: bool,
        start: TimeOfDay,
        end: TimeOfDay,
    },
    Auto(bool),
    Location(Option<location::Coords>),
}

/// Add `save` to the writes waiting, replacing an older value of the same
/// kind (only the latest is worth writing).
pub fn merge(pending: &mut Vec<Save>, save: Save) {
    let kind = std::mem::discriminant(&save);
    match pending
        .iter_mut()
        .find(|p| std::mem::discriminant(*p) == kind)
    {
        Some(slot) => *slot = save,
        None => pending.push(save),
    }
}

fn write(config: &Config, save: &Save) -> Result<(), cosmic::cosmic_config::Error> {
    match *save {
        Save::Enabled(on) => Settings::save_enabled(config, on),
        Save::Temperature(kelvin) => Settings::save_temperature(config, kelvin),
        Save::Schedule { on, start, end } => Settings::save_schedule(config, on, start, end),
        Save::Auto(on) => Settings::save_auto(config, on),
        Save::Location(custom) => Settings::save_location(config, custom),
    }
}

/// Handle to the saving thread. Dropping it writes what is waiting and
/// ends the thread.
pub struct Saver {
    tx: mpsc::Sender<Save>,
}

impl Saver {
    pub fn spawn() -> Self {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || run(&rx));
        Self { tx }
    }

    /// Queue a setting to save; never blocks.
    pub fn save(&self, save: Save) {
        let _ = self.tx.send(save);
    }
}

fn run(rx: &mpsc::Receiver<Save>) {
    let config = match Config::new(crate::APP_ID, crate::CONFIG_VERSION) {
        Ok(c) => Some(c),
        Err(e) => {
            eprintln!("nightlight: cannot open settings, changes won't be saved: {e}");
            None
        }
    };
    let mut pending = Vec::new();
    while let Ok(first) = rx.recv() {
        merge(&mut pending, first);
        let open = loop {
            match rx.recv_timeout(QUIET) {
                Ok(save) => merge(&mut pending, save),
                Err(mpsc::RecvTimeoutError::Timeout) => break true,
                Err(mpsc::RecvTimeoutError::Disconnected) => break false,
            }
        };
        for save in pending.drain(..) {
            if let Some(c) = &config {
                if let Err(e) = write(c, &save) {
                    eprintln!("nightlight: cannot save {save:?}: {e}");
                }
            }
        }
        if !open {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_keeps_the_latest_of_each_kind_in_first_seen_order() {
        let mut pending = Vec::new();
        for k in [3400, 3300, 3200] {
            merge(&mut pending, Save::Temperature(k));
        }
        merge(&mut pending, Save::Enabled(true));
        merge(&mut pending, Save::Temperature(3100));
        merge(&mut pending, Save::Enabled(false));
        assert_eq!(pending, vec![Save::Temperature(3100), Save::Enabled(false)]);
    }

    #[test]
    fn every_kind_writes_its_keys() {
        let dir = tempfile::tempdir().expect("temp dir");
        let config = Config::with_custom_path(
            crate::APP_ID,
            crate::CONFIG_VERSION,
            dir.path().to_path_buf(),
        )
        .expect("config in temp dir");
        let saves = [
            Save::Enabled(true),
            Save::Temperature(2800),
            Save::Schedule {
                on: true,
                start: TimeOfDay(20 * 60),
                end: TimeOfDay(6 * 60),
            },
            Save::Auto(true),
            Save::Location(Some((48.85, 2.35))),
        ];
        for save in &saves {
            write(&config, save).expect("write");
        }
        let s = Settings::load(&config);
        assert!(s.enabled && s.schedule_on && s.auto_on);
        assert_eq!(s.temperature, 2800);
        assert_eq!(s.schedule_start, TimeOfDay(20 * 60));
        assert_eq!(s.schedule_end, TimeOfDay(6 * 60));
        assert_eq!(s.custom_location, Some((48.85, 2.35)));
    }
}
