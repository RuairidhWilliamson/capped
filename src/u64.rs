use core::marker::PhantomData;

use crate::{error::CapError, num::CapNum};

/// A [`u64`] capped between 0..`N`
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CapU64<const N: u64>(u64);

impl<const N: u64> CapNum for CapU64<N> {
    type Inner = u64;

    fn range() -> core::ops::Range<Self::Inner> {
        0..N
    }
}

impl<const N: u64> TryFrom<u64> for CapU64<N> {
    type Error = CapError<Self>;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value <= N {
            Ok(Self(value))
        } else {
            Err(CapError(PhantomData))
        }
    }
}

#[cfg(feature = "serde")]
impl<const N: u64> serde::Serialize for CapU64<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: u64> serde::Deserialize<'de> for CapU64<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Visitor;

        struct CapU64Visitor<const N: u64>;

        impl<'de, const N: u64> Visitor<'de> for CapU64Visitor<N> {
            type Value = CapU64<N>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_fmt(format_args!("u64 in the range 0..{N}"))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v <= N {
                    Ok(CapU64(v))
                } else {
                    Err(E::custom(format!("number {v} is greater than {N}")))
                }
            }
        }
        deserializer.deserialize_u64(CapU64Visitor)
    }
}

#[cfg(test)]
mod tests {
    use core::marker::PhantomData;

    #[test]
    fn from_u64() {
        assert_eq!(super::CapU64::<5>::try_from(5), Ok(super::CapU64(5)));
        assert_eq!(
            super::CapU64::<5>::try_from(6),
            Err(super::CapError(PhantomData))
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_u64() -> serde_json::Result<()> {
        let obj: Vec<super::CapU64<10>> = serde_json::from_str("[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]")?;
        assert_eq!(obj, (0..10).map(super::CapU64).collect::<Vec<_>>());

        let res: serde_json::Result<super::CapU64<2000>> = serde_json::from_str("5000");
        assert!(res.is_err());

        Ok(())
    }
}
