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

    let patches = format_patches(
        &src,
        revisions,
        args.is_present("single-commit"),
        args.value_of("src-directory"),
    );

    if let Some(trailers) = args.values_of("trailer") {
        add_trailers(&patches, trailers);
    }

    apply_patches(
        &patches,
        args.value_of("dst-directory"),
        args.value_of("src-directory"),
    );
}

fn format_patches(
    src: &Path,
    revisions: &str,
    single_commit: bool,
    src_directory: Option<&str>,
) -> Vec<String> {
    let mut git = Command::new("git");
    git.args(&["-C", src.to_str().unwrap(), "format-patch"]);

    if single_commit {
        git.arg("-1");
    }

    git.arg(revisions);

    // filter patches to only include changes in the given source directory
    if let Some(src_dir) = src_directory {
        git.args(&["--", src_dir]);
    }

    let output = git.output().expect("internal error during format-patch");

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

fn add_trailers<'a>(
    patches: &[String],
    trailers: impl std::iter::Iterator<Item = &'a str>,
) {
    let mut git_interpret_trailers = Command::new("git");
    git_interpret_trailers.args(&["interpret-trailers", "--in-place"]);

    // transform a list of [token0, value0, token1, value1] into
    // [(token0, value0), (token1, value1)]
    let (tokens, values): (Vec<(usize, &str)>, Vec<(usize, &str)>) =
        trailers.enumerate().partition(|&(i, _)| i % 2 == 0);
    for ((_, token), (_, value)) in tokens.iter().zip(values.iter()) {
        let trailer = format!("{}={}", token, value);
        git_interpret_trailers.args(&["--trailer", &trailer]);
    }

    // Note that this will respect the user's git config for trailer
    // placement (eg trailer.<token>.where). I recommend using the
    // config file instead of passing args through to
    // git-interpret-trailers(1).
    git_interpret_trailers.args(patches);

    let rc = git_interpret_trailers
        .status()
        .expect("internal error during git interpret-trailers");
    if !rc.success() {
        std::process::exit(rc.code().unwrap());
    }
}

fn apply_patches(
    patches: &[String],
    dst_directory: Option<&str>,
    src_directory: Option<&str>,
) {
    let mut git_am = Command::new("git");
    git_am.arg("am");

    if let Some(dst_dir) = dst_directory {
        git_am.args(&["--directory", dst_dir]);
    }

    // if the patch was filtered on a subdirectory in the source repo,
    // remove those path components to re-root it in this repo
    if let Some(src_dir) = src_directory {
        // TODO: this doesn't work correctly if the patch is filtered
        // by a file instead of a directory
        let num_components = Path::new(src_dir).components().count();
        // +1 here to account for the leading 'b/' (note that the
        // default argument is -p1)
        git_am.arg(format!("-p{}", num_components + 1));
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
