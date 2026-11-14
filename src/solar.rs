//! Sunrise/sunset times and the smooth ramp around them. Pure functions: no
//! clock, no files; the caller passes the date, time and location in.

use std::f64::consts::PI;

/// Length of the ramp around each sunrise and sunset, in minutes. The window
/// is centred on the event: half of it before, half after.
pub const RAMP_WINDOW_MINUTES: f64 = 60.0;

/// The sun on one day, with times in minutes since local midnight.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Sun {
    Rises { sunrise: f64, sunset: f64 },
    /// The sun stays up all day.
    PolarDay,
    /// The sun stays down all day.
    PolarNight,
}

/// Sunrise and sunset (NOAA's general solar position formulas, accurate to
/// a few minutes) for `lat`/`lon` in degrees (north and east positive), on
/// day `ordinal` of the year (1 = 1 January), in a zone `utc_offset_minutes`
/// ahead of UTC.
pub fn sun_times(lat: f64, lon: f64, ordinal: u32, utc_offset_minutes: i32) -> Sun {
    let g = 2.0 * PI / 365.0 * (f64::from(ordinal) - 1.0);
    let eqtime = 229.18
        * (0.000075 + 0.001868 * g.cos()
            - 0.032077 * g.sin()
            - 0.014615 * (2.0 * g).cos()
            - 0.040849 * (2.0 * g).sin());
    let decl = 0.006918 - 0.399912 * g.cos() + 0.070257 * g.sin()
        - 0.006758 * (2.0 * g).cos()
        + 0.000907 * (2.0 * g).sin()
        - 0.002697 * (3.0 * g).cos()
        + 0.00148 * (3.0 * g).sin();
    let lat_r = lat.to_radians();
    // 90.833 degrees: the sun's centre at the horizon, with refraction.
    let cos_ha = 90.833_f64.to_radians().cos() / (lat_r.cos() * decl.cos())
        - lat_r.tan() * decl.tan();
    if cos_ha.is_nan() {
        return Sun::PolarNight;
    }
    if cos_ha > 1.0 {
        return Sun::PolarNight;
    }
    if cos_ha < -1.0 {
        return Sun::PolarDay;
    }
    let ha = cos_ha.acos().to_degrees();
    let offset = f64::from(utc_offset_minutes);
    Sun::Rises {
        sunrise: 720.0 - 4.0 * (lon + ha) - eqtime + offset,
        sunset: 720.0 - 4.0 * (lon - ha) - eqtime + offset,
    }
}

/// Half-cosine ease: 0 at `x = 0`, 1 at `x = 1`, flat at both ends.
pub fn ease(x: f64) -> f64 {
    (PI / 2.0 * x.clamp(0.0, 1.0)).sin().powi(2)
}

/// How far the night-light effect should be, as a whole percent: 0 is
/// neutral, 100 is the user's target. It reaches 100 in the evening ramp
/// around sunset, falls back to 0 in the morning ramp around sunrise.
/// `now` is minutes since local midnight.
pub fn ramp_percent(sun: Sun, now: f64, window: f64) -> u8 {
    let fraction = match sun {
        Sun::PolarDay => 0.0,
        Sun::PolarNight => 1.0,
        Sun::Rises { sunrise, sunset } => {
            let half = window / 2.0;
            let morning = 1.0 - ease((now - (sunrise - half)) / window);
            let evening = ease((now - (sunset - half)) / window);
            // Where the windows overlap (a very short day) the stronger wins.
            morning.max(evening)
        }
    };
    (fraction * 100.0).round() as u8
}

/// The Kelvin value for `percent` of the way from neutral to `target`.
pub fn kelvin_for_percent(percent: u8, target: u32, neutral: u32) -> u32 {
    let (neutral, target) = (i64::from(neutral), i64::from(target));
    let percent = i64::from(percent.min(100));
    (neutral + (target - neutral) * percent / 100) as u32
}

