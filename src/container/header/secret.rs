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

use nuts_bytes::{Reader, Writer};
use openssl::error::ErrorStack;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::backend::Backend;
use crate::container::cipher::Cipher;
use crate::container::error::ContainerResult;
use crate::container::kdf::Kdf;
use crate::container::ossl;
use crate::container::password::PasswordStore;
use crate::container::svec::SecureVec;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(try_from = "[u32; 2]")]
struct Magics([u32; 2]);

impl Magics {
    fn generate() -> Result<Magics, ErrorStack> {
        ossl::rand_u32().map(|magic| Magics([magic, magic]))
    }
}

impl TryFrom<[u32; 2]> for Magics {
    type Error = String;

    fn try_from(value: [u32; 2]) -> Result<Self, String> {
        if value[0] == value[1] {
            Ok(Magics(value))
        } else {
            Err("secret-magic mismatch".to_string())
        }
    }
}

impl PartialEq<[u32; 2]> for Magics {
    fn eq(&self, other: &[u32; 2]) -> bool {
        self.0[0] == other[0] && self.0[1] == other[1]
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Secret(Vec<u8>);

impl Secret {
    #[cfg(test)]
    pub fn new(vec: Vec<u8>) -> Secret {
        Secret(vec)
    }

    pub fn decrypt<B: Backend>(
        self,
        store: &mut PasswordStore,
        cipher: Cipher,
        kdf: &Kdf,
        iv: &[u8],
    ) -> ContainerResult<PlainSecret<B>, B> {
        let key = if cipher.key_len() > 0 {
            let password = store.value()?;
            kdf.create_key(password)?
        } else {
            vec![].into()
        };

        let mut pbuf = Vec::new();

        cipher.decrypt(&self.0, &mut pbuf, &key, &iv)?;

        let plain_secret = Reader::new(&pbuf[..]).deserialize()?;

        Ok(plain_secret)
    }
}

impl<T: AsRef<[u8]>> PartialEq<T> for Secret {
    fn eq(&self, other: &T) -> bool {
        self.0 == other.as_ref()
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct PlainSecret<B: Backend> {
    magics: Magics,
    pub key: SecureVec,
    pub iv: SecureVec,
    pub userdata: SecureVec,
    pub settings: B::Settings,
}

impl<B: Backend> PlainSecret<B> {
    pub fn generate(
        key: SecureVec,
        iv: SecureVec,
        userdata: SecureVec,
        settings: B::Settings,
    ) -> Result<PlainSecret<B>, ErrorStack> {
        Ok(PlainSecret {
            magics: Magics::generate()?,
            key,
            iv,
            userdata,
            settings,
        })
    }

    pub fn encrypt(
        self,
        store: &mut PasswordStore,
        cipher: Cipher,
        kdf: &Kdf,
        iv: &[u8],
    ) -> ContainerResult<Secret, B> {
        let mut writer = Writer::new(vec![]);
        let pbuf: SecureVec = writer
            .serialize(&self)
            .map(|_| writer.into_target().into())?;

        let key = if cipher.key_len() > 0 {
            let password = store.value()?;
            kdf.create_key(password)?
        } else {
            vec![].into()
        };

        let mut cbuf = Vec::new();
        cipher.encrypt(&pbuf, &mut cbuf, &key, &iv)?;

        Ok(Secret(cbuf))
    }
}
