// MIT License
//
// Copyright (c) 2022,2023 Robin Doer
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

mod cipher;
mod digest;
mod error;
mod header;
mod info;
mod kdf;
mod options;
mod password;

use log::debug;
use std::borrow::Cow;
use std::{any, cmp};

use crate::backend::{Backend, BlockId, Create, Open, HEADER_MAX_SIZE};
use crate::container::cipher::CipherCtx;
use crate::container::header::Header;
use crate::container::password::PasswordStore;
use crate::svec::SecureVec;

pub use cipher::{Cipher, CipherError};
pub use digest::{Digest, DigestError};
pub use error::{ContainerResult, Error};
pub use header::HeaderError;
pub use info::Info;
pub use kdf::Kdf;
pub use options::{CreateOptions, CreateOptionsBuilder, OpenOptions, OpenOptionsBuilder};
pub use password::NoPasswordError;

macro_rules! map_err {
    ($result:expr) => {
        $result.map_err(|cause| Error::Backend(cause))
    };
}

/// The Container type.
///
/// A `Container` acts like an encrypted block device, where you can read and
/// write encrypted blocks of data.
///
/// To create a new container use the [`Container::create`] method. You can
/// open an existing container with the [`Container::open`] method. With the
/// [`Container::read`] and [`Container::write`] methods you can read data from
/// the container resp. write data into the container.
pub struct Container<B: Backend> {
    backend: B,
    header: Header<B>,
    ctx: CipherCtx,
}

impl<B: Backend> Container<B> {
    /// Creates a new container.
    ///
    /// The new container is initialized with the given `options`. In case of
    /// an invalid option, the container is not created.
    ///
    // If encryption is turned on, you will be asked for a password over the
    // [`password callback`]. The returned password is then used for
    // encryption of the secure part of the header.
    //
    /// The header with the (possibly encrypted) secret is created and stored
    /// in the header-block of the container. The header contains all
    /// information you need to open the container again.
    ///
    /// # Errors
    ///
    /// Errors are listed in the [`Error`] type.
    // If encryption is enabled but no password callback is assigned or the
    // assigned callback returns an error, an [`Error::NoPassword`] error is
    // returned.
    //
    // Further errors are listed in the [`Error`] type.
    pub fn create(mut options: CreateOptions<B>) -> ContainerResult<Container<B>, B> {
        let header = Header::create(&options)?;
        let settings = options.backend.settings();

        let callback = options.callback.map(|cb| cb.clone());
        let mut store = PasswordStore::new(callback);

        Self::write_header(&mut options.backend, &header, settings, &mut store)?;
        let backend = map_err!(options.backend.build())?;

        let ctx = CipherCtx::new(header.cipher, backend.block_size())?;

        debug!(
            "Container created, backend: {}, header: {:?}",
            any::type_name::<B>(),
            header
        );

        Ok(Container {
            backend,
            header,
            ctx,
        })
    }

    /// Opens an existing container.
    ///
    /// The `options` argument are options used to open the container. Use the
    /// [`OpenOptionsBuilder`] utility to create such an instance.
    ///
    /// # Errors
    ///
    /// Errors are listed in the [`Error`] type.
    // If encryption is enabled but no password callback is assigned or the
    // assigned callback returns an error, an [`Error::NoPassword`] error is
    // returned.
    ///
    // Further errors are listed in the [`Error`] type.
    pub fn open(mut options: OpenOptions<B>) -> ContainerResult<Container<B>, B> {
        let callback = options.callback.map(|cb| cb.clone());
        let mut store = PasswordStore::new(callback);

        let (header, settings) = Self::read_header(&mut options.backend, &mut store)?;
        let backend = map_err!(options.backend.build(settings))?;

        let ctx = CipherCtx::new(header.cipher, backend.block_size())?;

        debug!(
            "Container opened, backend: {}, header: {:?}",
            any::type_name::<B>(),
            header
        );

        Ok(Container {
            backend,
            header,
            ctx,
        })
    }

    /// Returns the backend of this container.
    pub fn backend(&self) -> &B {
        &self.backend
    }

    /// Returns the top-id of the container.
    ///
    /// The top-id can be useful for services runnning on top of the container
    /// to have a defined, first, top block. The id is stored in the header of
    /// the container and is generated during creation of the container.
    ///
    /// A [`None`] value is returned, if the generation of the top-id is
    /// disabled. See [`CreateOptionsBuilder::with_top_id()`] for details.
    pub fn top_id(&self) -> Option<&B::Id> {
        self.header.top_id.as_ref()
    }

