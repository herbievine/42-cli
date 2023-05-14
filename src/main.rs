mod args;
mod lib {
    pub mod actions;
    pub mod config;
    pub mod process;
}

use args::{CliArgs, Commands};
use clap::Parser;
use colored::Colorize;

use crate::lib::config::Config;

fn main() {
    let cli = CliArgs::parse();
    let mut config = Config::new(".");

    if let Err(error) = match cli.subcommand {
        Commands::Build(args) => config.build(&args),
        Commands::Clean(args) => config.clean(&args),
        Commands::Lint(args) => config.lint(&args),
        Commands::Test(args) => config.test(&args),
        Commands::Run(args) => config.run(&args),
    } {
        println!(
            "{} {}",
            "Error in:".bright_red().bold(),
            config.name.bright_red().bold()
        );
        println!("{} {}", "Error running:".red().bold(), error.command.red());
        std::process::exit(error.exit_code);
    }

    println!("{}", "Done!".bright_green().bold());
}
