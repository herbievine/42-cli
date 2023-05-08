use std::{
    path::Path,
    process::{Command, Stdio},
};

#[derive(Debug)]
pub struct ExecError {
    pub command: String,
    pub trace: String,
    pub exit_code: i32,
}

pub fn exec_command<P: AsRef<Path>>(command: &str, dir: P) -> Result<(), ExecError> {
    let exe = command.split_whitespace().next().unwrap();
    let args = command.split_whitespace().skip(1).collect::<Vec<_>>();

    let command_output = Command::new(exe)
        .args(args)
        .current_dir(dir.as_ref())
        .stdout(Stdio::inherit())
        .spawn()
        .unwrap();

    let output = command_output.wait_with_output().unwrap();

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let message = if !stdout.is_empty() { stdout } else { stderr };

        return Err(ExecError {
            command: command.to_string(),
            trace: message,
            exit_code: output.status.code().unwrap_or(1),
        });
    }

    Ok(())
}
