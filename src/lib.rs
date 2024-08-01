//! Capped is a series of wrappers around types in the core and std library that have a maximum length.
//!
//! This is useful for dealing with deserializing and serde support can be enabled with the `serde` feature flag.

mod num;

mod string;
mod vec;

mod error;

pub use error::CapError;
pub use num::cap_u16::CapU16;
pub use num::cap_u32::CapU32;
pub use num::cap_u64::CapU64;
pub use num::cap_u8::CapU8;
pub use num::cap_usize::CapUsize;
pub use string::{CapString, CapStringLengthError};
pub use vec::{CapVec, CapVecLengthError};
