use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::fmt;
use std::error;
use std::path;

use toml;

#[derive(Clone)]
pub struct Config {
    pub listen: String,
    pub repository_root: path::PathBuf,
    pub asset_root: path::PathBuf,
    pub template_root: path::PathBuf
}

#[derive(Debug)]
enum TrussError {
    Io(io::Error),
    Config(toml::Error),
    Fmt(fmt::Error)
}

impl Config {
    pub fn read_file(filepath: &path::Path) -> Result<Self, TrussError> {
        let configpath: &path::PathBuf = &env::current_dir().unwrap().join(filepath);
        let mut s = String::new();
        let mut f = try!(fs::File::open(configpath));
        try!(f.read_to_string(&mut s));

        let mut parser = toml::Parser::new(&s);
        let parsed = match parser.parse() {
            Some(parsed) => parsed,
            None => {
				for err in &parser.errors {
					let (loline, locol) = parser.to_linecol(err.lo);
					let (hiline, hicol) = parser.to_linecol(err.hi);
					println!("{}:{}:{}-{}:{} error: {}",
							configpath.to_str().unwrap(), loline, locol, hiline, hicol, err.desc);
				}
				panic!("Error reading configuration file");
            }
        };

        println!("parsed: {}", parsed);

        Ok(Config {
            listen: "0:8080",
            repository_root: *configpath,
            asset_root: *configpath,
            template_root: *configpath
        })

        /*
        Ok(Config {
            listen: listen,
            repository_root: repository_root,
            asset_root: asset_root,
            template_root: template_root
        })
        */
    }
}
