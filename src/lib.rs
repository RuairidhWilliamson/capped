//! Capped is a series of wrappers around types in the core and std library that have a maximum length.
//!
//! This is useful for dealing with deserializing and serde support can be enabled with the `serde` feature flag.

mod cap_u16;
mod cap_u32;
mod cap_u64;
mod cap_u8;
mod cap_usize;
mod num;

mod string;
mod vec;

mod error;

pub use cap_u16::CapU16;
pub use cap_u32::CapU32;
pub use cap_u64::CapU64;
pub use cap_u8::CapU8;
pub use cap_usize::CapUsize;
pub use error::CapError;
pub use string::{CapString, CapStringLengthError};
pub use vec::{CapVec, CapVecLengthError};
