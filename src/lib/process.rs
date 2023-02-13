use std::{path::Path, process::Command};

#[derive(Debug)]
pub struct ExecuteCommandError {
    pub message: String,
    pub command: String,
}

pub fn exec_commands<P: AsRef<Path>>(
    commands: Vec<&str>,
    target_dir: P,
) -> Result<(), ExecuteCommandError> {
    for command in commands {
        let exe = command.split_whitespace().next().unwrap();
        let args = command.split_whitespace().skip(1).collect::<Vec<_>>();

        // println!("Running command: {} {}", exe, args.join(" "));

        let output = Command::new(exe)
            .args(args)
            .current_dir(target_dir.as_ref())
            .output()
            .unwrap();

        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let message = if !stdout.is_empty() { stdout } else { stderr };

            // return Err(Error::new(std::io::ErrorKind::Other, message));
            return Err(ExecuteCommandError {
                message,
                command: command.to_string(),
            });
        }
    }

    Ok(())
}
