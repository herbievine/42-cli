mod args;
mod commands {
    pub mod test;
}
mod lib {
    pub mod config;
    pub mod process;
}

use args::{CliArgs, Commands};
use clap::Parser;
use colored::Colorize;
use commands::test;

fn main() {
    let cli = CliArgs::parse();
    let config = lib::config::get_config();

    let error = match cli.subcommand {
        Commands::Test(args) => test::exec(&args, &config),
    };

    if error.exit_code != 0 {
        println!("{} {}", "Error running: ".red().bold(), error.command.red());
        println!("{} {}", "Trace: ".red().bold(), error.trace.red());
        std::process::exit(error.exit_code);
    } else {
        println!("{}", "Done!".bright_green().bold());
    }
}
