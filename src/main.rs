mod args;
mod commands {
    pub mod build;
    pub mod clean;
    pub mod lint;
    pub mod run;
    pub mod test;
}
mod lib {
    pub mod actions;
    pub mod config;
    pub mod process;
}

use args::{CliArgs, Commands};
use clap::Parser;
use colored::Colorize;
use commands::{build, clean, lint, run, test};

fn main() {
    let cli = CliArgs::parse();
    let config = lib::config::get_config();

    if let Err(error) = match cli.subcommand {
        Commands::Build(args) => build::exec(&args, &config),
        Commands::Clean(args) => clean::exec(&args, &config),
        Commands::Lint(args) => lint::exec(&args, &config),
        Commands::Test(args) => test::exec(&args, &config),
        Commands::Run(args) => run::exec(&args, &config),
    } {
        println!(
            "{} {}",
            "Error in:".bright_red().bold(),
            config.name.bright_red().bold()
        );
        println!("{} {}", "Error running:".red().bold(), error.command.red());
        println!("{} {}", "Trace:".red().bold(), error.trace.red());
        std::process::exit(error.exit_code);
    }

    println!("{}", "Done!".bright_green().bold());
}
