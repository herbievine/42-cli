use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub name: String,
    pub program: Option<String>,
    pub scripts: Scripts,
}

#[derive(Deserialize)]
pub struct Command {
    pub cmd: String,
    pub dir: Option<String>,
}

#[derive(Deserialize)]
pub struct Scripts {
    pub install: Option<Vec<Command>>,
    pub test: Option<Vec<Command>>,
    pub clean: Option<Vec<Command>>,
}
fn get_raw_config() -> String {
    fs::read_to_string("42-cli.toml").unwrap()
}

pub fn get_config() -> Config {
    let raw_config = get_raw_config();
    match toml::from_str::<Config>(&raw_config) {
        Ok(config) => config,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
