use regex::Regex;
use std::{fs, io::Result, path::Path};

pub fn clone_directory<P: AsRef<Path>>(
    project_directory: P,
    destination_directory: P,
    pattern: Option<&str>,
) -> Result<()> {
    let regex = Regex::new(pattern.unwrap_or("")).unwrap();

    fs::create_dir_all(&destination_directory)?;

    for entry in fs::read_dir(project_directory)? {
        let entry = entry?;
        let filetype = entry.file_type()?;

        if filetype.is_dir() {
            clone_directory(
                entry.path(),
                destination_directory.as_ref().join(entry.file_name()),
                pattern,
            )?;
        } else if regex.is_match(entry.file_name().to_str().unwrap()) {
            fs::copy(
                entry.path(),
                destination_directory.as_ref().join(entry.file_name()),
            )?;
        }
    }

    Ok(())
}
