/// A wrapper around [`String`] with a limit on its length, its length must be in the range `0..=N`.
///
/// [`CapString`] is capped in bytes instead of characters.
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

impl<const N: usize> CapString<N> {
    /// Returns the inner [`String`]
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Extracts a string slice containing the entire String
    ///
    /// See [`String::as_str`]
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Removes all contents.
    ///
    /// See [`String::clear`]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Removes the last character from the string and returns it.
    ///
    /// See [`String::pop`]
    pub fn pop(&mut self) -> Option<char> {
        self.0.pop()
    }

    /// Appends the given [`char`] to the end of this [`CapString`] if it will fit within the cap.
    ///
    /// An error is returned if the [`char`] cannot be added.
    ///
    /// See [`String::push`]
    /// # Errors
    /// Will return `Err` if the new string length would be greater than the cap string limit `N`.
    pub fn push(&mut self, ch: char) -> Result<(), CapStringLengthError<N>> {
        debug_assert!(self.0.len() <= N);
        debug_assert!(self.0.capacity() <= N);
        let len = self.0.len() + ch.len_utf8();
        if len <= N {
            // OPTIMISE: we should speculatively reserve more capacity up to the limit
            self.0.reserve_exact(1);
            self.0.push(ch);
            Ok(())
        } else {
            Err(CapStringLengthError(len))
        }
    }

    /// Appends a given string slice onto the end of this [`CapString`] if it will fit within the cap.
    ///
    /// An error is returned if the string cannot be added.
    ///
    /// See [`String::push_str`]
    /// # Errors
    /// Will return `Err` if the new string length would be greater than the cap string limit `N`.
    pub fn push_str(&mut self, string: &str) -> Result<(), CapStringLengthError<N>> {
        let len = self.0.len() + string.len();
        if len <= N {
            self.0.push_str(string);
            Ok(())
        } else {
            Err(CapStringLengthError(len))
        }
    }

    /// Shortens [`CapString`] to the specified length.
    ///
    /// If `new_len` is greater than or equal to the string's current length, this has no effect.
    ///
    /// See [`String::truncate`]
    ///
    /// # Panics
    /// Panics if `new_len` does not lie on a [`char`] boundary.
    pub fn truncate(&mut self, new_len: usize) {
        self.0.truncate(new_len);
    }

    /// Get mutable access to the inner [`String`].
    ///
    /// # Safety
    /// Must not grow the [`String`] beyond the cap length `N`.
    #[must_use]
    #[allow(unsafe_code)]
    pub unsafe fn get_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

/// Error returned when converting a string longer than N
#[derive(Debug)]
pub struct CapStringLengthError<const N: usize>(pub usize);

impl<const N: usize> core::fmt::Display for CapStringLengthError<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let len = self.0;
        f.write_fmt(format_args!(
            "cap string length error, length {len} must be in range 0..={N}",
        ))
    }
}

impl<const N: usize> std::error::Error for CapStringLengthError<N> {}

impl<const N: usize> TryFrom<String> for CapString<N> {
    type Error = CapStringLengthError<N>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= N {
            Ok(Self(value))
        } else {
            Err(CapStringLengthError(value.len()))
        }
    }
}

impl<const N: usize> From<CapString<N>> for String {
    fn from(value: CapString<N>) -> Self {
        value.0
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
    #[test]
    fn manipulate_string_ok() {
        let s = String::from("abc");
        let mut cap_s = super::CapString::<5>::try_from(s).unwrap();

        cap_s.push('d').unwrap();
        assert_eq!(cap_s.as_str(), "abcd");

        cap_s.pop();
        assert_eq!(cap_s.as_ref(), "abc");

        cap_s.push_str("de").unwrap();
        assert_eq!(cap_s.to_string(), "abcde");

        cap_s.truncate(2);
        assert_eq!(cap_s.as_ref(), "ab");

        #[allow(unsafe_code)]
        // Safety: We will only push one more element, this won't go beyond the length of the cap string
        let s = unsafe { cap_s.get_mut() };
        s.push_str("ababab");

        cap_s.clear();
        assert_eq!(cap_s.into_inner(), "");
    }

    #[test]
    fn manipulate_string_err() {
        assert!(super::CapString::<3>::try_from(String::from("abcd")).is_err());
        // Emoji takes up more than 1 bytes so exceeds cap length
        assert!(super::CapString::<3>::try_from(String::from("ab😃")).is_err());

        let mut cap_s = super::CapString::<3>::try_from(String::from("hi")).unwrap();
        assert!(cap_s.push_str("abc").is_err());
        assert!(cap_s.push('h').is_ok());
        assert!(cap_s
            .push('h')
            .unwrap_err()
            .to_string()
            .contains("length 4 must be in range 0..=3"));
        assert!(cap_s.push('h').is_err());
        assert_eq!(String::from(cap_s), "hih");
    }

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
