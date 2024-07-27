use core::marker::PhantomData;

use crate::num::CapNum;

/// Error produced when a cap is exceeded for the type T
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapError<T>(pub PhantomData<T>);

impl<T, U> core::fmt::Display for CapError<T>
where
    T: CapNum<Inner = U>,
    U: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let rng = T::range();
        let start = rng.start;
        let end = rng.end;
        f.write_fmt(format_args!("value is not in range {start}..{end}"))
    }
}

impl<T> std::error::Error for CapError<T>
where
    Self: core::fmt::Display,
    T: core::fmt::Debug,
{
}
