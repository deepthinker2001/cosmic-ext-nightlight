//! Gamma control backend over Wayland (`zwlr_gamma_control_manager_v1`).
//!
//! Binds the manager, creates one gamma control per `wl_output`, and applies
//! a colour-temperature ramp to each. The compositor resets gamma as soon as
//! a control object is destroyed (or the connection closes), so the controls
//! are kept alive for as long as night light is enabled and destroyed on
//! disable to restore normal colour.

use std::fmt;
use std::io::Write;
use std::os::fd::AsFd;

use wayland_client::globals::{registry_queue_init, GlobalListContents};
use wayland_client::protocol::wl_output::{self, WlOutput};
use wayland_client::protocol::wl_registry::{self, WlRegistry};
use wayland_client::{Connection, Dispatch, EventQueue, Proxy, QueueHandle};
use wayland_protocols_wlr::gamma_control::v1::client::zwlr_gamma_control_manager_v1::ZwlrGammaControlManagerV1;
use wayland_protocols_wlr::gamma_control::v1::client::zwlr_gamma_control_v1::{
    self, ZwlrGammaControlV1,
};

/// Default colour temperature in Kelvin.
pub const DEFAULT_TEMPERATURE: u32 = 3500;
/// Lowest (warmest) temperature offered to the user, in Kelvin.
pub const MIN_TEMPERATURE: u32 = 1900;
/// Highest (coolest) temperature offered to the user; the neutral point.
pub const MAX_TEMPERATURE: u32 = 6500;

/// Limit a Kelvin value to the range offered to the user.
pub fn clamp_temperature(kelvin: u32) -> u32 {
    kelvin.clamp(MIN_TEMPERATURE, MAX_TEMPERATURE)
}

/// Neutral temperature: produces an identity ramp.
pub const NEUTRAL_TEMPERATURE: u32 = 6500;

/// Largest gamma table size accepted from the compositor (real hardware
/// reports 256 to 4096 entries); bounds the ramp allocation.
pub const MAX_GAMMA_SIZE: u32 = 65536;

const MANAGER_INTERFACE: &str = "zwlr_gamma_control_manager_v1";

#[derive(Debug)]
pub enum GammaError {
    /// Could not connect to the Wayland display.
    Connect(String),
    /// The compositor does not advertise `zwlr_gamma_control_manager_v1`.
    Unsupported,
    /// Wayland protocol/IO error.
    Wayland(String),
}

impl fmt::Display for GammaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GammaError::Connect(e) => write!(f, "cannot connect to Wayland display: {e}"),
            GammaError::Unsupported => write!(
                f,
                "the compositor does not advertise {MANAGER_INTERFACE}; \
                 night light cannot change screen colour on this compositor"
            ),
            GammaError::Wayland(e) => write!(f, "Wayland error: {e}"),
        }
    }
}

impl std::error::Error for GammaError {}

// ---------------------------------------------------------------------------
// Ramp maths
// ---------------------------------------------------------------------------

/// Per-channel multipliers (0.0..=1.0) for a blackbody-ish colour temperature,
/// using Tanner Helland's approximation. Clamped to 1000..=10000 K.
pub fn temperature_to_rgb(kelvin: u32) -> [f64; 3] {
    let t = f64::from(kelvin.clamp(1000, 10000)) / 100.0;
    let r = if t <= 66.0 {
        255.0
    } else {
        329.698_727_446 * (t - 60.0).powf(-0.133_204_759_2)
    };
    let g = if t <= 66.0 {
        99.470_802_586_1 * t.ln() - 161.119_568_166_1
    } else {
        288.122_169_528_3 * (t - 60.0).powf(-0.075_514_849_2)
    };
    let b = if t >= 66.0 {
        255.0
    } else if t <= 19.0 {
        0.0
    } else {
        138.517_731_223_1 * (t - 10.0).ln() - 305.044_792_730_7
    };
    [r, g, b].map(|c| c.clamp(0.0, 255.0) / 255.0)
}

