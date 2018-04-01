use std::io;

mod error;
pub mod packets;
pub mod data;

pub use self::error::*;

pub const VERSION: u32 = 47;