use clap::{ArgAction, Args, Parser, Subcommand};

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
    /// Run command in children
    #[clap(long, action=ArgAction::SetFalse)]
    pub all: bool,
}

#[derive(Debug, Args)]
pub struct RunArgs {
    /// Run clean command after run
    #[clap(short, long, action=ArgAction::SetFalse)]
    pub clean: bool,
}

#[derive(Debug, Args)]
pub struct BuildArgs {
    /// Run command in children
    #[clap(long, action=ArgAction::SetFalse)]
    pub all: bool,
}

#[derive(Debug, Args)]
pub struct CleanArgs {
    /// Run command in children
    #[clap(long, action=ArgAction::SetFalse)]
    pub all: bool,
}

#[derive(Debug, Args)]
pub struct LintArgs {
    /// Run command in children
    #[clap(long, action=ArgAction::SetFalse)]
    pub all: bool,
}
