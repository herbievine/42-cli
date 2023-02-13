mod lib;
mod push;
use clap::{arg, Command};

fn main() {
    let matches = Command::new("42-cli")
        .version("0.1.0")
        .author("Herbie V. <me@herbievine.com>")
        .about("A command line tool for 42")
        .subcommand(
            Command::new("push")
                .about("Pushes your project to 42 Vogsphere")
                .args(&[
                    arg!(<project_directory> "the directory of your project"),
                    arg!(<git_repository> "the git repository of your project"),
                    arg!(-f --force "force push"),
                    arg!(-i --include <pattern> "a pattern to only include matching files"),
                    arg!(-n --"no-norm" "do not run norminette"),
                ]),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("push", push_matches)) => push::exec(push_matches),
        _ => println!("No subcommand was used"),
    }
}