/// Build a gamma ramp of `size` entries per channel: red, then green, then
/// blue, as the protocol expects (`3 * size` u16 values).
pub fn build_ramp(size: usize, kelvin: u32) -> Vec<u16> {
    let mul = temperature_to_rgb(kelvin);
    let mut out = Vec::with_capacity(size * 3);
    for m in mul {
        for i in 0..size {
            let x = if size > 1 {
                i as f64 / (size - 1) as f64
            } else {
                1.0
            };
            out.push((x * m * f64::from(u16::MAX)).round() as u16);
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Wayland state
// ---------------------------------------------------------------------------

struct OutputEntry {
    /// Registry global name; also the control's user data.
    name: u32,
    output: WlOutput,
    control: Option<ZwlrGammaControlV1>,
    gamma_size: u32,
    /// The compositor reported `failed` for this output's control.
    failed: bool,
}

struct State {
    manager: ZwlrGammaControlManagerV1,
    outputs: Vec<OutputEntry>,
    /// `Some(kelvin)` while night light is on.
    target: Option<u32>,
}

impl State {
    fn ensure_control(&mut self, idx: usize, qh: &QueueHandle<State>) {
        if self.target.is_none() {
            return;
        }
        let e = &mut self.outputs[idx];
        if e.control.is_none() && !e.failed {
            e.control = Some(self.manager.get_gamma_control(&e.output, qh, e.name));
        }
    }

    fn apply(&self, idx: usize) {
        let (Some(kelvin), e) = (self.target, &self.outputs[idx]) else {
            return;
        };
        let Some(control) = &e.control else { return };
        if e.gamma_size == 0 {
            return;
        }
        let ramp = build_ramp(e.gamma_size as usize, kelvin);
        let bytes: Vec<u8> = ramp.iter().flat_map(|v| v.to_ne_bytes()).collect();
        match ramp_fd(&bytes) {
            Ok(fd) => control.set_gamma(fd.as_fd()),
            Err(err) => eprintln!("nightlight: cannot create gamma table fd: {err}"),
        }
    }

    fn index_of(&self, name: u32) -> Option<usize> {
        self.outputs.iter().position(|o| o.name == name)
    }
}

/// An anonymous file holding the ramp, rewound to the start.
fn ramp_fd(bytes: &[u8]) -> std::io::Result<std::fs::File> {
    let fd = rustix::fs::memfd_create("nightlight-gamma", rustix::fs::MemfdFlags::CLOEXEC)?;
    let mut file = std::fs::File::from(fd);
    file.write_all(bytes)?;
    rustix::fs::seek(&file, rustix::fs::SeekFrom::Start(0))?;
    Ok(file)
}

impl Dispatch<WlRegistry, GlobalListContents> for State {
    fn event(
        state: &mut Self,
        registry: &WlRegistry,
        event: wl_registry::Event,
        _: &GlobalListContents,
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            wl_registry::Event::Global {
                name,
                interface,
                version,
            } if interface == "wl_output" => {
                let output = registry.bind::<WlOutput, _, _>(name, version.min(4), qh, ());
                state.outputs.push(OutputEntry {
                    name,
                    output,
                    control: None,
                    gamma_size: 0,
                    failed: false,
                });
                let idx = state.outputs.len() - 1;
                state.ensure_control(idx, qh);
            }
            wl_registry::Event::GlobalRemove { name } => {
                if let Some(idx) = state.index_of(name) {
                    let e = state.outputs.remove(idx);
                    if let Some(c) = e.control {
                        c.destroy();
                    }
                    if e.output.version() >= 3 {
                        e.output.release();
                    }
                }
            }
            _ => {}
        }
    }
}

impl Dispatch<WlOutput, ()> for State {
    fn event(
        _: &mut Self,
        _: &WlOutput,
        _: wl_output::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ZwlrGammaControlManagerV1, ()> for State {
    fn event(
        _: &mut Self,
        _: &ZwlrGammaControlManagerV1,
        _: <ZwlrGammaControlManagerV1 as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ZwlrGammaControlV1, u32> for State {
    fn event(
        state: &mut Self,
        control: &ZwlrGammaControlV1,
        event: zwlr_gamma_control_v1::Event,
        name: &u32,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        // Only the control currently stored for the output may change its
        // state: a stale control's late event must not drop a newer one.
        let current = state
            .index_of(*name)
            .filter(|&i| state.outputs[i].control.as_ref() == Some(control));
        match event {
            zwlr_gamma_control_v1::Event::GammaSize { size } => {
                let Some(idx) = current else {
                    control.destroy();
                    return;
                };
                if size == 0 || size > MAX_GAMMA_SIZE {
                    // Untrusted value: refuse rather than allocate a huge ramp.
                    eprintln!("nightlight: ignoring bad gamma size {size} for output {name}");
                    control.destroy();
                    let e = &mut state.outputs[idx];
                    e.control = None;
                    e.failed = true;
                    return;
                }
                state.outputs[idx].gamma_size = size;
                state.apply(idx);
            }
            zwlr_gamma_control_v1::Event::Failed => {
                // The control is now inert (output lost, or another client
                // holds gamma control). Always destroy it; only clear the
                // output's entry if this is the control stored there.
                control.destroy();
                if let Some(idx) = current {
                    eprintln!("nightlight: gamma control failed for output {name}");
                    let e = &mut state.outputs[idx];
                    e.control = None;
                    e.failed = true;
                }
            }
            _ => {}
        }
    }
}

// ---------------------------------------------------------------------------
// Public backend
// ---------------------------------------------------------------------------

pub struct GammaBackend {
    conn: Connection,
    queue: EventQueue<State>,
    state: State,
}

impl GammaBackend {
    /// Connect and bind the gamma manager. Returns [`GammaError::Unsupported`]
    /// when the compositor (e.g. cosmic-comp) does not advertise
    /// `zwlr_gamma_control_manager_v1`.
    pub fn new() -> Result<Self, GammaError> {
        let conn = Connection::connect_to_env().map_err(|e| GammaError::Connect(e.to_string()))?;
        let (globals, mut queue) =
            registry_queue_init::<State>(&conn).map_err(|e| GammaError::Wayland(e.to_string()))?;
        let qh = queue.handle();

        let advertised = globals
            .contents()
            .with_list(|l| l.iter().any(|g| g.interface == MANAGER_INTERFACE));
        if !advertised {
            return Err(GammaError::Unsupported);
        }
        let manager: ZwlrGammaControlManagerV1 = globals
            .bind(&qh, 1..=1, ())
            .map_err(|_| GammaError::Unsupported)?;

        let mut state = State {
            manager,
            outputs: Vec::new(),
            target: None,
        };
        let registry = globals.registry();
        globals.contents().with_list(|list| {
            for g in list.iter().filter(|g| g.interface == "wl_output") {
                let output = registry.bind::<WlOutput, _, _>(g.name, g.version.min(4), &qh, ());
                state.outputs.push(OutputEntry {
                    name: g.name,
                    output,
                    control: None,
                    gamma_size: 0,
                    failed: false,
                });
            }
        });
        queue
            .roundtrip(&mut state)
            .map_err(|e| GammaError::Wayland(e.to_string()))?;
        Ok(Self { conn, queue, state })
    }

    /// Whether night light is currently applied.
    pub fn is_enabled(&self) -> bool {
        self.state.target.is_some()
    }

    /// Turn night light on (or change its temperature) on all outputs.
    pub fn enable(&mut self, kelvin: u32) -> Result<(), GammaError> {
        let qh = self.queue.handle();
        self.state.target = Some(kelvin);
        for e in &mut self.state.outputs {
            e.failed = false; // allow a retry on explicit enable
        }
        for idx in 0..self.state.outputs.len() {
            self.state.ensure_control(idx, &qh);
            self.state.apply(idx); // controls that already know their size
        }
        self.sync()
    }

    /// Restore normal colour by destroying every control.
    pub fn disable(&mut self) -> Result<(), GammaError> {
        self.release_controls();
        self.sync()
    }

    fn release_controls(&mut self) {
        self.state.target = None;
        for e in &mut self.state.outputs {
            if let Some(c) = e.control.take() {
                c.destroy();
            }
            e.gamma_size = 0;
        }
    }

    /// Blocking round trip: waits for the compositor to answer, so it must
    /// never run on a UI thread. Used by `enable`/`disable` so their results
    /// (gamma sizes, failures) are known when they return.
    fn sync(&mut self) -> Result<(), GammaError> {
        self.queue
            .roundtrip(&mut self.state)
            .map(|_| ())
            .map_err(|e| GammaError::Wayland(e.to_string()))
    }

    /// Process pending events (outputs added/removed, `gamma_size`, `failed`)
    /// without blocking: flushes requests, reads whatever is available and
    /// dispatches it. Call periodically or when the Wayland fd is readable.
    pub fn dispatch(&mut self) -> Result<(), GammaError> {
        let werr = |e: wayland_client::backend::WaylandError| GammaError::Wayland(e.to_string());
        self.queue
            .dispatch_pending(&mut self.state)
            .map_err(|e| GammaError::Wayland(e.to_string()))?;
        match self.conn.flush() {
            Ok(()) => {}
            Err(wayland_client::backend::WaylandError::Io(e))
                if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(werr(e)),
        }
        if let Some(guard) = self.queue.prepare_read() {
            match guard.read() {
                Ok(_) => {}
                Err(wayland_client::backend::WaylandError::Io(e))
                    if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(e) => return Err(werr(e)),
            }
        }
        self.queue
            .dispatch_pending(&mut self.state)
            .map(|_| ())
            .map_err(|e| GammaError::Wayland(e.to_string()))
    }

    /// Number of outputs that currently have a working gamma control.
    pub fn active_outputs(&self) -> usize {
        self.state
            .outputs
            .iter()
            .filter(|o| o.control.is_some())
            .count()
    }

    /// The underlying connection (for polling its fd).
    pub fn connection(&self) -> &Connection {
        &self.conn
    }
}

impl Drop for GammaBackend {
    fn drop(&mut self) {
        // No round trip: a hung compositor must not block exit. Destroy the
        // controls and flush; closing the connection resets gamma anyway.
        self.release_controls();
        let _ = self.conn.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neutral_is_identity() {
        let [r, g, b] = temperature_to_rgb(NEUTRAL_TEMPERATURE);
        assert!(r > 0.99 && g > 0.97 && b > 0.97, "{r} {g} {b}");
    }

    #[test]
    fn warm_reduces_blue_most() {
        let [r, g, b] = temperature_to_rgb(DEFAULT_TEMPERATURE);
        assert_eq!(r, 1.0);
        assert!(g < r && b < g, "{r} {g} {b}");
        assert!(b > 0.0);
    }

    #[test]
    fn lower_is_warmer() {
        assert!(temperature_to_rgb(2500)[2] < temperature_to_rgb(4500)[2]);
    }

    #[test]
    fn clamps_extremes() {
        assert_eq!(temperature_to_rgb(0), temperature_to_rgb(1000));
        assert_eq!(temperature_to_rgb(100_000), temperature_to_rgb(10000));
        for k in [1000, 1500, 3500, 6500, 10000] {
            assert!(temperature_to_rgb(k)
                .iter()
                .all(|c| (0.0..=1.0).contains(c)));
        }
    }

    #[test]
    fn ramp_layout_and_monotonic() {
        let size = 256;
        let ramp = build_ramp(size, DEFAULT_TEMPERATURE);
        assert_eq!(ramp.len(), size * 3);
        for ch in ramp.chunks(size) {
            assert_eq!(ch[0], 0);
            assert!(ch.windows(2).all(|w| w[0] <= w[1]));
        }
        assert_eq!(ramp[size - 1], u16::MAX); // red untouched
        assert!(ramp[3 * size - 1] < ramp[size - 1]); // blue reduced
    }

    #[test]
    fn clamp_temperature_limits_to_slider_range() {
        assert_eq!(clamp_temperature(0), MIN_TEMPERATURE);
        assert_eq!(clamp_temperature(3500), 3500);
        assert_eq!(clamp_temperature(20_000), MAX_TEMPERATURE);
    }

    #[test]
    fn range_ends_are_ordered_and_default_inside() {
        const { assert!(MIN_TEMPERATURE < DEFAULT_TEMPERATURE && DEFAULT_TEMPERATURE < MAX_TEMPERATURE) };
        let warm = temperature_to_rgb(MIN_TEMPERATURE);
        let cool = temperature_to_rgb(MAX_TEMPERATURE);
        assert!(warm[2] < cool[2] && warm[1] < cool[1]);
    }

    #[test]
    fn ramp_gets_warmer_as_kelvin_drops() {
        let size = 64;
        let blue_top = |k| build_ramp(size, k)[3 * size - 1];
        assert!(blue_top(1900) < blue_top(3500));
        assert!(blue_top(3500) < blue_top(5000));
        assert!(blue_top(5000) <= blue_top(6500));
    }

    #[test]
    fn ramp_degenerate_sizes() {
        assert!(build_ramp(0, 3500).is_empty());
        assert_eq!(build_ramp(1, 6500).len(), 3);
    }
}

#[cfg(test)]
mod live {
    /// Informational probe of the running compositor (it prints and asserts
    /// nothing, so it cannot fail on compositors lacking the protocol); run
    /// with `--ignored --nocapture`.
    #[test]
    #[ignore]
    fn probe_compositor() {
        match super::GammaBackend::new() {
            Ok(b) => println!("gamma control supported; {} outputs", b.state.outputs.len()),
            Err(e) => println!("probe result: {e}"),
        }
    }
}
