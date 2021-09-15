// SPDX-FileCopyrightText: 2021 Brian Kubisiak <brian@kubisiak.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use std::io::Write;
use std::path::Path;
use std::process::Command;

use clap::ArgMatches;

pub fn run(args: &ArgMatches) {
    // these arguments are required, so should never be none
    let revisions = args.value_of("REVISIONS").unwrap();
    let src = Path::new(args.value_of("src-repo").unwrap());

    let patches = format_patches(&src, revisions);
    apply_patches(&patches, args.value_of("dst-directory"));
}

fn format_patches(src: &Path, revisions: &str) -> Vec<String> {
    let output = Command::new("git")
        .args(["-C", src.to_str().unwrap(), "format-patch", revisions])
        .output()
        .expect("internal error during format-patch");

    if !output.status.success() {
        std::io::stderr().write_all(&output.stderr).unwrap();
        std::process::exit(output.status.code().unwrap());
    }

    let patch_output = std::str::from_utf8(&output.stdout).unwrap();
    return patch_output
        .lines()
        .map(|line| src.join(line).as_path().to_str().unwrap().to_string())
        .collect();
}

fn apply_patches(patches: &[String], dir: Option<&str>) {
    let mut git_am = Command::new("git");
    git_am.arg("am");
    if let Some(dst_dir) = dir {
        git_am.args(["--directory", dst_dir]);
    }
    git_am.args(patches);

    let rc = git_am.status().expect("internal error during git am");

    for patch in patches {
        if let Err(reason) = std::fs::remove_file(patch) {
            eprintln!("warning: failed to remove {}: {}", patch, reason);
        }
    }

    if !rc.success() {
        std::process::exit(rc.code().unwrap());
    }
}