/// One check of the ramp. `last` is the percent applied last (`None` at
/// start-up, after a setting changed, after a failure, or while auto is
/// off). Returns the new `last` and, when the whole percent changed, the
/// percent to apply. Acting only on a changed step lets a manual toggle hold
/// until the next step.
pub fn auto_step(percent: u8, last: Option<u8>) -> (Option<u8>, Option<u8>) {
    if last == Some(percent) {
        (last, None)
    } else {
        (Some(percent), Some(percent))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rises(sun: Sun) -> (f64, f64) {
        match sun {
            Sun::Rises { sunrise, sunset } => (sunrise, sunset),
            other => panic!("expected sunrise and sunset, got {other:?}"),
        }
    }

    #[test]
    fn london_midsummer() {
        // 21 June 2024 (day 173): sunrise 03:43 UTC, sunset 20:21 UTC.
        let (rise, set) = rises(sun_times(51.5, -0.12, 173, 0));
        assert!((rise - (3.0 * 60.0 + 43.0)).abs() < 5.0, "rise {rise}");
        assert!((set - (20.0 * 60.0 + 21.0)).abs() < 5.0, "set {set}");
    }

    #[test]
    fn utc_offset_shifts_both_times() {
        let (r0, s0) = rises(sun_times(51.5, -0.12, 173, 0));
        let (r1, s1) = rises(sun_times(51.5, -0.12, 173, 60));
        assert!((r1 - r0 - 60.0).abs() < 1e-9);
        assert!((s1 - s0 - 60.0).abs() < 1e-9);
    }

    #[test]
    fn equinox_at_the_equator_is_about_twelve_hours() {
        let (rise, set) = rises(sun_times(0.0, 0.0, 80, 0));
        assert!((set - rise - 727.0).abs() < 8.0, "day {}", set - rise);
        assert!((rise - 360.0).abs() < 15.0);
    }

    #[test]
    fn polar_day_and_night() {
        assert_eq!(sun_times(80.0, 10.0, 173, 0), Sun::PolarDay);
        assert_eq!(sun_times(80.0, 10.0, 356, 0), Sun::PolarNight);
        assert_eq!(sun_times(-80.0, 10.0, 173, 0), Sun::PolarNight);
        assert_eq!(sun_times(90.0, 0.0, 173, 0), Sun::PolarDay);
    }

    #[test]
    fn ease_is_smooth_and_bounded() {
        assert_eq!(ease(-1.0), 0.0);
        assert_eq!(ease(0.0), 0.0);
        assert!((ease(0.5) - 0.5).abs() < 1e-12);
        assert!((ease(1.0) - 1.0).abs() < 1e-12);
        assert_eq!(ease(2.0), ease(1.0));
        let mut previous = 0.0;
        for i in 0..=100 {
            let v = ease(f64::from(i) / 100.0);
            assert!(v >= previous);
            previous = v;
        }
    }

    const SUN: Sun = Sun::Rises {
        sunrise: 400.0,
        sunset: 1200.0,
    };

    #[test]
    fn ramp_follows_the_day() {
        let w = RAMP_WINDOW_MINUTES;
        assert_eq!(ramp_percent(SUN, 0.0, w), 100); // night
        assert_eq!(ramp_percent(SUN, 369.0, w), 100); // before the morning window
        assert_eq!(ramp_percent(SUN, 400.0, w), 50); // sunrise: halfway down
        assert_eq!(ramp_percent(SUN, 431.0, w), 0);
        assert_eq!(ramp_percent(SUN, 800.0, w), 0); // day
        assert_eq!(ramp_percent(SUN, 1169.0, w), 0);
        assert_eq!(ramp_percent(SUN, 1200.0, w), 50); // sunset: halfway up
        assert_eq!(ramp_percent(SUN, 1231.0, w), 100);
        assert_eq!(ramp_percent(SUN, 1439.0, w), 100);
    }

    #[test]
    fn ramp_is_monotonic_through_each_event() {
        let w = RAMP_WINDOW_MINUTES;
        let mut previous = 100;
        for m in 370..=430 {
            let p = ramp_percent(SUN, f64::from(m), w);
            assert!(p <= previous);
            previous = p;
        }
        let mut previous = 0;
        for m in 1170..=1230 {
            let p = ramp_percent(SUN, f64::from(m), w);
            assert!(p >= previous);
            previous = p;
        }
    }

    #[test]
    fn polar_ramp_is_constant() {
        assert_eq!(ramp_percent(Sun::PolarDay, 700.0, RAMP_WINDOW_MINUTES), 0);
        assert_eq!(ramp_percent(Sun::PolarNight, 700.0, RAMP_WINDOW_MINUTES), 100);
    }

    #[test]
    fn kelvin_runs_from_neutral_to_target() {
        assert_eq!(kelvin_for_percent(0, 3500, 6500), 6500);
        assert_eq!(kelvin_for_percent(100, 3500, 6500), 3500);
        assert_eq!(kelvin_for_percent(50, 3500, 6500), 5000);
        assert_eq!(kelvin_for_percent(200, 3500, 6500), 3500);
    }

    #[test]
    fn step_applies_only_when_the_whole_percent_changes() {
        assert_eq!(auto_step(40, None), (Some(40), Some(40)));
        assert_eq!(auto_step(40, Some(40)), (Some(40), None));
        assert_eq!(auto_step(41, Some(40)), (Some(41), Some(41)));
        // A forgotten step (failure, new target) is applied again.
        assert_eq!(auto_step(40, None), (Some(40), Some(40)));
    }
}
