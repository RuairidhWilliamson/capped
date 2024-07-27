//! Capped is a series of wrappers around types in the core and std library that have a maximum length.
//!
//! This is useful for dealing with deserializing and serde support can be enabled with the `serde` feature flag.

mod num;
mod u16;
mod u32;
mod u64;
mod u8;

mod string;
mod vec;

mod error;
mod usize;

pub use error::CapError;
pub use string::{CapString, CapStringLengthError};
pub use u16::CapU16;
pub use u32::CapU32;
pub use u64::CapU64;
pub use u8::CapU8;
pub use usize::CapUsize;
pub use vec::{CapVec, CapVecLengthError};
