//! The default location: the coordinates of the system timezone, read from
//! the local tz database tables. No network, no IP lookup.

use std::fs;

const ZONE_TABS: [&str; 2] = [
    "/usr/share/zoneinfo/zone.tab",
    "/usr/share/zoneinfo/zone1970.tab",
];

/// A latitude and longitude in degrees (north and east positive).
pub type Coords = (f64, f64);

/// Whether `lat`/`lon` are valid degrees.
pub fn valid(lat: f64, lon: f64) -> bool {
    (-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lon)
}

/// Parse text typed by the user as degrees.
pub fn parse_degrees(text: &str) -> Option<f64> {
    text.trim().parse::<f64>().ok().filter(|v| v.is_finite())
}

/// Parse one ISO 6709 part, `+DDMM`, `+DDMMSS` (`deg_digits` degree digits).
fn parse_part(part: &str, deg_digits: usize) -> Option<f64> {
    let sign = match part.chars().next()? {
        '+' => 1.0,
        '-' => -1.0,
        _ => return None,
    };
    let digits = &part[1..];
    if !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let field = |from: usize, len: usize| -> Option<f64> {
        digits.get(from..from + len)?.parse::<f64>().ok()
    };
    let degrees = field(0, deg_digits)?;
    let minutes = field(deg_digits, 2)?;
    let seconds = match digits.len() - deg_digits {
        2 => 0.0,
        4 => field(deg_digits + 2, 2)?,
        _ => return None,
    };
    Some(sign * (degrees + minutes / 60.0 + seconds / 3600.0))
}

/// Parse the coordinates column of a zone table: `+3114+12128`, `-3352+15113`
/// or with seconds `+404251-0740023`.
pub fn parse_iso6709(text: &str) -> Option<Coords> {
    // The sign is one ASCII byte, so `split` is always a character boundary.
    let rest = text.strip_prefix(['+', '-'])?;
    let split = rest.find(['+', '-'])? + 1;
    let (lat, lon) = text.split_at(split);
    let coords = (parse_part(lat, 2)?, parse_part(lon, 3)?);
    valid(coords.0, coords.1).then_some(coords)
}

/// The coordinates of `zone` in the contents of a zone table (tab separated:
/// countries, coordinates, zone name, comments).
pub fn find_zone(table: &str, zone: &str) -> Option<Coords> {
    table
        .lines()
        .filter(|l| !l.starts_with('#'))
        .find_map(|line| {
            let mut cols = line.split('\t');
            let (_, coords, name) = (cols.next()?, cols.next()?, cols.next()?);
            (name == zone).then(|| parse_iso6709(coords)).flatten()
        })
}

/// The zone name in a `/etc/localtime` link target such as
/// `/usr/share/zoneinfo/Europe/Paris`.
pub fn zone_from_link(target: &str) -> Option<&str> {
    let (_, zone) = target.split_once("zoneinfo/")?;
    (!zone.is_empty()).then_some(zone)
}

/// The system timezone's name, e.g. `Europe/Paris`.
fn system_timezone() -> Option<String> {
    if let Ok(link) = fs::read_link("/etc/localtime") {
        if let Some(zone) = zone_from_link(&link.to_string_lossy()) {
            return Some(zone.to_string());
        }
    }
    if let Ok(name) = fs::read_to_string("/etc/timezone") {
        let name = name.trim();
        if !name.is_empty() {
            return Some(name.to_string());
        }
    }
    let tz = std::env::var("TZ").ok()?;
    let tz = tz.trim_start_matches(':');
    (!tz.is_empty()).then(|| tz.to_string())
}

/// The system timezone and its coordinates.
#[derive(Clone, Debug, PartialEq)]
pub struct Timezone {
    /// The zone's name, e.g. `Europe/Paris`.
    pub zone: String,
    pub coords: Coords,
}

/// The system timezone, if the zone and a table listing it are found.
pub fn timezone_location() -> Option<Timezone> {
    let zone = system_timezone()?;
    let coords = ZONE_TABS
        .iter()
        .filter_map(|path| fs::read_to_string(path).ok())
        .find_map(|table| find_zone(&table, &zone))?;
    Some(Timezone { zone, coords })
}

/// The place a zone is named after, for people: `America/New_York` gives
/// `New York`, `UTC` gives `UTC`.
pub fn place_name(zone: &str) -> String {
    zone.rsplit('/').next().unwrap_or(zone).replace('_', " ")
}

/// The user's location setting.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Setting {
    /// Use the system timezone's coordinates (the default).
    pub from_timezone: bool,
    /// Coordinates the user typed in; kept while `from_timezone` is on, so
    /// switching it off brings them back.
    pub custom: Option<Coords>,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            from_timezone: true,
            custom: None,
        }
    }
}

/// Where the location in use comes from.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Source<'a> {
    /// The system timezone (the latitude/longitude entry is hidden).
    Timezone(&'a Timezone),
    /// The coordinates typed in, if any yet.
    Custom(Option<Coords>),
    /// The timezone was asked for but couldn't be resolved: the typed
    /// coordinates, if any, are used instead.
    TimezoneUnresolved(Option<Coords>),
}

