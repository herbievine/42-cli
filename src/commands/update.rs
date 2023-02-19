use dirs::home_dir;

use crate::args::UpdateArgs;
use crate::error::display_error;
use crate::lib;
use std::fs;
use std::path::PathBuf;

pub fn exec(args: &UpdateArgs) {
    let UpdateArgs {
        git_repository,
        project_directory,
        include,
        no_norm,
    } = args;

    let project_directory = PathBuf::from(project_directory);
    let project_name = project_directory.file_name().unwrap().to_str().unwrap();
    let destination_directory = home_dir().unwrap().join("42-cli/tmp").join(project_name);

    match fs::create_dir_all(&destination_directory) {
        Ok(_) => println!("Successfully created temporary directory!"),
        Err(e) => display_error(e.to_string(), "fs::create_dir_all".to_string(), true),
    }

    match lib::git::clone(destination_directory.clone(), git_repository) {
        Ok(_) => println!("Successfully cloned project!"),
        Err(e) => {
            display_error(e.message, e.command, false);
            match fs::remove_dir_all(destination_directory.clone()) {
                Ok(_) => {
                    println!("Removed all temporary files!");
                    std::process::exit(1);
                }
                Err(e) => display_error(e.to_string(), "fs::remove_dir_all".to_string(), true),
            }
        }
    }

    match lib::fs::merge_directory(
        project_directory,
        destination_directory.clone(),
        true,
        include.as_ref().map(|s| s.as_str()),
    ) {
        Ok(_) => println!("Successfully copied directory!"),
        Err(e) => {
            display_error(e.to_string(), "lib::fs::merge_directory".to_string(), false);
            match fs::remove_dir_all(destination_directory.clone()) {
                Ok(_) => {
                    println!("Removed all temporary files!");
                    std::process::exit(1);
                }
                Err(e) => {
                    display_error(e.to_string(), "lib::fs::merge_directory".to_string(), true)
                }
            }
        }
    }

    if !no_norm {
        match lib::misc::norm(destination_directory.clone()) {
            Ok(_) => println!("Norminette passed!"),
            Err(e) => {
                display_error(e.message, e.command, false);
                match fs::remove_dir_all(destination_directory.clone()) {
                    Ok(_) => {
                        println!("Removed all temporary files!");
                        std::process::exit(1);
                    }
                    Err(e) => display_error(e.to_string(), "fs::remove_dir_all".to_string(), true),
                }
            }
        }
    }

    match lib::git::add(destination_directory.clone()) {
        Ok(_) => println!("Successfully added files!"),
        Err(e) => {
            display_error(e.message, e.command, false);
            match fs::remove_dir_all(destination_directory.clone()) {
                Ok(_) => {
                    println!("Removed all temporary files!");
                    std::process::exit(1);
                }
                Err(e) => display_error(e.to_string(), "fs::remove_dir_all".to_string(), true),
            }
        }
    }

    match lib::git::commit(destination_directory.clone()) {
        Ok(_) => println!("Successfully committed files!"),
        Err(e) => {
            display_error(e.message, e.command, false);
            match fs::remove_dir_all(destination_directory.clone()) {
                Ok(_) => {
                    println!("Removed all temporary files!");
                    std::process::exit(1);
                }
                Err(e) => display_error(e.to_string(), "fs::remove_dir_all".to_string(), true),
            }
        }
    }

    match lib::git::push(destination_directory.clone()) {
        Ok(_) => println!("Successfully pushed files!"),
        Err(e) => {
            display_error(e.message, e.command, false);
            match fs::remove_dir_all(destination_directory.clone()) {
                Ok(_) => {
                    println!("Removed all temporary files!");
                    std::process::exit(1);
                }
                Err(e) => display_error(e.to_string(), "fs::remove_dir_all".to_string(), true),
            }
        }
    }

    match fs::remove_dir_all(destination_directory.clone()) {
        Ok(_) => println!("Removed all temporary files!"),
        Err(e) => display_error(e.to_string(), "fs::remove_dir_all".to_string(), true),
    }
}
