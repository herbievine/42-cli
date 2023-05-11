use colored::Colorize;
use serde::Deserialize;
use std::fs;

use crate::{args::RunArgs, lib::actions};

use super::process::ExecError;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub scripts: Scripts,
    pub projects: Option<Vec<String>>,
    pub run_in: Option<String>,
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
    pub lint: Option<Command>,
}

impl Config {
    /// Get the raw config file from the given dir. If no dir is given, we assume
    /// the config file is in the current directory, and if it's not, we create it.
    pub fn new(dir: &str) -> Config {
        let config_path = format!("{}/42-cli.toml", dir);

        let raw_config = if let Ok(config) = fs::read_to_string(&config_path) {
            Some(config)
        } else {
            None
        };

        if raw_config.is_none() && dir == "." {
            let default_config = r#"name = "42-cli"

[scripts]
install = [{ cmd = "make re" }]
test = [{ cmd = "echo test script" }]
clean = [{ cmd = "make fclean" }]
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
        } else if raw_config.is_none() && dir != "." {
            println!("Error: No config file found at {}", config_path);
            std::process::exit(1);
        }

        match toml::from_str::<Config>(&raw_config.unwrap()) {
            Ok(mut config) => {
                if dir != "." {
                    config.run_in = Some(String::from(dir));
                }
                config
            }
            Err(e) => {
                println!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }

    /// Run the build script.
    pub fn build(&self) -> Result<(), ExecError> {
        println!("{}", "42 CLI - Build".bright_magenta().bold());

        let mut error = ExecError {
            command: String::from(""),
            trace: String::from(""),
            exit_code: 0,
        };

        if let Err(e) = actions::build(self) {
            error = e;
        }

        if error.exit_code != 0 {
            return Err(error);
        }

        Ok(())
    }

    /// Run the clean script.
    pub fn clean(&self) -> Result<(), ExecError> {
        println!("{}", "42 CLI - Clean".bright_magenta().bold());

        let mut error = ExecError {
            command: String::from(""),
            trace: String::from(""),
            exit_code: 0,
        };

        if let Err(e) = actions::clean(self) {
            error = e;
        }

        if error.exit_code != 0 {
            return Err(error);
        }

        Ok(())
    }

    /// Run the lint script.
    pub fn lint(&self) -> Result<(), ExecError> {
        println!("{}", "42 CLI - Lint".bright_magenta().bold());

        let mut error = ExecError {
            command: String::from(""),
            trace: String::from(""),
            exit_code: 0,
        };

        if let Err(e) = actions::lint(self) {
            error = e;
        }

        if error.exit_code != 0 {
            return Err(error);
        }

        Ok(())
    }

    /// Run the run script (dependency on `build`).
    pub fn run(&self, args: &RunArgs) -> Result<(), ExecError> {
        println!("{}", "42 CLI - Run".bright_magenta().bold());

        let RunArgs { clean } = args;
        let mut error = ExecError {
            command: String::from(""),
            trace: String::from(""),
            exit_code: 0,
        };

        if let Err(e) = actions::build(self) {
            error = e;
        }

        if let Err(e) = actions::run(self) {
            error = e;
        }

        if *clean {
            println!("{}", "Not running cleanup script.".yellow());
        } else if let Err(e) = actions::clean(self) {
            error = e;
        }

        if error.exit_code != 0 {
            return Err(error);
        }

        Ok(())
    }

    /// Run the test script (dependency on `build`).
    pub fn test(&self) -> Result<(), ExecError> {
        println!("{}", "42 CLI - Test".bright_magenta().bold());

        let mut error = ExecError {
            command: String::from(""),
            trace: String::from(""),
            exit_code: 0,
        };

        if let Err(e) = actions::build(self) {
            error = e;
        }

        if let Err(e) = actions::test(self) {
            error = e;
        }

        if let Err(e) = actions::clean(self) {
            error = e;
        }

        if error.exit_code != 0 {
            return Err(error);
        }

        Ok(())
    }
}
