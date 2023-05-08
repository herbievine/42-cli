use crate::{
    args::TestArgs,
    lib::{
        config::Config,
        process::{self, CommandError},
    },
};
use colored::Colorize;

pub fn exec(_: &TestArgs, config: &Config) -> CommandError {
    println!("{}", "42 CLI - Test".bright_magenta().bold());

    let mut error = CommandError {
        command: String::from(""),
        trace: String::from(""),
        exit_code: 0,
    };

    if config.scripts.install.is_some() {
        println!("{}", "Running install script...".bright_green().bold());

        for script in config.scripts.install.as_ref().unwrap() {
            match process::exec_command(
                &script.cmd,
                script.dir.as_ref().unwrap_or(&String::from(".")),
                script.pipe.as_ref().map(|x| &**x),
            ) {
                Ok(_) => (),
                Err(e) => error = e,
            }
        }
    } else {
        println!("{}", "No install script found.".yellow());
    }

    if config.scripts.test.is_some() {
        println!("{}", "Running test script...".bright_green().bold());

        for script in config.scripts.test.as_ref().unwrap() {
            match process::exec_command(
                &script.cmd,
                script.dir.as_ref().unwrap_or(&String::from(".")),
                script.pipe.as_ref().map(|x| &**x),
            ) {
                Ok(_) => (),
                Err(e) => error = e,
            }
        }
    } else {
        println!("{}", "No test script found.".yellow());
    }

    if config.scripts.clean.is_some() {
        println!("{}", "Running cleanup script...".bright_green().bold());

        for script in config.scripts.clean.as_ref().unwrap() {
            match process::exec_command(
                &script.cmd,
                script.dir.as_ref().unwrap_or(&String::from(".")),
                script.pipe.as_ref().map(|x| &**x),
            ) {
                Ok(_) => (),
                Err(e) => error = e,
            }
        }
    } else {
        println!("{}", "No clean script found.".yellow());
    }

    error
}
