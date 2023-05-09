use colored::Colorize;

use crate::{
    args::TestArgs,
    lib::{actions, config::Config, process::ExecError},
};

pub fn exec(_: &TestArgs, config: &Config) -> Result<(), ExecError> {
    println!("{}", "42 CLI - Test".bright_magenta().bold());

    let mut error = ExecError {
        command: String::from(""),
        trace: String::from(""),
        exit_code: 0,
    };

    if let Err(e) = actions::build(config) {
        error = e;
    }

    if let Err(e) = actions::test(config) {
        error = e;
    }

    if let Err(e) = actions::clean(config) {
        error = e;
    }

    if error.exit_code != 0 {
        return Err(error);
    }

    Ok(())
}
