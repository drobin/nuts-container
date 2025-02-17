// MIT License
//
// Copyright (c) 2024 Robin Doer
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

use crate::migrate::{Migration, MigrationError, Migrator};

struct OkSample;

impl Migration for OkSample {
    fn migrate_rev0(&self, userdata: &[u8]) -> Result<(u32, Vec<u8>), String> {
        Ok((666, userdata.to_vec()))
    }
}

struct ErrSample;

impl Migration for ErrSample {
    fn migrate_rev0(&self, _userdata: &[u8]) -> Result<(u32, Vec<u8>), String> {
        Err("xxx".to_string())
    }
}

#[test]
fn rev0_assigned_ok() {
    let migrator = Migrator::default().with_migration(OkSample);

    let (sid, top_id) = migrator.migrate_rev0(&[1, 2, 3]).unwrap().unwrap();

    assert_eq!(sid, 666);
    assert_eq!(*top_id, [1, 2, 3]);
}

#[test]
fn rev0_assigned_err() {
    let migrator = Migrator::default().with_migration(ErrSample);

    let err = migrator.migrate_rev0(&[1, 2, 3]).unwrap_err();

    assert!(matches!(err, MigrationError::Rev0(cause) if cause == "xxx"));
}

#[test]
fn rev0_unassigned() {
    let migrator = Migrator::default();

    let opt = migrator.migrate_rev0(&[1, 2, 3]).unwrap();

    assert!(opt.is_none());
}
