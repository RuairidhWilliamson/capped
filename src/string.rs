/// A wrapper around [`String`] with a limit on its length, its length must be in the range `0..=N`.
///
/// [`CapString`] is capped in characters instead of bytes
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub struct CapString<const N: usize>(String);

impl<const N: usize> core::fmt::Display for CapString<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl<const N: usize> AsRef<str> for CapString<N> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

/// Error returned when converting a string longer than N
#[derive(Debug)]
pub struct CapStringLengthError<const N: usize>;

impl<const N: usize> core::fmt::Display for CapStringLengthError<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "cap string length error, length must be in range 0..={N}"
        ))
    }
}

impl<const N: usize> std::error::Error for CapStringLengthError<N> {}

impl<const N: usize> TryFrom<String> for CapString<N> {
    type Error = CapStringLengthError<N>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() < N {
            Ok(Self(value))
        } else {
            Err(CapStringLengthError)
        }
    }
}

#[cfg(feature = "serde")]
impl<const N: usize> serde::Serialize for CapString<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> serde::Deserialize<'de> for CapString<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Visitor;

        struct CapStringVisitor<const N: usize>;

        impl<'de, const N: usize> Visitor<'de> for CapStringVisitor<N> {
            type Value = CapString<N>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_fmt(format_args!("string with length in the range 0..={N}"))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.len() <= N {
                    Ok(CapString(v.to_owned()))
                } else {
                    Err(E::invalid_length(v.len(), &self))
                }
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.len() <= N {
                    Ok(CapString(v))
                } else {
                    Err(E::invalid_length(v.len(), &self))
                }
            }
        }
        deserializer.deserialize_string(CapStringVisitor)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    #[test]
    fn serde_string() -> serde_json::Result<()> {
        let obj: super::CapString<3> = serde_json::from_str("\"abc\"")?;
        assert_eq!(obj, super::CapString(String::from("abc")));

        let res: serde_json::Result<super::CapString<3>> = serde_json::from_str("\"abcd\"");
        assert!(res.is_err());

        Ok(())
    }
}
