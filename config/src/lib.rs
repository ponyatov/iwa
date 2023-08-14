#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

/// daemon's localhost ip
pub const ip: &str = "127.0.0.1";

/// daemon's localhost port
pub const port: u16 = 54321;

/// socket bind string
pub const bind: &str = const_format::formatcp!("{ip}:{port}");

/// URL for [auto]opening web interface
pub const url: &str = const_format::formatcp!("http://{bind}");
