use crate::{
    args::RunArgs,
    lib::{
        config::Config,
        process::{self, CommandError},
    },
};
use colored::Colorize;

pub fn exec(args: &RunArgs, config: &Config) -> CommandError {
    let mut error = CommandError {
        command: String::from(""),
        trace: String::from(""),
        exit_code: 0,
    };
    let RunArgs { clean } = args;

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

    if config.scripts.run.is_some() {
        println!("{}", "Running run script...".bright_green().bold());

        for script in config.scripts.run.as_ref().unwrap() {
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

    if *clean {
        println!("{}", "Not running cleanup script.".yellow());
    } else if config.scripts.clean.is_some() {
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
