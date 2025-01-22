// MIT License
//
// Copyright (c) 2024,2025 Robin Doer
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

#[allow(dead_code)]
mod common;
#[allow(dead_code)]
mod predicates_ext;

use assert_cmd::Command;
use assert_fs::fixture::TempDir;
use predicates::prelude::PredicateBooleanExt;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::common::{container_create, handle_password_args, nuts_tool, setup};
use crate::predicates_ext::{hash, list};

fn archive_add(home: &Path, name: Option<&str>, pass: Option<&[u8]>) -> Command {
    let mut cmd = nuts_tool(home, ["archive", "add"]);

    if let Some(name) = name {
        cmd.args(["--container", name]);
    }

    handle_password_args(cmd, pass)
}

fn archive_create(home: &Path, name: Option<&str>, pass: Option<&[u8]>) -> Command {
    let mut cmd = nuts_tool(home, ["archive", "create"]);

    if let Some(name) = name {
        cmd.args(["--container", name]);
    }

    handle_password_args(cmd, pass)
}

fn archive_info(home: &Path, name: &str, pass: Option<&[u8]>) -> Command {
    let cmd = nuts_tool(home, ["archive", "info", "--container", name]);

    handle_password_args(cmd, pass)
}

fn archive_list(home: &Path, name: &str, pass: Option<&[u8]>) -> Command {
    let cmd = nuts_tool(home, ["archive", "list", "--container", name]);

    handle_password_args(cmd, pass)
}

fn setup_archive() -> TempDir {
    let tmp_dir = setup();

    container_create(&tmp_dir, "sample", "directory", Some(b"123"))
        .assert()
        .success();
    archive_create(&tmp_dir, Some("sample"), Some(b"123"))
        .assert()
        .success();

    tmp_dir
}

#[test]
fn help() {
    let tmp_dir = TempDir::new().unwrap();

    for (args, migrate_arg) in [
        (["archive", "--help"].as_slice(), true), // FIXME
        (["archive", "add", "--help"].as_slice(), true),
        (["archive", "add", "file", "--help"].as_slice(), true),
        (["archive", "add", "directory", "--help"].as_slice(), true),
        (["archive", "add", "symlink", "--help"].as_slice(), true),
        (["archive", "create", "--help"].as_slice(), false),
        (["archive", "get", "--help"].as_slice(), true),
        (["archive", "info", "--help"].as_slice(), true),
        (["archive", "list", "--help"].as_slice(), true),
        (["archive", "migrate", "--help"].as_slice(), false),
    ] {
        let password_from_fd = predicates::str::contains("--password-from-fd");
        let password_from_file = predicates::str::contains("--password-from-file");
        let container = predicates::str::contains("--container");
        let migrate = predicates::str::contains("--migrate");
        let verbose = predicates::str::contains("--verbose");
        let quiet = predicates::str::contains("--quiet");

        let assert = nuts_tool(&tmp_dir, args)
            .assert()
            .success()
            .stdout(
                password_from_fd
                    .and(password_from_file)
                    .and(container)
                    .and(verbose)
                    .and(quiet),
            )
            .stderr("");

        if migrate_arg {
            assert.stdout(migrate);
        } else {
            assert.stdout(migrate.not());
        }
    }
}

#[test]
fn add() {
    let tmp_dir = setup_archive();
    let f1 = tmp_dir.join("f1.txt");
    let f2 = tmp_dir.join("f2.txt");
    let d1 = tmp_dir.join("d");
    let f3 = d1.join("f3.txt");
    let f4 = d1.join("f4.txt");

    {
        fs::create_dir(&d1).unwrap();

        let mut f = File::create(&f1).unwrap();
        f.write_all(b"xxx").unwrap();
        f.flush().unwrap();
        f.sync_all().unwrap();

        if cfg!(unix) {
            std::os::unix::fs::symlink(&f1, &f2).unwrap();
        } else {
            panic!("make a symlink on your target platform");
        }

        let mut f = File::create(&f3).unwrap();
        f.write_all(b"xxx").unwrap();
        f.flush().unwrap();
        f.sync_all().unwrap();

        if cfg!(unix) {
            std::os::unix::fs::symlink(&f3, &f4).unwrap();
        } else {
            panic!("make a symlink on your target platform");
        }
    }

    archive_add(&tmp_dir, None, Some(b"123"))
        .assert()
        .code(1)
        .stdout("")
        .stderr("error: a value is required for '--container' but none was supplied\n\n");
    archive_add(&tmp_dir, Some("xxx"), Some(b"123"))
        .assert()
        .code(1)
        .stdout("")
        .stderr("no such container: xxx\n");
    archive_add(&tmp_dir, Some("sample"), Some(b"xxx"))
        .assert()
        .code(1)
        .stdout("")
        .stderr("the password is wrong\n");

    archive_add(&tmp_dir, Some("sample"), Some(b"123"))
        .arg(f1.to_str().unwrap())
        .assert()
        .success()
        .stdout(format!("a {}\n", f1.display()))
        .stderr("");
    archive_list(&tmp_dir, "sample", Some(b"123"))
        .assert()
        .success()
        .stdout(list::eq([f1.to_str().unwrap()]));

    archive_add(&tmp_dir, Some("sample"), Some(b"123"))
        .arg(f2.to_str().unwrap())
        .assert()
        .success()
        .stdout(format!("a {}\n", f2.display()))
        .stderr("");
    archive_list(&tmp_dir, "sample", Some(b"123"))
        .assert()
        .success()
        .stdout(list::eq([f1.to_str().unwrap(), f2.to_str().unwrap()]));

    archive_add(&tmp_dir, Some("sample"), Some(b"123"))
        .arg(d1.to_str().unwrap())
        .assert()
        .success()
        .stdout(list::unordered([
            format!("a {}", d1.display()),
            format!("a {}", f3.display()),
            format!("a {}", f4.display()),
        ]))
        .stderr("");
    archive_list(&tmp_dir, "sample", Some(b"123"))
        .assert()
        .success()
        .stdout(list::unordered([
            f1.to_str().unwrap(),
            f2.to_str().unwrap(),
            d1.to_str().unwrap(),
            f3.to_str().unwrap(),
            f4.to_str().unwrap(),
        ]));
}

