use clap::{Args, Parser, Subcommand};

/// CLI tool for Otter Suite API
#[derive(Debug, Parser)]
#[clap(name = "Otter Suite")]
#[clap(about = "CLI tool for Otter Suite API", long_about = None)]
pub struct OtrCliArgs {
    #[clap(subcommand)]
    pub command: Option<Commands>,
    /// Use the interactive mode
    #[clap(short = 'i', long = "interactive")]
    pub interactive: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Submit formal verification task to the Otter Suite
    #[clap(arg_required_else_help = true)]
    Verify(VerifyTaskSolanaArgs),
}

// Create args for the Solana projects
#[derive(Debug, Args)]
pub struct VerifyTaskSolanaArgs {
    /// Overwrite an existing project
    #[clap(long, short)]
    pub remote: String,
    /// Commit of the project to use
    #[clap(long, short)]
    pub commit: String,
    /// Program path
    #[clap(long, short)]
    pub path: String,
}
