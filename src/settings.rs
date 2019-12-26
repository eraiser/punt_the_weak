/*
SETTINGS
*/
use std::time::Duration;
use glutin::dpi::LogicalSize;

pub const VSYNC: bool = true;

pub const TICKS_PER_SECOND: f32 = 25.0;
pub const MCS_PER_UPDATE: Duration = Duration::from_micros(1000000 / TICKS_PER_SECOND as u64);

pub const WINDOW_LABEL: &str = "punt the weak";

pub const MAX_LIGHTS: usize = 4;

pub const START_WINDOWSIZE: LogicalSize = LogicalSize{ width: 960.0, height: 540.0 };