#[test]
#[ignore]
fn add_file() {}

#[test]
#[ignore]
fn add_directory() {}

#[test]
#[ignore]
fn add_symlink() {}

#[test]
fn create() {
    let tmp_dir = setup();
    let d1 = tmp_dir.join("d");
    let f1 = d1.join("f1.txt");
    let f2 = d1.join("f2.txt");

    {
        fs::create_dir(&d1).unwrap();

        let mut f = File::create(&f1).unwrap();
        f.write_all(b"xxx").unwrap();
        f.flush().unwrap();
        f.sync_all().unwrap();

        if cfg!(unix) {
            std::os::unix::fs::symlink(&f1, &f2).unwrap();
        } else {
            panic!("make a symlink on your target platform");
        }
    }

    archive_create(&tmp_dir, None, Some(b"123"))
        .assert()
        .code(1)
        .stdout("")
        .stderr("error: a value is required for '--container' but none was supplied\n\n");
    archive_create(&tmp_dir, Some("sample"), Some(b"123"))
        .assert()
        .code(1)
        .stdout("")
        .stderr("no such container: sample\n");

    container_create(&tmp_dir, "sample", "directory", Some(b"123"))
        .assert()
        .success();

    archive_create(&tmp_dir, Some("sample"), Some(b"xxx"))
        .assert()
        .code(1)
        .stdout("")
        .stderr("the password is wrong\n");

    archive_create(&tmp_dir, Some("sample"), Some(b"123"))
        .assert()
        .success()
        .stdout("")
        .stderr("");
    archive_info(&tmp_dir, "sample", Some(b"123"))
        .assert()
        .success()
        .stdout(hash::contains([("blocks", "0"), ("files", "0")]))
        .stderr("");

    archive_create(&tmp_dir, Some("sample"), Some(b"123"))
        .assert()
        .code(1)
        .stdout("")
        .stderr("the container is already acquired by a service (sid = 0x61726368)\n");

    container_create(&tmp_dir, "sample1", "directory", Some(b"123"))
        .assert()
        .success();
    archive_create(&tmp_dir, Some("sample1"), Some(b"123"))
        .arg(f1.to_str().unwrap())
        .assert()
        .success()
        .stdout(format!("a {}\n", f1.display()))
        .stderr("");
    archive_list(&tmp_dir, "sample1", Some(b"123"))
        .assert()
        .success()
        .stdout(list::eq([f1.to_str().unwrap()]));

    container_create(&tmp_dir, "sample2", "directory", Some(b"123"))
        .assert()
        .success();
    archive_create(&tmp_dir, Some("sample2"), Some(b"123"))
        .arg(f2.to_str().unwrap())
        .assert()
        .success()
        .stdout(format!("a {}\n", f2.display()))
        .stderr("");
    archive_list(&tmp_dir, "sample2", Some(b"123"))
        .assert()
        .success()
        .stdout(list::eq([f2.to_str().unwrap()]));

    container_create(&tmp_dir, "sample3", "directory", Some(b"123"))
        .assert()
        .success();
    archive_create(&tmp_dir, Some("sample3"), Some(b"123"))
        .arg(d1.to_str().unwrap())
        .assert()
        .success()
        .stdout(list::unordered([
            format!("a {}", d1.display()),
            format!("a {}", f1.display()),
            format!("a {}", f2.display()),
        ]))
        .stderr("");
    archive_list(&tmp_dir, "sample3", Some(b"123"))
        .assert()
        .success()
        .stdout(list::unordered([
            d1.to_str().unwrap(),
            f1.to_str().unwrap(),
            f2.to_str().unwrap(),
        ]));
}

#[test]
#[ignore]
fn get() {}

#[test]
#[ignore]
fn info() {}

#[test]
#[ignore]
fn list() {}

#[test]
#[ignore]
fn migrate() {}
