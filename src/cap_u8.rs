use core::marker::PhantomData;

use crate::{error::CapError, num::CapNum};

/// A [`u8`] capped in the range 0..`N`
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CapU8<const N: u8>(u8);

impl<const N: u8> CapU8<N> {
    /// Create a new [`CapU8`] from a [`u8`] by modulo `N`.
    #[must_use]
    pub const fn new_wrap(value: u8) -> Self {
        Self(value % N)
    }

    /// Add `rhs` to [`CapU8`] using modulo `N`.
    #[must_use]
    pub const fn wrapping_add(self, rhs: u8) -> Self {
        Self((self.0 + rhs % N) % N)
    }

    /// Get the inner value
    #[must_use]
    pub const fn into_inner(self) -> u8 {
        self.0
    }
}

impl<const N: u8> CapNum for CapU8<N> {
    type Inner = u8;

    fn range() -> core::ops::Range<Self::Inner> {
        0..N
    }
}

impl<const N: u8> TryFrom<u8> for CapU8<N> {
    type Error = CapError<Self>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < N {
            Ok(Self(value))
        } else {
            Err(CapError(PhantomData))
        }
    }
}

impl<const N: u8> core::ops::Deref for CapU8<N> {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "serde")]
impl<const N: u8> serde::Serialize for CapU8<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: u8> serde::Deserialize<'de> for CapU8<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Visitor;

        struct CapU8Visitor<const N: u8>;

        impl<'de, const N: u8> Visitor<'de> for CapU8Visitor<N> {
            type Value = CapU8<N>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_fmt(format_args!("u8 in the range 0..{N}"))
            }

            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v < N {
                    Ok(CapU8(v))
                } else {
                    Err(E::custom(format!("number {v} is greater than {N}")))
                }
            }

            fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v.try_into() {
                    Ok(v) => self.visit_u8(v),
                    Err(_) => Err(E::custom(format!("number {v} is greater than {N}"))),
                }
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v.try_into() {
                    Ok(v) => self.visit_u8(v),
                    Err(_) => Err(E::custom(format!("number {v} is greater than {N}"))),
                }
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v.try_into() {
                    Ok(v) => self.visit_u8(v),
                    Err(_) => Err(E::custom(format!("number {v} is greater than {N}"))),
                }
            }
        }
        deserializer.deserialize_u8(CapU8Visitor)
    }
}

#[cfg(test)]
mod tests {
    use core::marker::PhantomData;

    #[test]
    fn from_u8() {
        assert_eq!(super::CapU8::<5>::try_from(4), Ok(super::CapU8(4)));
        assert_eq!(
            super::CapU8::<5>::try_from(5),
            Err(super::CapError(PhantomData))
        );
    }

    #[test]
    fn wrapping_u8() -> Result<(), crate::CapError<super::CapU8<10>>> {
        assert_eq!(
            super::CapU8::<10>::try_from(4)?.wrapping_add(15),
            super::CapU8::<10>(9)
        );
        assert_eq!(
            super::CapU8::<10>::try_from(9)?.wrapping_add(249),
            super::CapU8::<10>(8)
        );
        assert_eq!(
            super::CapU8::<240>(239).wrapping_add(255),
            super::CapU8::<240>(14)
        );
        Ok(())
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_u8() -> serde_json::Result<()> {
        let obj: Vec<super::CapU8<10>> = serde_json::from_str("[6]")?;
        assert_eq!(obj, vec![super::CapU8(6)]);

        let res: serde_json::Result<Vec<super::CapU8<10>>> = serde_json::from_str("[10]");
        assert!(res.is_err());

        Ok(())
    }
}
