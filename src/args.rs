use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "42 CLI", author, about)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub subcommand: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Pushes your project to 42 vogsphere
    #[clap(name = "push")]
    Push(PushArgs),

    /// Updates your project from 42 vogsphere
    #[clap(name = "update")]
    Update(UpdateArgs),

    /// Test
    #[clap(name = "test")]
    Test(TestArgs),
}

#[derive(Debug, Args)]
pub struct PushArgs {
    /// The project directory
    #[clap(name = "project_directory")]
    pub project_directory: String,

    /// The git repository
    #[clap(name = "git_repository")]
    pub git_repository: String,

    /// Includes only the files matching the pattern
    #[clap(short, long)]
    pub include: Option<String>,

    /// Disables the norminette check
    #[clap(short, long)]
    pub no_norm: bool,
}

#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// The project directory
    #[clap(name = "project_directory")]
    pub project_directory: String,

    /// The git repository
    #[clap(name = "git_repository")]
    pub git_repository: String,

    /// Includes only the files matching the pattern
    #[clap(short, long)]
    pub include: Option<String>,

    /// Disables the norminette check
    #[clap(short, long)]
    pub no_norm: bool,
}

#[derive(Debug, Args)]
pub struct TestArgs {
    // /// Program to Test
    // #[clap()]
    // pub program: String,

    // /// Arguments to pass to the program
    // #[clap()]
    // pub arguments: Vec<String>,

    // /// Test make
    // #[clap(short, long)]
    // pub make: bool,
}
