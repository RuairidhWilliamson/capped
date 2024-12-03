/// [`CapVec`] is a [`Vec`] with a limit on its length, its length must be in the range `0..=N`.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub struct CapVec<const N: usize, T>(Vec<T>);

impl<const N: usize, T> CapVec<N, T> {
    /// Gets the length of the vector
    ///
    /// Guaranteed to be in the range `0..=N`.
    ///
    /// See [`Vec::len`]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns [`true`] if the vector contains no elements
    ///
    /// See [`Vec::is_empty`]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Pushes an element on to the end of the [`CapVec`] if there is space returns [`None`], otherwise returns [`Some`] `element`.
    ///
    /// See [`Vec::push`]
    #[must_use]
    pub fn push(&mut self, element: T) -> Option<T> {
        debug_assert!(self.0.len() <= N);
        debug_assert!(self.0.capacity() <= N);
        let len = self.0.len() + 1;
        if len <= N {
            // OPTIMISE: we should speculatively reserve more capacity up to the limit
            self.0.reserve_exact(1);
            self.0.push(element);
            None
        } else {
            Some(element)
        }
    }

    /// Removes the last element from the [`CapVec`] and returns it.
    ///
    /// See [`Vec::pop`]
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    /// Clears the [`CapVec`] setting the length to `0`.
    ///
    /// See [`Vec::clear`]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Truncate the [`CapVec`] to `len`.
    ///
    /// See [`Vec::truncate`]
    pub fn truncate(&mut self, len: usize) {
        self.0.truncate(len);
    }

    /// Get the underlying [`Vec`]
    #[must_use]
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }
}

impl<const N: usize, T> AsRef<Vec<T>> for CapVec<N, T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.0
    }
}

/// Error returned when converting a vec longer than N
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapVecLengthError<const N: usize>(usize);

impl<const N: usize> core::fmt::Display for CapVecLengthError<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let len = self.0;
        f.write_fmt(format_args!(
            "cap vec length error, vec of length {len} is longer than {N}"
        ))
    }
}

impl<const N: usize> std::error::Error for CapVecLengthError<N> {}

impl<const N: usize, T> From<CapVec<N, T>> for Vec<T> {
    fn from(value: CapVec<N, T>) -> Self {
        value.0
    }
}

impl<const N: usize, T> TryFrom<Vec<T>> for CapVec<N, T> {
    type Error = CapVecLengthError<N>;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if value.len() <= N {
            Ok(Self(value))
        } else {
            Err(CapVecLengthError(value.len()))
        }
    }
}

#[cfg(feature = "serde")]
impl<const N: usize, T> serde::Serialize for CapVec<N, T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for e in &self.0 {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize, T: serde::Deserialize<'de>> serde::Deserialize<'de> for CapVec<N, T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use core::marker::PhantomData;

        use serde::{
            de::{Error, Visitor},
            Deserialize,
        };

        struct CapVecVisitor<const N: usize, T>(PhantomData<T>);

        impl<'de, const N: usize, T: Deserialize<'de>> Visitor<'de> for CapVecVisitor<N, T> {
            type Value = CapVec<N, T>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_fmt(format_args!("a sequence of at most {N} elements"))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values: Vec<T>;
                if let Some(size) = seq.size_hint() {
                    values = Vec::with_capacity(size.min(N).min(1024));
                } else {
                    values = Vec::with_capacity(N.min(1024));
                }
                while let Some(value) = seq.next_element()? {
                    if values.len() >= N {
                        return Err(A::Error::invalid_length(values.len(), &self));
                    }
                    values.push(value);
                }
                Ok(CapVec(values))
            }
        }
        deserializer.deserialize_seq(CapVecVisitor(PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use super::{CapVec, CapVecLengthError};

    #[test]
    fn from_vec() {
        assert_eq!(
            CapVec::<3, u32>::try_from(vec![1, 2, 3]),
            Ok(CapVec::<3, u32>(vec![1, 2, 3]))
        );
        assert_eq!(
            CapVec::<3, _>::try_from(vec![1, 2, 3, 4]),
            Err(CapVecLengthError::<3>(4))
        );
        assert_eq!(CapVec::<3, u32>(vec![0, 1, 2]).as_ref(), &[0, 1, 2]);
        assert_eq!(Vec::from(CapVec::<3, u32>(vec![0, 1])), vec![0, 1]);
        let err = CapVec::<3, usize>::try_from(vec![1, 2, 3, 4]).unwrap_err();
        let err_msg = err.to_string();
        assert!(err_msg.contains("vec of length 4"));
        assert!(err_msg.contains("longer than 3"));
    }

    #[test]
    fn manipulate_vec() {
        let mut v = CapVec::<3, u32>::default();
        assert!(v.is_empty());
        assert_eq!(v.push(1), None);
        assert_eq!(v.push(1), None);
        assert_eq!(v.push(1), None);
        assert_eq!(v.push(1), Some(1));
        assert_eq!(v.pop(), Some(1));
        v.truncate(1);
        assert_eq!(v.len(), 1);
        v.clear();
        assert!(v.is_empty());
        assert_eq!(v.push(1), None);
        assert_eq!(v.into_inner(), vec![1]);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_vec() -> serde_json::Result<()> {
        let obj: CapVec<3, u32> = serde_json::from_str("[6, 3, 4]")?;
        assert_eq!(obj, CapVec(vec![6, 3, 4]));

        let res: serde_json::Result<CapVec<3, u32>> = serde_json::from_str("[24, 25, 26, 27]");
        assert!(res.is_err());

        assert_eq!(
            serde_json::from_str::<CapVec<3, i32>>(&serde_json::to_string(&CapVec::<3, i32>(
                vec![1, 2]
            ))?)?
            .as_ref(),
            &[1, 2]
        );

        Ok(())
    }
}
