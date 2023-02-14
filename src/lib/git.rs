use super::process::{exec_commands, ExecuteCommandError};
use std::path::Path;

pub fn init<P: AsRef<Path>>(target_dir: P) -> Result<(), ExecuteCommandError> {
    exec_commands(vec!["git init"], target_dir)
}

pub fn add<P: AsRef<Path>>(target_dir: P) -> Result<(), ExecuteCommandError> {
    exec_commands(vec!["git add ."], target_dir)
}

pub fn commit<P: AsRef<Path>>(target_dir: P) -> Result<(), ExecuteCommandError> {
    exec_commands(vec!["git commit -m \"42-cli\""], target_dir)
}

pub fn push<P: AsRef<Path>>(target_dir: P) -> Result<(), ExecuteCommandError> {
    exec_commands(vec!["git push -u origin main"], target_dir)
}

pub fn clone<P: AsRef<Path>>(target_dir: P, remote: &str) -> Result<(), ExecuteCommandError> {
    let command = format!("git clone {}", remote);
    exec_commands(vec![&command], target_dir)
}

pub fn add_remote<P: AsRef<Path>>(target_dir: P, remote: &str) -> Result<(), ExecuteCommandError> {
    let command = format!("git remote add origin {}", remote);
    exec_commands(vec![&command], target_dir)
}
