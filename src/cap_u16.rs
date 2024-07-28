use core::marker::PhantomData;

use crate::{error::CapError, num::CapNum};

/// A [`u16`] capped between 0..`N`
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CapU16<const N: u16>(u16);

impl<const N: u16> CapU16<N> {
    /// Create a new [`CapU16`] from a [`u16`] by modulo `N`.
    #[must_use]
    pub const fn new_wrap(value: u16) -> Self {
        Self(value % N)
    }

    /// Add `rhs` to [`CapU8`] using modulo `N`.
    #[must_use]
    pub const fn wrapping_add(self, rhs: u16) -> Self {
        Self((self.0 + rhs % N) % N)
    }

    /// Get the inner value
    #[must_use]
    pub const fn into_inner(self) -> u16 {
        self.0
    }
}

impl<const N: u16> CapNum for CapU16<N> {
    type Inner = u16;

    fn range() -> core::ops::Range<Self::Inner> {
        0..N
    }
}

impl<const N: u16> TryFrom<u16> for CapU16<N> {
    type Error = CapError<Self>;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value < N {
            Ok(Self(value))
        } else {
            Err(CapError(PhantomData))
        }
    }
}

impl<const N: u16> core::ops::Deref for CapU16<N> {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "serde")]
impl<const N: u16> serde::Serialize for CapU16<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: u16> serde::Deserialize<'de> for CapU16<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Visitor;

        struct CapU16Visitor<const N: u16>;

        impl<'de, const N: u16> Visitor<'de> for CapU16Visitor<N> {
            type Value = CapU16<N>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_fmt(format_args!("u16 in the range 0..{N}"))
            }

            fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v < N {
                    Ok(CapU16(v))
                } else {
                    Err(E::custom(format!("number {v} is greater than {N}")))
                }
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v.try_into() {
                    Ok(v) => self.visit_u16(v),
                    Err(_) => Err(E::custom(format!("number {v} is greater than {N}"))),
                }
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v.try_into() {
                    Ok(v) => self.visit_u16(v),
                    Err(_) => Err(E::custom(format!("number {v} is greater than {N}"))),
                }
            }
        }
        deserializer.deserialize_u16(CapU16Visitor)
    }
}

#[cfg(test)]
mod tests {
    use core::marker::PhantomData;

    #[test]
    fn from_u16() {
        assert_eq!(super::CapU16::<5>::try_from(4), Ok(super::CapU16(4)));
        assert_eq!(
            super::CapU16::<5>::try_from(5),
            Err(super::CapError(PhantomData))
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_u16() -> serde_json::Result<()> {
        let obj: Vec<super::CapU16<10>> = serde_json::from_str("[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]")?;
        assert_eq!(obj, (0..10).map(super::CapU16).collect::<Vec<_>>());

        let res: serde_json::Result<super::CapU16<2000>> = serde_json::from_str("5000");
        assert!(res.is_err());

        Ok(())
    }
}
