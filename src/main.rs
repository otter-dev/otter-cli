use anyhow::Result;
use clap::Args;
use clap::Parser;
use serde_json::json;

use crate::{
    blockchains::{get_task_command_list_from_vec, Blockchain, SolanaTaskCommand},
    client::{
        auth::{authenticate, is_authenticated},
        listen_for_changes,
        models::JobRespose,
        output::{pretty_print_error, pretty_print_output},
        process_create_task, process_get_job,
    },
};

mod blockchains;
mod client;

/// CLI tool for Otter Suite API
#[derive(Debug, Parser)]
pub enum Commands {
    /// Get the status of a job by ID
    Check(CheckJob),
    /// Submit formal verification job to the Otter Suite
    #[clap(arg_required_else_help = true)]
    SolanaVerify(SolanaVerifyArgs),
}

#[derive(Debug, Args)]
pub struct CheckJob {
    /// Job ID to check
    pub id: String,
}

// Create args for the Solana projects
#[derive(Debug, Args)]
pub struct SolanaVerifyArgs {
    /// Project git repository
    #[clap(long, short)]
    pub repository: String,
    /// Commit or branch of the project to use
    #[clap(long, short)]
    pub branch: String,
    /// Program path
    #[clap(long, short)]
    pub path: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let command = Commands::parse();

    // If the user is not authenticated, authenticate them
    if !is_authenticated() {
        let _ = authenticate().await;
    }
    // Process the command
    match command {
        Commands::SolanaVerify(args) => {
            let chain = Blockchain::Solana;
            let repo_cmds = vec![json!({
                "blockchain": "solana",
                "data": {
                    "set_program_path": {
                        "path": args.path
                    }
                }
            })];
            let task_cmds =
                get_task_command_list_from_vec(vec![SolanaTaskCommand::FormalVerification]);
            let response =
                process_create_task(chain, args.repository, args.branch, repo_cmds, task_cmds).await;
            match response {
                Ok(response) => {
                    let job_id = response.job_id;
                    listen_for_changes(&job_id).await;
                }
                Err(e) => println!("An unexpected error occurred : {}", e),
            }
            Ok(())
        }
        Commands::Check(args) => {
            let response = process_get_job(&args.id).await;
            match response {
                Ok(response) => handle_job_response(response),
                Err(e) => println!("An unexpected error occurred : {}", e),
            }
            Ok(())
        }
    }
}

fn handle_job_response(response: JobRespose) {
    if response.job_status.job_state == "pending" {
        println!("Your job is still in queue to be processed.");
        return;
    }

    for task in response.tasks {
        if task.task_state == "pending" {
            println!("Your task is still in queue to be processed.");
        } else if task.task_state == "running" {
            println!(
                "Your task is being processed. Please wait a moment while we complete the task."
            );
        } else if task.task_state == "failure" {
            pretty_print_error(&task.task_type, task.task_result);
        } else {
            pretty_print_output(&task.task_type, task.task_result);
        }
    }
}
