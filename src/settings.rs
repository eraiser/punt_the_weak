
/*
SETTINGS
*/
pub static VSYNC: bool = true;

use std::time::Duration;
pub static TICKS_PER_SECOND: u64 = 25;
pub static MCS_PER_UPDATE: Duration = Duration::from_micros(1000000 / TICKS_PER_SECOND);
