use crate::lib;
use clap::ArgMatches;
use std::fs;

pub fn exec(matches: &ArgMatches) {
    let project_directory = match lib::extract_and_check_project_directory(matches) {
        Ok(path) => path,
        Err(err) => panic!("{}", err),
    };
    let git_repository = match lib::extract_and_check_git_repository(matches) {
        Ok(url) => url,
        Err(err) => panic!("{}", err),
    };
    let project_name = project_directory.file_name().unwrap().to_str().unwrap();
    // let _include = matches.get_one::<String>("include");
    // let _no_norm = matches.get_one::<bool>("no-norm").unwrap_or(&false);

    let destination_directory = match lib::check_project_destination(project_name) {
        Ok(pathbuf) => pathbuf,
        Err(err) => panic!("{}", err),
    };

    match lib::clone_directory(project_directory.clone(), destination_directory.clone()) {
        Ok(_) => println!(
            "Successfully copied your code to {}",
            destination_directory.display()
        ),
        Err(err) => println!("{}", err),
    }

    match lib::push_to_repository(destination_directory, &git_repository) {
        Ok(_) => println!("Successfully pushed your code to {}", git_repository),
        Err(err) => println!("{}", err),
    }
}
