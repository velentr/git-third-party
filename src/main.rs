// SPDX-FileCopyrightText: 2021 Brian Kubisiak <brian@kubisiak.com>
//
// SPDX-License-Identifier: GPL-3.0-only

extern crate clap;
use clap::{App, Arg, SubCommand};

mod cherrypick;

fn main() {
    let matches = App::new("git-third-party")
        .version("0.0")
        .author("Brian Kubisiak <brian@kubisiak.com>")
        .about("Manage vendored third-party code in git")
        .subcommand(
            SubCommand::with_name("cherry-pick")
                .about("Cherry-pick a series of patches between repos")
                .arg(
                    Arg::with_name("src-repo")
                        .short("s")
                        .help("Path to the source git repo")
                        .required(true)
                        .value_name("PATH")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("dst-directory")
                        .short("d")
                        .help("Subdirectory where the code is vendored")
                        .value_name("PATH")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("REVISIONS")
                        .help("Source git revisions to cherry-pick")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("cherry-pick", Some(submatch)) => {
            cherrypick::run(submatch);
        }
        _ => {
            // no subcommand
        }
    }
}