impl Source<'_> {
    /// The coordinates in use, if any.
    pub fn coords(self) -> Option<Coords> {
        match self {
            Source::Timezone(tz) => Some(tz.coords),
            Source::Custom(c) | Source::TimezoneUnresolved(c) => c,
        }
    }

    /// Whether the latitude/longitude entry is shown.
    pub fn shows_entry(self) -> bool {
        !matches!(self, Source::Timezone(_))
    }
}

/// Pick the location source from the setting and the timezone found.
pub fn choose<'a>(setting: &Setting, timezone: Option<&'a Timezone>) -> Source<'a> {
    match (setting.from_timezone, timezone) {
        (true, Some(tz)) => Source::Timezone(tz),
        (true, None) => Source::TimezoneUnresolved(setting.custom),
        (false, _) => Source::Custom(setting.custom),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: Coords, b: Coords) -> bool {
        (a.0 - b.0).abs() < 1e-3 && (a.1 - b.1).abs() < 1e-3
    }

    #[test]
    fn parses_degrees_and_minutes() {
        assert!(close(
            parse_iso6709("+3114+12128").unwrap(),
            (31.2333, 121.4667)
        ));
        assert!(close(
            parse_iso6709("-3352+15113").unwrap(),
            (-33.8667, 151.2167)
        ));
        assert!(close(
            parse_iso6709("+4903-12307").unwrap(),
            (49.05, -123.1167)
        ));
    }

    #[test]
    fn parses_seconds() {
        assert!(close(
            parse_iso6709("+404251-0740023").unwrap(),
            (40.7142, -74.0064)
        ));
    }

    #[test]
    fn rejects_garbage() {
        for bad in [
            "",
            "+",
            "+31",
            "3114+12128",
            "+3114",
            "+31x4+12128",
            "+9914+12128",
            "+3114+18128",
            "é+3114+12128",
        ] {
            assert_eq!(parse_iso6709(bad), None, "{bad}");
        }
    }

    #[test]
    fn finds_a_zone_in_a_table() {
        let table = "# comment\nFR\t+4852+00220\tEurope/Paris\nDE,DK\t+5230+01322\tEurope/Berlin\tmost of Germany\n";
        assert!(close(
            find_zone(table, "Europe/Paris").unwrap(),
            (48.8667, 2.3333)
        ));
        assert!(close(
            find_zone(table, "Europe/Berlin").unwrap(),
            (52.5, 13.3667)
        ));
        assert_eq!(find_zone(table, "Europe/Nowhere"), None);
    }

    #[test]
    fn zone_name_from_localtime_link() {
        assert_eq!(
            zone_from_link("/usr/share/zoneinfo/Europe/Paris"),
            Some("Europe/Paris")
        );
        assert_eq!(zone_from_link("../usr/share/zoneinfo/UTC"), Some("UTC"));
        assert_eq!(zone_from_link("/etc/other"), None);
    }

    fn paris() -> Timezone {
        Timezone {
            zone: "Europe/Paris".to_string(),
            coords: (48.8667, 2.3333),
        }
    }

    #[test]
    fn place_names() {
        assert_eq!(place_name("America/New_York"), "New York");
        assert_eq!(place_name("America/Argentina/Buenos_Aires"), "Buenos Aires");
        assert_eq!(place_name("UTC"), "UTC");
    }

    #[test]
    fn default_setting_uses_the_timezone() {
        let tz = paris();
        let source = choose(&Setting::default(), Some(&tz));
        assert_eq!(source, Source::Timezone(&tz));
        assert_eq!(source.coords(), Some(tz.coords));
        assert!(!source.shows_entry());
    }

    #[test]
    fn timezone_wins_over_typed_coordinates_while_on() {
        let tz = paris();
        let setting = Setting {
            from_timezone: true,
            custom: Some((10.0, 20.0)),
        };
        assert_eq!(choose(&setting, Some(&tz)).coords(), Some(tz.coords));
    }

    #[test]
    fn toggle_off_uses_the_typed_coordinates() {
        let tz = paris();
        let setting = Setting {
            from_timezone: false,
            custom: Some((10.0, 20.0)),
        };
        let source = choose(&setting, Some(&tz));
        assert_eq!(source, Source::Custom(Some((10.0, 20.0))));
        assert!(source.shows_entry());
        // Nothing typed yet: no location, and the entry is shown.
        let source = choose(
            &Setting {
                custom: None,
                ..setting
            },
            Some(&tz),
        );
        assert_eq!(source.coords(), None);
        assert!(source.shows_entry());
    }

    #[test]
    fn unresolved_timezone_falls_back_to_the_entry() {
        let source = choose(&Setting::default(), None);
        assert_eq!(source, Source::TimezoneUnresolved(None));
        assert!(source.shows_entry());
        let setting = Setting {
            from_timezone: true,
            custom: Some((10.0, 20.0)),
        };
        assert_eq!(choose(&setting, None).coords(), Some((10.0, 20.0)));
    }

    #[test]
    fn typed_degrees() {
        assert_eq!(parse_degrees(" 12.5 "), Some(12.5));
        assert_eq!(parse_degrees("abc"), None);
        assert_eq!(parse_degrees("NaN"), None);
        assert!(valid(90.0, -180.0));
        assert!(!valid(90.1, 0.0));
        assert!(!valid(0.0, 181.0));
    }
}
