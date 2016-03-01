#![feature(plugin)]
#![plugin(clippy)]

use std::path;

extern crate truss;

extern crate clap;
extern crate git2;
extern crate toml;

mod commands;
mod cli;
use cli::config;

fn main() {
    let matches = clap::App::new("truss-cli")
        .subcommand(clap::SubCommand::with_name("ls")
                    .about("List all repositories with latest commit"))
        .get_matches();

    if matches.is_present("ls") {
        commands::ls::run();
    }

    config::read_file(path::Path::new("./truss.toml"));
}
