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

use crate::{location, Settings};

/// How long the settings must stay unchanged before they are written.
const QUIET: Duration = Duration::from_millis(250);

/// One setting to save.
#[derive(Clone, Debug, PartialEq)]
pub enum Save {
    Enabled(bool),
    Temperature(u32),
    Auto(bool),
    /// The "From time zone" toggle and the typed coordinates.
    Location(location::Setting),
}

impl Save {
    /// The setting's name for messages; never its value.
    fn kind(&self) -> &'static str {
        match self {
            Save::Enabled(_) => "state",
            Save::Temperature(_) => "temperature",
            Save::Auto(_) => "auto setting",
            Save::Location(_) => "location",
        }
    }
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
        Save::Auto(on) => Settings::save_auto(config, on),
        Save::Location(custom) => Settings::save_location(config, custom),
    }
}

/// Handle to the saving thread. Dropping it writes what is waiting and
/// ends the thread.
pub struct Saver {
    /// `None` only while dropping.
    tx: Option<mpsc::Sender<Save>>,
    thread: Option<thread::JoinHandle<()>>,
}

impl Saver {
    /// Save to the applet's settings.
    pub fn spawn() -> Self {
        Self::spawn_with(|| Config::new(crate::APP_ID, crate::CONFIG_VERSION))
    }

    /// Save to the config `open` returns; it runs on the thread, since
    /// opening creates the config folder.
    fn spawn_with<F>(open: F) -> Self
    where
        F: FnOnce() -> Result<Config, cosmic::cosmic_config::Error> + Send + 'static,
    {
        let (tx, rx) = mpsc::channel();
        let thread = thread::spawn(move || run(open(), &rx));
        Self {
            tx: Some(tx),
            thread: Some(thread),
        }
    }

    /// Queue a setting to save; never blocks.
    pub fn save(&self, save: Save) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(save);
        }
    }
}

impl Drop for Saver {
    /// Close the channel, so the thread writes what is waiting at once
    /// (without the quiet period), and wait for it: a change made just
    /// before the applet exits is not lost.
    fn drop(&mut self) {
        drop(self.tx.take());
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn run(config: Result<Config, cosmic::cosmic_config::Error>, rx: &mpsc::Receiver<Save>) {
    let config = match config {
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
                    // The kind only: a value (e.g. the location) is private
                    // and stderr goes to the journal.
                    eprintln!("nightlight: cannot save {}: {e}", save.kind());
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
    fn kind_names_the_setting_without_its_value() {
        let kind = Save::Location(location::Setting {
            from_timezone: false,
            custom: Some((48.8566, 2.3522)),
        })
        .kind();
        assert_eq!(kind, "location");
        assert!(!kind.contains("48"));
    }

    #[test]
    fn dropping_the_saver_writes_what_is_waiting() {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().to_path_buf();
        let open = move || Config::with_custom_path(crate::APP_ID, crate::CONFIG_VERSION, path);
        let saver = Saver::spawn_with(open.clone());
        saver.save(Save::Temperature(2700));
        saver.save(Save::Enabled(true));
        // Dropped at once, well inside the quiet period.
        drop(saver);
        let s = Settings::load(&open().expect("config in temp dir"));
        assert_eq!(s.temperature, 2700);
        assert!(s.enabled);
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
            Save::Auto(true),
            Save::Location(location::Setting {
                from_timezone: false,
                custom: Some((48.85, 2.35)),
            }),
        ];
        for save in &saves {
            write(&config, save).expect("write");
        }
        let s = Settings::load(&config);
        assert!(s.enabled && s.auto_on);
        assert_eq!(s.temperature, 2800);
        assert_eq!(
            s.location,
            location::Setting {
                from_timezone: false,
                custom: Some((48.85, 2.35)),
            }
        );
    }
}
