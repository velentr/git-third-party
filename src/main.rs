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
                        .long("src-repo")
                        .help("Path to the source git repo")
                        .required(true)
                        .value_name("PATH")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("src-directory")
                        .short("S")
                        .long("src-directory")
                        .help("Directory in the source repo to pull patches from")
                        .value_name("PATH")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("dst-directory")
                        .short("d")
                        .long("dst-directory")
                        .help("Directory in the destination repo where patches are applied")
                        .value_name("PATH")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("trailer")
                        .long("trailer")
                        .help("Specify a (<token>, <value>) pair to apply as a trailer")
                        .value_name("TOKEN>=<VALUE")
                        .takes_value(true)
                        .multiple(true)
                        .number_of_values(2)
                        .require_delimiter(true)
                        .value_delimiter("="),
                )
                .arg(
                    Arg::with_name("REVISIONS")
                        .help("Source git revisions to cherry-pick")
                        .required(true)
                        .index(1),
                )
                .arg(Arg::with_name("single-commit").short("1").help(
                    "Treat <REVISIONS> as a single commit to cherry-pick",
                )),
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
