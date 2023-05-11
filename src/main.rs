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
    let config = Config::new(".");

    if let Err(error) = match cli.subcommand {
        Commands::Build(_) => config.build(),
        Commands::Clean(_) => config.clean(),
        Commands::Lint(_) => config.lint(),
        Commands::Test(_) => config.test(),
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
