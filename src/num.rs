use core::marker::PhantomData;
use core::ops::Range;

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

            /// Takes the current value of [`Self`] and increments it by 1 wrapping if it exceeds the limit `N`.
            #[must_use]
            pub fn take_increment(&mut self) -> Self {
                let out = *self;
                *self = self.wrapping_add(1);
                out
            }

            /// Get the inner value
            #[must_use]
            pub const fn into_inner(self) -> $inner {
                self.0
            }
        }

        // TODO: Does this violate PartialEq's requirement for transitive relation?
        // e.g. a == b && b == c => a == c
        // Specifically if:
        // let a = Cap<3>(2);
        // let b = 2;
        // let c = Cap<4>(2);
        // Then a == c is a compile error because they have different `N`
        impl<const N: $inner> PartialEq<$inner> for $cap_name<N> {
            fn eq(&self, other: &$inner) -> bool {
                self.0 == *other
            }
        }

        impl<const N: $inner> PartialEq<$cap_name<N>> for $inner {
            fn eq(&self, other: &$cap_name<N>) -> bool {
                *self == other.0
            }
        }

        impl<const N: $inner> crate::num::CapNum for $cap_name<N> {
            type Inner = $inner;

            fn range() -> core::ops::Range<Self::Inner> {
                0..N
            }
        }

        impl<const N: $inner> TryFrom<$inner> for $cap_name<N> {
            type Error = super::CapError<Self>;

            fn try_from(value: $inner) -> Result<Self, Self::Error> {
                if value < N {
                    Ok(Self(value))
                } else {
                    Err(super::CapError(core::marker::PhantomData))
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
                    Err(crate::num::CapError(PhantomData))
                );
                let c = $cap_name::<10>::new_wrap(29);
                assert_eq!(c.into_inner(), 9);
                assert_eq!(*c, 9);
                assert_eq!(<$cap_name::<10> as crate::num::CapNum>::range(), 0..10);
                assert!($cap_name::<240>::try_from(250)
                    .unwrap_err()
                    .to_string()
                    .contains("not in range 0..240"));
            }

            #[test]
            fn take_increment() {
                let mut c = $cap_name::<3>::new_wrap(1);
                assert_eq!(c.take_increment(), $cap_name::<3>(1));
                assert_eq!(c, $cap_name::<3>(2));
                assert_eq!(c.take_increment(), $cap_name::<3>(2));
                assert_eq!(c.take_increment(), $cap_name::<3>(0));
            }

            #[test]
            fn inner_eq() {
                assert_eq!($cap_name::<5>::new_wrap(2), $cap_name::<5>::new_wrap(2));
                assert_eq!($cap_name::<5>::new_wrap(2), 2);
                assert_eq!(2, $cap_name::<5>::new_wrap(2));
            }

            #[test]
            fn wrapping_add() {
                assert_eq!(
                    $cap_name::<10>::try_from(4).unwrap().wrapping_add(15),
                    $cap_name::<10>(9)
                );
                assert_eq!(
                    $cap_name::<10>::new_wrap(9).wrapping_add(249),
                    $cap_name::<10>(8)
                );
                assert_eq!(
                    $cap_name::<240>(239).wrapping_add(255),
                    $cap_name::<240>(14)
                );
            }

            #[cfg(feature = "serde")]
            #[test]
            fn serde_inner() -> serde_json::Result<()> {
                assert_eq!(
                    serde_json::to_string(&$cap_name::<10>(3))?,
                    String::from("3")
                );
                let obj: Vec<$cap_name<10>> = serde_json::from_str("[6, 9]")?;
                assert_eq!(obj, vec![$cap_name(6), $cap_name(9)]);

                let res: serde_json::Result<Vec<$cap_name<10>>> = serde_json::from_str("[10]");
                assert!(res.is_err());

                assert_eq!(
                    serde_json::from_str::<$cap_name::<6>>(&serde_json::to_string(
                        &$cap_name::<5>(3)
                    )?)?,
                    $cap_name::<6>(3)
                );

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
