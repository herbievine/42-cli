use super::process::{exec_commands, ExecuteCommandError};
use std::path::Path;

pub fn norm<P: AsRef<Path>>(destination_directory: P) -> Result<(), ExecuteCommandError> {
    exec_commands(
        vec!["norminette -R CheckForbiddenSourceHeader"],
        destination_directory.as_ref(),
    )
}
