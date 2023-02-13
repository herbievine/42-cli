use clap::ArgMatches;
use dirs::home_dir;
use regex::{NoExpand, Regex};
use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

pub fn extract_and_check_git_repository(matches: &ArgMatches) -> Result<String, String> {
    let git_repository = matches.get_one::<String>("git_repository").unwrap();
    // let re = Regex::new(r"^git@vogsphere\.42paris\.fr:vogsphere/([\w\d-]{57,})$").unwrap();
    let re = Regex::new(r"").unwrap();

    if re.is_match(git_repository) {
        Ok(git_repository.to_string())
    } else {
        Err("Invalid git repository".to_string())
    }
}

pub fn extract_and_check_project_directory(matches: &ArgMatches) -> Result<&Path, String> {
    let project_directory = matches.get_one::<String>("project_directory").unwrap();
    let path = Path::new(project_directory);

    if !path.exists() {
        return Err(format!(
            "The project directory does not exist: {}",
            project_directory
        ));
    }

    if !path.is_dir() {
        return Err(format!(
            "The project directory is not a directory: {}",
            project_directory
        ));
    }

    Ok(path)
}

pub fn check_project_destination(project_name: &str) -> Result<PathBuf, String> {
    let destination = home_dir().unwrap().join("/tmp/42-cli").join(project_name);

    if destination.exists() {
        fs::remove_dir_all(&destination).unwrap();
    }

    Ok(destination)
}

pub fn run_commands(
    raw_commands: Vec<&str>,
    directory: Option<impl AsRef<Path>>,
) -> Result<(), String> {
    for command in raw_commands {
        println!("Running command: {}", command);

        let exe = command.split_whitespace().next().unwrap();
        let args = command.split_whitespace().skip(1).collect::<Vec<_>>();

        let out = Command::new(exe)
            .args(args)
            .current_dir(directory.as_ref().unwrap())
            .output()
            .expect("failed to execute process");

        println!("stdout: {}", String::from_utf8_lossy(&out.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&out.stderr));
    }

    // let commands: Vec<String> = raw_commands
    //     .iter()
    //     .map(|&git_command| {
    //         let mut command = String::from(git_command);
    //         if let Some(directory) = &directory {
    //             command = format!("cd {} && {}", directory.as_ref().display(), "ls -l -a");
    //         }
    //         command
    //     })
    //     .collect();

    // for command in commands {
    //     println!("Running command: {}", command);

    //     let exe = command.split_whitespace().next().unwrap();
    //     let args = command.split_whitespace().skip(1).collect::<Vec<_>>();

    //     let out = Command::new(exe)
    //         .args(args)
    //         .output()
    //         .expect("failed to execute process");

    //     println!("stdout: {}", String::from_utf8_lossy(&out.stdout));
    //     println!("stderr: {}", String::from_utf8_lossy(&out.stderr));
    // }

    Ok(())
}

pub fn clone_directory(
    project_directory: impl AsRef<Path>,
    destination_directory: impl AsRef<Path>,
) -> io::Result<()> {
    fs::create_dir_all(&destination_directory)?;
    for entry in fs::read_dir(project_directory)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            clone_directory(
                entry.path(),
                destination_directory.as_ref().join(entry.file_name()),
            )?;
        } else {
            fs::copy(
                entry.path(),
                destination_directory.as_ref().join(entry.file_name()),
            )?;
        }
    }
    Ok(())
}

pub fn push_to_repository(directory: impl AsRef<Path>, git_repository: &str) -> Result<(), String> {
    let remote = format!("git remote add origin {}", git_repository);
    let commands = vec![
        "git init",
        "git add .",
        "git commit -am.",
        &remote,
        "git push -u origin main",
    ];

    run_commands(commands, Some(directory))?;

    Ok(())
}
