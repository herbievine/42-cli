use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub name: String,
    pub scripts: Scripts,
}

#[derive(Deserialize)]
pub struct Command {
    pub cmd: String,
    pub pipe: Option<String>,
    pub dir: Option<String>,
}

#[derive(Deserialize)]
pub struct Scripts {
    pub install: Option<Vec<Command>>,
    pub run: Option<Vec<Command>>,
    pub test: Option<Vec<Command>>,
    pub clean: Option<Vec<Command>>,
}

// name = "42-cli"

// [scripts]
// install = [{ cmd = "echo install" }]
// test = [{ cmd = "echo test" }]
// clean = [{ cmd = "echo clean" }]
// "#;

fn get_raw_config() -> Option<String> {
    if let Ok(config) = fs::read_to_string("42-cli.toml") {
        Some(config)
    } else {
        None
    }
}

pub fn get_config() -> Config {
    if let Some(raw_config) = get_raw_config() {
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