    /// Returns information from the container.
    ///
    /// # Errors
    ///
    /// Errors are listed in the [`Error`] type.
    pub fn info(&self) -> ContainerResult<Info<B>, B> {
        let backend = map_err!(self.backend.info())?;

        Ok(Info {
            backend,
            cipher: self.header.cipher,
            kdf: self.header.kdf.clone(),
        })
    }

    /// Aquires a new block in the backend.
    ///
    /// Once aquired you should be able to [read](Container::read) and
    /// [write](Container::write) from/to it.
    ///
    /// By default an aquired block, which is not written yet, returns an
    /// all-zero buffer.
    ///
    /// Returns the [id](Backend::Id) of the block.
    ///
    /// # Errors
    ///
    /// Errors are listed in the [`Error`] type.
    pub fn aquire(&mut self) -> ContainerResult<B::Id, B> {
        map_err!(self.backend.aquire())
    }

    /// Releases a block again.
    ///
    /// A released block cannot be [read](Container::read) and
    /// [written](Container::write), the [id](Backend::Id) cannot be used
    /// afterwards.
    ///
    /// # Errors
    ///
    /// Errors are listed in the [`Error`] type.
    pub fn release(&mut self, id: B::Id) -> ContainerResult<(), B> {
        map_err!(self.backend.release(id))
    }

    /// Reads a block from the container.
    ///
    /// Reads the block with the given `id` and places the decrypted data in
    /// `buf`.
    ///
    /// You cannot read not more data than [block-size](Backend::block_size)
    /// bytes. If `buf` is larger, than not the whole buffer is filled. In the
    /// other direction, if `buf` is not large enough to store the whole block,
    /// `buf` is filled with the first `buf.len()` bytes.
    ///
    /// The methods returns the number of bytes actually read, which cannot be
    /// greater than the [block-size](Backend::block_size).
    ///
    /// # Errors
    ///
    /// Errors are listed in the [`Error`] type.
    pub fn read(&mut self, id: &B::Id, buf: &mut [u8]) -> ContainerResult<usize, B> {
        if id.is_null() {
            return Err(Error::NullId);
        }

        let mut ctext = vec![0; self.backend.block_size() as usize];
        let n = map_err!(self.backend.read(id, &mut ctext))?;

        let key = &self.header.key;
        let iv = &self.header.iv;
        let ptext = self.ctx.decrypt(key, iv, &ctext[..n])?;

        let n = cmp::min(ptext.len(), buf.len());
        buf[..n].copy_from_slice(&ptext[..n]);

        Ok(n)
    }

    /// Writes a block into the container.
    ///
    /// Encrypts the plain data from `buf` and writes the encrypted data into
    /// the block with the given `id`.
    ///
    /// Writes up to `buf.len()` bytes from the unencrypted `buf` buffer into
    /// the container.
    ///
    /// If `buf` is not large enough to fill the whole block, the destination
    /// block is automatically padded with all zeros.
    ///
    /// If `buf` holds more data than the block-size, then the first
    /// [block-size](Backend::block_size) bytes are copied into the block.
    ///
    /// The method returns the number of bytes actually written.
    ///
    /// # Errors
    ///
    /// Errors are listed in the [`Error`] type.
    pub fn write(&mut self, id: &B::Id, buf: &[u8]) -> ContainerResult<usize, B> {
        if id.is_null() {
            return Err(Error::NullId);
        }

        let block_size = self.backend.block_size() as usize;
        let key = &self.header.key;
        let iv = &self.header.iv;

        let mut ptext = Cow::from(buf);

        if ptext.len() < block_size {
            // pad with 0 if not a complete block
            ptext.to_mut().resize(block_size, 0);
        }

        let result = self.ctx.encrypt(key, iv, &ptext);

        match ptext {
            Cow::Owned(buf) => {
                let _: SecureVec = buf.into();
            }
            _ => {}
        };

        let ctext = result?;
        map_err!(self.backend.write(id, ctext))
    }

    fn read_header(
        backend: &mut B::OpenOptions,
        store: &mut PasswordStore,
    ) -> ContainerResult<(Header<B>, B::Settings), B> {
        let mut buf = [0; HEADER_MAX_SIZE];

        match backend.get_header_bytes(&mut buf) {
            Ok(_) => Ok(Header::read(&buf, store)?),
            Err(cause) => Err(Error::Backend(cause)),
        }
    }

    fn write_header(
        backend: &mut B::CreateOptions,
        header: &Header<B>,
        settings: B::Settings,
        store: &mut PasswordStore,
    ) -> ContainerResult<(), B> {
        let mut buf = [0; HEADER_MAX_SIZE];

        header.write(settings, &mut buf, store)?;
        map_err!(backend.put_header_bytes(&buf))
    }
}
