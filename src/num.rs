use core::ops::Range;

pub trait CapNum {
    type Inner;

    fn range() -> Range<Self::Inner>;
}

macro_rules! num {
    ($cap_name:ident, $inner:ident, $str_inner:literal) => {
        #[doc = concat!("A [`", $str_inner, "`] capped between 0..`N`")]
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $cap_name<const N: $inner>($inner);

        impl<const N: $inner> $cap_name<N> {
            #[doc = concat!("Create a new [`Self`] from a [`", $str_inner, "`] by modulo `N`.")]
            #[must_use]
            pub const fn new_wrap(value: $inner) -> Self {
                Self(value % N)
            }

            /// Add `rhs` to [`Self`] using modulo `N`.
            #[must_use]
            pub const fn wrapping_add(self, rhs: $inner) -> Self {
                Self((self.0 + rhs % N) % N)
            }

            /// Get the inner value
            #[must_use]
            pub const fn into_inner(self) -> $inner {
                self.0
            }
        }

        impl<const N: $inner> crate::num::CapNum for $cap_name<N> {
            type Inner = $inner;

            fn range() -> core::ops::Range<Self::Inner> {
                0..N
            }
        }

        impl<const N: $inner> TryFrom<$inner> for $cap_name<N> {
            type Error = crate::error::CapError<Self>;

            fn try_from(value: $inner) -> Result<Self, Self::Error> {
                if value < N {
                    Ok(Self(value))
                } else {
                    Err(crate::error::CapError(core::marker::PhantomData))
                }
            }
        }

        impl<const N: $inner> core::ops::Deref for $cap_name<N> {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[cfg(test)]
        mod tests {
            use core::marker::PhantomData;

            use super::$cap_name;

            #[test]
            fn from_inner() {
                assert_eq!($cap_name::<5>::try_from(4), Ok($cap_name(4)));
                assert_eq!(
                    $cap_name::<5>::try_from(5),
                    Err(crate::error::CapError(PhantomData))
                );
            }

            #[cfg(feature = "serde")]
            #[test]
            fn serde_inner() -> serde_json::Result<()> {
                let obj: Vec<$cap_name<10>> = serde_json::from_str("[6, 9]")?;
                assert_eq!(obj, vec![$cap_name(6), $cap_name(9)]);

                let res: serde_json::Result<Vec<$cap_name<10>>> = serde_json::from_str("[10]");
                assert!(res.is_err());

                Ok(())
            }
        }
    };
}

pub mod cap_u8 {
    num!(CapU8, u8, "u8");

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
                    formatter.write_fmt(format_args!("U8 in the range 0..{N}"))
                }

                fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    if v < u64::from(N) {
                        Ok(CapU8(u8::try_from(v).expect("v less than N")))
                    } else {
                        Err(E::custom(format!("number {v} is greater than {N}")))
                    }
                }
            }
            deserializer.deserialize_u8(CapU8Visitor)
        }
    }
}
pub mod cap_u16 {
    num!(CapU16, u16, "u16");

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
                    formatter.write_fmt(format_args!("U16 in the range 0..{N}"))
                }

                fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    if v < u64::from(N) {
                        Ok(CapU16(u16::try_from(v).expect("v less than N")))
                    } else {
                        Err(E::custom(format!("number {v} is greater than {N}")))
                    }
                }
            }
            deserializer.deserialize_u16(CapU16Visitor)
        }
    }
}
pub mod cap_u32 {
    num!(CapU32, u32, "u32");

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
                    formatter.write_fmt(format_args!("U32 in the range 0..{N}"))
                }

                fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    if v < u64::from(N) {
                        Ok(CapU32(u32::try_from(v).expect("v less than N")))
                    } else {
                        Err(E::custom(format!("number {v} is greater than {N}")))
                    }
                }
            }
            deserializer.deserialize_u32(CapU32Visitor)
        }
    }
}
pub mod cap_u64 {
    num!(CapU64, u64, "u64");

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
                    if v < N {
                        Ok(CapU64(v))
                    } else {
                        Err(E::custom(format!("number {v} is greater than {N}")))
                    }
                }
            }
            deserializer.deserialize_u64(CapU64Visitor)
        }
    }
}
pub mod cap_usize {
    num!(CapUsize, usize, "usize");

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

            impl<'de, const N: usize> Visitor<'de> for CapUsizeVisitor<N> {
                type Value = CapUsize<N>;

                fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                    formatter.write_fmt(format_args!("usize in the range 0..{N}"))
                }

                fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    if v < N as u64 {
                        Ok(CapUsize(usize::try_from(v).expect("v less than N")))
                    } else {
                        Err(E::custom(format!("number {v} is greater than {N}")))
                    }
                }
            }
            deserializer.deserialize_u64(CapUsizeVisitor)
        }
    }
}
