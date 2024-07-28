use core::marker::PhantomData;

use crate::{error::CapError, num::CapNum};

/// A [`usize`] capped in the range 0..`N`
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CapUsize<const N: usize>(usize);

impl<const N: usize> CapUsize<N> {
    /// Create a new [`CapUsize`] from a [`usize`] by modulo `N`.
    #[must_use]
    pub const fn new_wrap(value: usize) -> Self {
        Self(value % N)
    }

    /// Add `rhs` to [`CapUsize`] using modulo `N`.
    #[must_use]
    pub const fn wrapping_add(self, rhs: usize) -> Self {
        Self((self.0 + rhs % N) % N)
    }

    /// Get the inner value
    #[must_use]
    pub const fn into_inner(self) -> usize {
        self.0
    }
}

impl<const N: usize> CapNum for CapUsize<N> {
    type Inner = usize;

    fn range() -> core::ops::Range<Self::Inner> {
        0..N
    }
}

impl<const N: usize> TryFrom<usize> for CapUsize<N> {
    type Error = CapError<Self>;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < N {
            Ok(Self(value))
        } else {
            Err(CapError(PhantomData))
        }
    }
}

impl<const N: usize> core::ops::Deref for CapUsize<N> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "serde")]
impl<const N: usize> serde::Serialize for CapUsize<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.0 as u64)
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> serde::Deserialize<'de> for CapUsize<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Visitor;

        struct CapUsizeVisitor<const N: usize>;

        impl<const N: usize> CapUsizeVisitor<N> {
            fn visit_usize<E>(v: usize) -> Result<CapUsize<N>, E>
            where
                E: serde::de::Error,
            {
                if v < N {
                    Ok(CapUsize(v))
                } else {
                    Err(E::custom(format!("number {v} is greater than {N}")))
                }
            }
        }

        impl<'de, const N: usize> Visitor<'de> for CapUsizeVisitor<N> {
            type Value = CapUsize<N>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_fmt(format_args!("usize in the range 0..{N}"))
            }

            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Self::visit_usize(v.into())
            }

            fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Self::visit_usize(v.into())
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v.try_into() {
                    Ok(v) => Self::visit_usize(v),
                    Err(_) => Err(E::custom(format!("number {v} is greater than {N}"))),
                }
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v.try_into() {
                    Ok(v) => Self::visit_usize(v),
                    Err(_) => Err(E::custom(format!("number {v} is greater than {N}"))),
                }
            }
        }
        deserializer.deserialize_u64(CapUsizeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use core::marker::PhantomData;

    #[test]
    fn from_usize() {
        assert_eq!(super::CapUsize::<5>::try_from(4), Ok(super::CapUsize(4)));
        assert_eq!(
            super::CapUsize::<5>::try_from(5),
            Err(super::CapError(PhantomData))
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_usize() -> serde_json::Result<()> {
        let obj: Vec<super::CapUsize<10>> = serde_json::from_str("[6, 9]")?;
        assert_eq!(obj, vec![super::CapUsize(6), super::CapUsize(9)]);

        let res: serde_json::Result<Vec<super::CapUsize<10>>> = serde_json::from_str("[10]");
        assert!(res.is_err());

        Ok(())
    }
}
