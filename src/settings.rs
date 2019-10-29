
/*
SETTINGS
*/
pub static VSYNC: bool = false;

use std::time::Duration;
pub static TICKS_PER_SECOND: f32 = 25.0;
pub static MCS_PER_UPDATE: Duration = Duration::from_micros(1000000 / TICKS_PER_SECOND as u64);

pub static WINDOW_LABEL: &str = "punt the weak";

pub const MAX_LIGHTS:usize = 4;