mod args;
mod error;
mod setup;
mod commands {
    pub mod push;
    pub mod update;
}
mod lib {
    pub mod fs;
    pub mod git;
    pub mod process;
}

use args::{CliArgs, Commands};
use clap::Parser;
use commands::{push, update};

fn main() {
    setup::setup();

    let cli = CliArgs::parse();

    match cli.subcommand {
        Commands::Push(args) => push::exec(&args),
        Commands::Update(args) => update::exec(&args),
    }
}
