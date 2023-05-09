use crate::{
    args::BuildArgs,
    lib::{actions, config::Config, process::ExecError},
};
use colored::Colorize;

pub fn exec(_: &BuildArgs, config: &Config) -> Result<(), ExecError> {
    println!("{}", "42 CLI - Build".bright_magenta().bold());

    let mut error = ExecError {
        command: String::from(""),
        trace: String::from(""),
        exit_code: 0,
    };

    if let Err(e) = actions::build(config) {
        error = e;
    }

    if error.exit_code != 0 {
        return Err(error);
    }

    Ok(())
}
