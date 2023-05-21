use std::{
    path::Path,
    process::{Command, Stdio},
};

use super::config::Config;

#[derive(Debug)]
pub struct ExecError {
    pub command: String,
    pub context: String,
    pub exit_code: i32,
}

pub fn exec_command<P: AsRef<Path>>(
    command: &str,
    dir: P,
    config: &Config,
) -> Result<(), ExecError> {
    let exe = command.split_whitespace().next().unwrap();
    let args = command.split_whitespace().skip(1).collect::<Vec<&str>>();

    let (stdout, stderr) = if let Some(silent_mode) = config.silent_mode {
        if silent_mode {
            (Stdio::piped(), Stdio::piped())
        } else {
            (Stdio::inherit(), Stdio::inherit())
        }
    } else {
        (Stdio::inherit(), Stdio::inherit())
    };

    if let Ok(command_output) = Command::new(exe)
        .args(args)
        .current_dir(dir.as_ref())
        .stdout(stdout)
        .stderr(stderr)
        .spawn()
    {
        let output = command_output.wait_with_output().unwrap();

        if !output.status.success() {
            return Err(ExecError {
                command: command.to_string(),
                context: config.name.clone(),
                exit_code: output.status.code().unwrap_or(1),
            });
        }

        Ok(())
    } else {
        return Err(ExecError {
            command: command.to_string(),
            context: config.name.clone(),
            exit_code: 1,
        });
    }
}
