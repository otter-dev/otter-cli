use clap::{Args, Parser, Subcommand};

use crate::blockchains::solana::SolanaTaskCommand;

/// CLI tool for Otter Suite API
#[derive(Debug, Parser)]
#[clap(name = "Otter Suite")]
#[clap(about = "CLI tool for Otter Suite API", long_about = None)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub command: Option<Commands>,
    /// Use the interactive mode
    #[clap(short = 'i', long = "interactive")]
    pub interactive: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create a job to run given tasks on the repo
    #[clap(arg_required_else_help = true)]
    Create(CreateJob),
    /// Get the status of a job by ID
    #[clap(arg_required_else_help = true)]
    Get(GetJob),
    /// Submit formal verification task to the Otter Suite
    #[clap(arg_required_else_help = true)]
    Verify(VerifyTaskSolanaArgs),
}

#[derive(Debug, Parser)]
pub struct CreateJob {
    #[structopt(subcommand)]
    pub create_task_commands: CreateTaskCommands,
}

// The Possible subcommands for the create task command
#[derive(Debug, Subcommand)]
pub enum CreateTaskCommands {
    /// Solana project commands
    Solana(CreateTaskSolanaArgs),
}

#[derive(Debug, Args)]
pub struct GetJob {
    /// Job ID to get
    #[clap(long, short)]
    pub id: String,
}

// Create args for the Solana projects
#[derive(Debug, Args)]
pub struct CreateTaskSolanaArgs {
    /// Overwrite an existing project
    #[clap(long, short)]
    pub remote: String,
    /// Commit of the project to use
    #[clap(long, short)]
    pub commit: String,
    /// Task commands to run
    #[clap(
        long,
        short,
        required = true,
        value_parser,
        use_value_delimiter = true,
        value_delimiter = ','
    )]
    pub tasks: Vec<SolanaTaskCommand>,
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
