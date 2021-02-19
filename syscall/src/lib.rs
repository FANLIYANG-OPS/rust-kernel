extern crate core;

pub use self::number::*;
pub use self::number::*;

pub mod call;
pub mod error;
pub mod number;

#[cfg(not(any(target_os = "none", target_os = "smaug")))]
#[path = "arch/smaug.rs"]
mod arch;
