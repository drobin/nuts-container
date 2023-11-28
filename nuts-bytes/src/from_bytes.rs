// MIT License
//
// Copyright (c) 2023 Robin Doer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

#[cfg(test)]
mod tests;

use std::{mem, string::FromUtf8Error};
use thiserror::Error;

use crate::take_bytes::{TakeBytes, TakeBytesError};

/// Error type of the [`FromBytes`] trait.
#[derive(Debug, Error, PartialEq)]
pub enum FromBytesError {
    /// Errors coming from [`TakeBytes`].
    #[error(transparent)]
    TakeBytes(#[from] TakeBytesError),

    /// Failed to deserialize into a `char`. The source `u32` cannot be
    /// converted into a `char`.
    #[error("the char is invalid, {0} is not a char")]
    InvalidChar(u32),

    /// Failed to deserialize into a string. The source byte data are not valid
    /// UTF-8.
    #[error("the string is invalid: {0}")]
    InvalidString(#[source] FromUtf8Error),

    /// Deserialized an invalid variant index.
    /// There is no enum variant at the given index.
    #[cfg(feature = "derive")]
    #[error("invalid enum, no variant at {0}")]
    InvalidVariantIndex(usize),
}

/// Trait that supports reading datatypes from a binary data stream.
///
/// Datatypes that implements this trait can be read from a binary data stream.
pub trait FromBytes
where
    Self: Sized,
{
    /// Reads data from the given `source`.
    ///
    /// Reads as much as necessary from `source`. The method deserializes the
    /// instance and returns it.
    ///
    /// # Errors
    ///
    /// If not enough data are available in `source`, the
    /// [`TakeBytes::take_bytes()`] call returns a [`TakeBytesError::Eof`]
    /// error, which should be simply forwarded.
    fn from_bytes<TB: TakeBytes>(source: &mut TB) -> Result<Self, FromBytesError>;
}

impl FromBytes for bool {
    fn from_bytes<TB: TakeBytes>(source: &mut TB) -> Result<Self, FromBytesError> {
        let val: u8 = FromBytes::from_bytes(source)?;

        Ok(val != 0)
    }
}

macro_rules! impl_from_bytes_for_primitive {
    ($type:ty) => {
        impl FromBytes for $type {
            fn from_bytes<TB: TakeBytes>(source: &mut TB) -> Result<Self, FromBytesError> {
                let mut buf = [0; mem::size_of::<$type>()];

                source.take_bytes(&mut buf)?;

                Ok(<$type>::from_be_bytes(buf))
            }
        }
    };
}

impl_from_bytes_for_primitive!(i8);
impl_from_bytes_for_primitive!(i16);
impl_from_bytes_for_primitive!(i32);
impl_from_bytes_for_primitive!(i64);
impl_from_bytes_for_primitive!(u8);
impl_from_bytes_for_primitive!(u16);
impl_from_bytes_for_primitive!(u32);
impl_from_bytes_for_primitive!(u64);
impl_from_bytes_for_primitive!(f32);
impl_from_bytes_for_primitive!(f64);

impl FromBytes for usize {
    fn from_bytes<TB: TakeBytes>(source: &mut TB) -> Result<Self, FromBytesError> {
        let mut buf = [0; mem::size_of::<u64>()];

        source.take_bytes(&mut buf)?;

        Ok(u64::from_be_bytes(buf) as usize)
    }
}

impl FromBytes for char {
    fn from_bytes<TB: TakeBytes>(source: &mut TB) -> Result<Self, FromBytesError> {
        let n: u32 = FromBytes::from_bytes(source)?;

        char::from_u32(n).ok_or_else(|| FromBytesError::InvalidChar(n))
    }
}

impl<FB: Copy + Default + FromBytes, const COUNT: usize> FromBytes for [FB; COUNT] {
    fn from_bytes<TB: TakeBytes>(source: &mut TB) -> Result<Self, FromBytesError> {
        let mut target = [Default::default(); COUNT];

        for i in 0..COUNT {
            target[i] = FromBytes::from_bytes(source)?;
        }

        Ok(target)
    }
}

impl<FB: FromBytes> FromBytes for Vec<FB> {
    fn from_bytes<TB: TakeBytes>(source: &mut TB) -> Result<Self, FromBytesError> {
        let len = usize::from_bytes(source)?;
        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            vec.push(FromBytes::from_bytes(source)?);
        }

        Ok(vec)
    }
}

impl FromBytes for String {
    fn from_bytes<TB: TakeBytes>(source: &mut TB) -> Result<Self, FromBytesError> {
        let len = usize::from_bytes(source)?;

        let mut vec = vec![0; len];
        source.take_bytes(&mut vec)?;

        String::from_utf8(vec).map_err(|err| FromBytesError::InvalidString(err))
    }
}

impl<T: FromBytes> FromBytes for Option<T> {
    fn from_bytes<TB: TakeBytes>(source: &mut TB) -> Result<Self, FromBytesError> {
        let n: u8 = FromBytes::from_bytes(source)?;

        if n == 0 {
            Ok(None)
        } else {
            Ok(Some(FromBytes::from_bytes(source)?))
        }
    }
}

impl FromBytes for () {
    fn from_bytes<TB: TakeBytes>(_source: &mut TB) -> Result<Self, FromBytesError> {
        Ok(())
    }
}
