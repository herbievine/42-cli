use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub scripts: Scripts,
    pub children: Option<Vec<String>>,
    pub prepend_path: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Command {
    pub cmd: String,
    pub dir: Option<String>,
    pub mlx: Option<bool>,
    pub mlx_dir: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Scripts {
    pub build: Option<Command>,
    pub run: Option<Vec<Command>>,
    pub test: Option<Vec<Command>>,
    pub clean: Option<Command>,
    pub norm: Option<Command>,
}

// name = "42-cli"

// [scripts]
// build = [{ cmd = "echo build" }]
// test = [{ cmd = "echo test" }]
// clean = [{ cmd = "echo clean" }]
// "#;

impl Config {
    pub fn new(path: &str) -> Config {
        let raw_config = get_raw_config(path).unwrap_or_else(|| {
            println!("Error: No config file found.");
            std::process::exit(1);
        });
        match toml::from_str::<Config>(&raw_config) {
            Ok(mut config) => {
                config.prepend_path = Some(String::from(path));
                config
            }
            Err(e) => {
                println!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn get_raw_config(path: &str) -> Option<String> {
    let path = format!("{}/42-cli.toml", path);

    if let Ok(config) = fs::read_to_string(path) {
        Some(config)
    } else {
        None
    }
}

pub fn get_config() -> Config {
    if let Some(raw_config) = get_raw_config(".") {
        match toml::from_str::<Config>(&raw_config) {
            Ok(config) => config,
            Err(e) => {
                println!("Error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        let default_config = r#"name = "42-cli"

[scripts]
install = [{ cmd = "echo install" }]
test = [{ cmd = "echo test" }]
clean = [{ cmd = "echo clean" }]
        "#;
        match fs::write("42-cli.toml", default_config) {
            Ok(_) => {
                println!("We created a default config file for you.");
                println!("Run `code 42-cli.toml` to edit it.");
                std::process::exit(1);
            }
            Err(e) => {
                println!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
