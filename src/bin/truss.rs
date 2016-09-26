#![feature(plugin, custom_derive)]
#![plugin(clippy)]
#![plugin(serde_macros)]
use std::path;

extern crate truss;

extern crate clap;
extern crate toml;
extern crate serde;

mod commands;
mod cli;
use cli::config;

fn main() {
    let matches = clap::App::new("truss-cli")
        .subcommand(clap::SubCommand::with_name("ls")
                    .about("List all repositories with latest commit"))
        .get_matches();

    match config::read_file(path::Path::new("./truss.toml")) {
        Ok(_) => println!("Read configuration file"),
        Err(e) => println!("Error: {:?}", e),
    }
}
