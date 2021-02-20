extern crate core;

pub use self::arch::*;
pub use self::call::*;
pub use self::data::*;
pub use self::error::*;
pub use self::flag::*;
pub use self::number::*;

#[cfg(not(any(target_os = "none", target_os = "smaug")))]
#[path = "arch/smaug.rs"]
mod arch;

pub mod call;
pub mod data;
pub mod error;
pub mod flag;
pub mod number;

