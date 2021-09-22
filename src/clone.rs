// SPDX-FileCopyrightText: 2021 Brian Kubisiak <brian@kubisiak.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use clap::ArgMatches;

pub fn run(args: &ArgMatches) {
    if args.is_present("squash") {
        squash(args);
    } else {
        // TODO: implement this
        panic!("git-third-party clone must be --squash'd!");
    }
}

fn toplevel() -> String {
    let output = Command::new("git")
        .args(&["rev-parse", "--show-toplevel"])
        .output()
        .expect("internal error during rev-parse");
    if !output.status.success() {
        std::io::stderr().write_all(&output.stderr).unwrap();
        std::process::exit(output.status.code().unwrap());
    }

    return std::str::from_utf8(&output.stdout)
        .unwrap()
        .trim_end()
        .to_string();
}

fn squash(args: &ArgMatches) {
    let toplevel_dir = toplevel();

    let vendor_dir = if let Some(dst_dir) = args.value_of("dst-directory") {
        let toplevel_path = Path::new(&toplevel_dir);
        toplevel_path
            .join(dst_dir)
            .as_path()
            .to_str()
            .unwrap()
            .to_string()
    } else {
        toplevel_dir
    };

    if let Err(e) = std::fs::create_dir_all(&vendor_dir) {
        eprintln!("error: failed to create {}: {}", vendor_dir, e);
        std::process::exit(1);
    }

    let mut git_archive = Command::new("git")
        .args(&[
            "-C",
            // this is a required arg and can't fail
            args.value_of("src-repo").unwrap(),
            "archive",
            "--format",
            "tar",
            // this has a default value and can't fail
            args.value_of("TREE-ISH").unwrap(),
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("internal error during archive");

    let mut tar = Command::new("tar")
        .args(&["-C", &vendor_dir, "-x"])
        .stdin(git_archive.stdout.take().unwrap())
        .spawn()
        .expect("internal error during tar");

    let git_archive_rc = git_archive.wait().unwrap();
    if !git_archive_rc.success() {
        // we're already exiting with an error and have to wait on
        // tar; but we don't care about the result since we already
        // have our exit code
        let _ = tar.wait();
        std::process::exit(git_archive_rc.code().unwrap());
    }

    let tar_rc = tar.wait().unwrap();
    if !tar_rc.success() {
        std::process::exit(tar_rc.code().unwrap());
    }
}
