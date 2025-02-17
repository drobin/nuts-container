// MIT License
//
// Copyright (c) 2023,2024 Robin Doer
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

use anyhow::Result;
use clap::{ArgAction, Args};
use log::debug;

use crate::cli::archive::add::{TimestampArgs, TSTAMP_HELP};
use crate::cli::archive::open_archive;

#[derive(Args, Debug)]
#[clap(after_help(TSTAMP_HELP))]
pub struct ArchiveAddDirectoryArgs {
    /// Name of the directory.
    name: String,

    #[clap(flatten)]
    timestamps: TimestampArgs,

    /// Starts the migration when the container/archive is opened
    #[clap(long, action = ArgAction::SetTrue)]
    pub migrate: bool,

    /// Specifies the name of the container
    #[clap(short, long, env = "NUTS_CONTAINER")]
    container: String,
}

impl ArchiveAddDirectoryArgs {
    pub fn run(&self) -> Result<()> {
        debug!("args: {:?}", self);

        let mut archive = open_archive(&self.container, self.migrate)?;
        let mut builder = archive.append_directory(&self.name);

        if let Some(created) = self.timestamps.created {
            builder.set_created(created);
        }

        if let Some(changed) = self.timestamps.changed {
            builder.set_changed(changed);
        }

        if let Some(modified) = self.timestamps.modified {
            builder.set_modified(modified);
        }

        builder.build().map_err(Into::into)
    }
}
