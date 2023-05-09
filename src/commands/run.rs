use colored::Colorize;

use crate::{
    args::RunArgs,
    lib::{actions, config::Config, process::ExecError},
};

pub fn exec(args: &RunArgs, config: &Config) -> Result<(), ExecError> {
    println!("{}", "42 CLI - Run".bright_magenta().bold());

    let RunArgs { clean } = args;
    let mut error = ExecError {
        command: String::from(""),
        trace: String::from(""),
        exit_code: 0,
    };

    if let Err(e) = actions::build(config) {
        error = e;
    }

    if let Err(e) = actions::run(config) {
        error = e;
    }

    if *clean {
        println!("{}", "Not running cleanup script.".yellow());
    } else if let Err(e) = actions::clean(config) {
        error = e;
    }

    if error.exit_code != 0 {
        return Err(error);
    }

    Ok(())
}
