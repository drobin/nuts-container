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

use crate::entry::r#mut::tests::{lookup, setup_directory_builder};
use crate::entry::{Inner, FULL};
use crate::tests::setup_container_with_bsize;
use crate::Archive;

#[test]
fn ok() {
    let container = setup_container_with_bsize(FULL as u32);
    let mut archive = Archive::create(container, false).unwrap();

    let tuple = setup_directory_builder(&mut archive).build().unwrap();
    assert_eq!(tuple, ());

    let id = lookup(&mut archive, 0).unwrap().clone();
    assert!(lookup(&mut archive, 1).is_none());

    let mut reader = archive.container.read_buf(&id).unwrap();
    let entry = reader.deserialize::<Inner>().unwrap();

    assert_eq!(entry.name, "foo");
    assert_eq!(entry.size, 0);
    assert!(entry.mode.is_directory());
}
