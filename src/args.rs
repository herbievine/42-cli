use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "42 CLI", author, about)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub subcommand: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(name = "push", about = "Pushes your project to 42 vogsphere")]
    Push(PushArgs),
}

#[derive(Debug, Args)]
pub struct PushArgs {
    /// The project directory
    #[clap(name = "project_directory")]
    pub project_directory: String,

    /// The git repository
    #[clap(name = "git_repository")]
    pub git_repository: String,

    /// Include only the files which match the given regex
    #[clap(short, long)]
    pub include: Option<String>,

    /// Do not run norminette - be careful :)
    #[clap(short, long)]
    pub no_norm: bool,
}
