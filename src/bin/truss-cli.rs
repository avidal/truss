#![feature(plugin)]
#![plugin(clippy)]

extern crate truss;

extern crate clap;
extern crate git2;

use git2::{Repository, Commit, Signature};
use truss::config::Config;

fn main() {
    let matches = clap::App::new("truss-cli")
        .subcommand(clap::SubCommand::with_name("ls")
                    .about("List all repositories with latest commit"))
        .get_matches();

    if matches.is_present("ls") {
        let repo = match Repository::open("repositories/git2-rs.git") {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
        };

        let head = repo.head().unwrap().target().unwrap();
        let mut commit: Commit = repo.find_commit(head).unwrap();
        {
            let summary = commit.summary().unwrap();
            println!("commit summary {}", summary);
        }
        let author: Signature = commit.author();
        println!("author {}", author);

    }

    let config: Config = match Config::read_file("./truss.toml") {
        Ok(x) => x,
        Err(e) => panic!("Unable to parse config file: {}", e)
    };
}
