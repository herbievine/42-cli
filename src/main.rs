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
use commands::test;

fn main() {
    let cli = CliArgs::parse();
    let config = lib::config::get_config();

    match cli.subcommand {
        Commands::Push(_) => (),
        Commands::Test(args) => test::exec(&args, &config),
        Commands::Update(_) => (),
    }
}
