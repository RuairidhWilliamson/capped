use core::marker::PhantomData;

use crate::{error::CapError, num::CapNum};

/// A [`u32`] capped between 0..`N`
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CapU32<const N: u32>(u32);

impl<const N: u32> CapNum for CapU32<N> {
    type Inner = u32;

    fn range() -> core::ops::Range<Self::Inner> {
        0..N
    }
}

impl<const N: u32> TryFrom<u32> for CapU32<N> {
    type Error = CapError<Self>;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= N {
            Ok(Self(value))
        } else {
            Err(CapError(PhantomData))
        }
    }
}

impl<const N: u32> core::ops::Deref for CapU32<N> {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "serde")]
impl<const N: u32> serde::Serialize for CapU32<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: u32> serde::Deserialize<'de> for CapU32<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Visitor;

        struct CapU32Visitor<const N: u32>;

        impl<'de, const N: u32> Visitor<'de> for CapU32Visitor<N> {
            type Value = CapU32<N>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_fmt(format_args!("u32 in the range 0..{N}"))
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v <= N {
                    Ok(CapU32(v))
                } else {
                    Err(E::custom(format!("number {v} is greater than {N}")))
                }
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v.try_into() {
                    Ok(v) => self.visit_u32(v),
                    Err(_) => Err(E::custom(format!("number {v} is greater than {N}"))),
                }
            }
        }
        deserializer.deserialize_u32(CapU32Visitor)
    }
}

#[cfg(test)]
mod tests {
    use core::marker::PhantomData;

    #[test]
    fn from_u32() {
        assert_eq!(super::CapU32::<5>::try_from(5), Ok(super::CapU32(5)));
        assert_eq!(
            super::CapU32::<5>::try_from(6),
            Err(super::CapError(PhantomData))
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_u32() -> serde_json::Result<()> {
        let obj: Vec<super::CapU32<10>> = serde_json::from_str("[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]")?;
        assert_eq!(obj, (0..10).map(cap_u32::CapU32).collect::<Vec<_>>());

        let res: serde_json::Result<super::CapU32<2000>> = serde_json::from_str("5000");
        assert!(res.is_err());

        Ok(())
    }
}
