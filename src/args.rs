use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "42 CLI", version, author, about)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub subcommand: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Test your project
    #[clap(name = "test")]
    Test(TestArgs),

    /// Run your project
    #[clap(name = "run")]
    Run(RunArgs),

    /// Compile your project
    #[clap(name = "build")]
    Build(BuildArgs),

    /// Clean your project
    #[clap(name = "clean")]
    Clean(CleanArgs),

    /// Lint your project
    #[clap(name = "lint")]
    Lint(LintArgs),
}

#[derive(Debug, Args)]
pub struct TestArgs {
    /// Silent mode
    #[clap(short, long)]
    pub silent: bool,
}

#[derive(Debug, Args)]
pub struct RunArgs {
    /// Silent mode
    #[clap(short, long)]
    pub silent: bool,

    /// Run clean command after run
    #[clap(short, long)]
    pub clean: bool,
}

#[derive(Debug, Args)]
pub struct BuildArgs {
    /// Silent mode
    #[clap(short, long)]
    pub silent: bool,
}

#[derive(Debug, Args)]
pub struct CleanArgs {
    /// Silent mode
    #[clap(short, long)]
    pub silent: bool,
}

#[derive(Debug, Args)]
pub struct LintArgs {
    /// Silent mode
    #[clap(short, long)]
    pub silent: bool,
}
