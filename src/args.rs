use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "42 CLI", author, about)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub subcommand: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Test your project
    #[clap(name = "test")]
    Test(TestArgs),
}

#[derive(Debug, Args)]
pub struct TestArgs {}
