use std::{
    path::Path,
    process::{Command, Stdio},
};

pub struct CommandError {
    pub command: String,
    pub trace: String,
    pub exit_code: i32,
}

pub fn exec_command<P: AsRef<Path>>(
    command: &str,
    dir: P,
    pipe: Option<&str>,
) -> Result<(), CommandError> {
    let exe = command.split_whitespace().next().unwrap();
    let args = command.split_whitespace().skip(1).collect::<Vec<_>>();

    let stdout = if pipe.is_some() {
        Stdio::piped()
    } else {
        Stdio::inherit()
    };

    let command_output = Command::new(exe)
        .args(args)
        .current_dir(dir.as_ref())
        .stdout(stdout)
        .spawn()
        .unwrap();

    let output = if let Some(pipe) = pipe {
        let pipe_exe = pipe.split_whitespace().next().unwrap();
        let pipe_args = pipe.split_whitespace().skip(1).collect::<Vec<_>>();

        Command::new(pipe_exe)
            .args(pipe_args)
            .current_dir(dir.as_ref())
            .stdin(command_output.stdout.unwrap())
            .stdout(Stdio::inherit())
            .output()
            .unwrap()
    } else {
        command_output.wait_with_output().unwrap()
    };

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let message = if !stdout.is_empty() { stdout } else { stderr };

        return Err(CommandError {
            command: command.to_string(),
            trace: message,
            exit_code: output.status.code().unwrap_or(1),
        });
    }

    Ok(())
}
