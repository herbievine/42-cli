use regex::Regex;
use std::{fs, io::Result, path::Path};

pub fn merge_directory<P: AsRef<Path>>(
    project_directory: P,
    destination_directory: P,
    override_files: bool,
    pattern: Option<&str>,
) -> Result<()> {
    let regex = Regex::new(pattern.unwrap_or("")).unwrap();

    // fs::create_dir_all(&destination_directory)?;

    for entry in fs::read_dir(project_directory)? {
        let entry = entry?;
        let filetype = entry.file_type()?;

        if filetype.is_dir() {
            merge_directory(
                entry.path(),
                destination_directory.as_ref().join(entry.file_name()),
                override_files,
                pattern,
            )?;
        } else if regex.is_match(entry.file_name().to_str().unwrap()) {
            if override_files
                && destination_directory
                    .as_ref()
                    .join(entry.file_name())
                    .exists()
            {
                fs::remove_file(destination_directory.as_ref().join(entry.file_name()))?;
            }
            fs::copy(
                entry.path(),
                destination_directory.as_ref().join(entry.file_name()),
            )?;
        }
    }

    Ok(())
}
