#![feature(plugin)]
#![plugin(clippy)]

extern crate truss;

extern crate clap;
extern crate git2;

mod commands;

fn main() {
    let matches = clap::App::new("truss-cli")
        .subcommand(clap::SubCommand::with_name("ls")
                    .about("List all repositories with latest commit"))
        .get_matches();

    if matches.is_present("ls") {
        commands::ls::run();
    }
}
