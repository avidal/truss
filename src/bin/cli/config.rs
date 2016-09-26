use std::env;
use std::fs;
use std::io::Read;
use std::path;
use std::collections::BTreeMap;

use cli::error::{TrussCliError, ErrorKind};

use toml::{Parser, Value, Decoder};
use toml;

#[derive(Serialize, Deserialize, Debug, Show)]
pub struct Config {
    pub bind: String,
    pub port: u16,

    pub admins: Vec<String>,

    pub users: Vec<UserConfig>,
}


#[derive(Serialize, Deserialize, Debug, Show)]
pub struct UserConfig {
    pub username: String,
    pub password: String,
    
    pub real: Option<String>,
    pub nick: Option<String>,
    pub nick2: Option<String>,
    pub nick3: Option<String>,

    pub servers: BTreeMap<String, ServerConfig>,
}

impl UserConfig {
    fn best_nick(self) -> String {
        self.nick.unwrap_or(self.nick2.unwrap_or(self.nick3.unwrap_or(self.username)))
    }
}

#[derive(Serialize, Deserialize, Debug, Show)]
pub struct ServerConfig {
    pub server: String,

    pub real: Option<String>,
    pub nick: Option<String>,
    pub nick2: Option<String>,
    pub nick3: Option<String>,
}

pub fn read_file(filepath: &path::Path) -> Result<Config, TrussCliError> {
    let configpath: &path::PathBuf = &env::current_dir().unwrap().join(filepath);
    let mut s = String::new();
    let mut f = try!(fs::File::open(configpath));
    try!(f.read_to_string(&mut s));

    let mut parser = Parser::new(&s);
    let toml = parser.parse();

    if toml.is_none() {
        panic!("Could not parse config file");
    }

    let config = Value::Table(toml.unwrap());
    let c: Config = toml::decode(config).unwrap();
    println!("{:?}", c);
    Ok(c)
}
