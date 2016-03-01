use std::env;
use std::fs;
use std::io::Read;
use std::path;

use cli::error::{TrussCliError, ErrorKind};

use toml;

#[derive(Debug, Clone)]
pub struct Config {
    pub listen: String,
    pub repository_root: path::PathBuf,
    pub asset_root: path::PathBuf,
    pub template_root: path::PathBuf
}

pub fn read_file(filepath: &path::Path) -> Result<(), TrussCliError> {
    let configpath: &path::PathBuf = &env::current_dir().unwrap().join(filepath);
    let mut s = String::new();
    let mut f = try!(fs::File::open(configpath));
    try!(f.read_to_string(&mut s));

    let mut parser = toml::Parser::new(&s);
    let parsed = match parser.parse() {
        Some(parsed) => parsed,
        // wtf
        None => return Err(TrussCliError { kind: ErrorKind::FileNotFound, detail: Some("what".to_owned()) }),
    };

    for (key, value) in &parsed {
        println!("{}: {}", key, value);
    }
    /*
     * this was in the None branch for parser.parse
        None => {
            for err in &parser.errors {
                let (loline, locol) = parser.to_linecol(err.lo);
                let (hiline, hicol) = parser.to_linecol(err.hi);
                println!("{}:{}:{}-{}:{} error: {}",
                        configpath.to_str().unwrap(), loline, locol, hiline, hicol, err.desc);
            }
            panic!("Error reading configuration file");
        }
    */

    /*
    Ok(Config {
        listen: parsed.get("listen"),
        repository_root: parsed.get(
        asset_root: *configpath,
        template_root: *configpath
    })
    */

    /*
    Ok(Config {
        listen: listen,
        repository_root: repository_root,
        asset_root: asset_root,
        template_root: template_root
    })
    */
    Ok(())
}
