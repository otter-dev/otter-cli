use anyhow::Result;
use clap::{CommandFactory, Parser};
use inquire::{error::InquireResult, required, Text};
use serde_json::json;

use crate::{
    blockchains::{
        select_blockchain,
        solana::{get_task_command_list_from_vec, SolanaTaskCommand},
        Blockchain,
    },
    cli::{Commands, CreateTaskCommands, OtrCliArgs},
    client::{
        auth::{authenticate, is_authenticated},
        listen_for_changes,
        models::JobRespose,
        output::{pretty_print_error, pretty_print_output},
        process_create_task, process_get_job,
    },
    endpoints::{select_endpoint, Endpoint},
};

mod blockchains;
mod cli;
mod client;
mod endpoints;

#[tokio::main(flavor = "current_thread")]
async fn main() -> InquireResult<()> {
    tracing_subscriber::fmt::init();
    let cli = OtrCliArgs::parse();

    if cli.interactive {
        interactive_mode().await?;
        Ok(())
    } else {
        clap_mode(cli).await
    }
}

async fn clap_mode(cli: OtrCliArgs) -> InquireResult<()> {
    // If the user is not authenticated, authenticate them
    if !is_authenticated() {
        let _ = authenticate().await;
    }
    // Process the command
    match cli.command {
        Some(Commands::Create(args)) => {
            match args.create_task_commands {
                CreateTaskCommands::Solana(args) => {
                    let chain = Blockchain::Solana;
                    let repo_cmds = chain.select_repo_builder_commands();
                    let task_cmds = get_task_command_list_from_vec(args.tasks);
                    let response =
                        process_create_task(chain, args.remote, args.commit, repo_cmds, task_cmds)
                            .await;
                    match response {
                        Ok(response) => {
                            let job_id = response.job_id;
                            listen_for_changes(&job_id).await;
                        }
                        Err(e) => println!("An unexpected error occurred : {}", e),
                    }
                }
            }
            Ok(())
        }
        Some(Commands::Verify(args)) => {
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
                process_create_task(chain, args.remote, args.commit, repo_cmds, task_cmds).await;
            match response {
                Ok(response) => {
                    let job_id = response.job_id;
                    listen_for_changes(&job_id).await;
                }
                Err(e) => println!("An unexpected error occurred : {}", e),
            }
            Ok(())
        }
        Some(Commands::Get(args)) => {
            let response = process_get_job(&args.id).await;
            match response {
                Ok(response) => handle_job_response(response),
                Err(e) => println!("An unexpected error occurred : {}", e),
            }
            Ok(())
        }
        None => {
            println!("Please provide either a command or use the interactive flag -i.");
            OtrCliArgs::command().print_help()?;
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

async fn interactive_mode() -> InquireResult<()> {
    let task = select_endpoint()?;

    if !is_authenticated() {
        let _ = authenticate().await;
    }

    let res = match task {
        Endpoint::CreateTask => create_tasks().await,
        Endpoint::GetTask => get_task().await,
    };

    if let Err(e) = res {
        tracing::error!("{:?}", e);
    }
    Ok(())
}

async fn create_tasks() -> Result<()> {
    if !is_authenticated() {
        anyhow::bail!("You must authenticate before creating tasks");
    }

    let chain = select_blockchain()?;

    let git_repo = Text::new("Please enter the URL of the Git repository:")
        .with_validator(required!())
        .prompt()?;

    let branch_or_hash = Text::new("Please enter the Git branch or commit hash:")
        .with_validator(required!())
        .prompt()?;

    let repo_cmds = chain.select_repo_builder_commands();
    let task_cmd_list = chain.get_task_command_list();

    let response =
        process_create_task(chain, git_repo, branch_or_hash, repo_cmds, task_cmd_list).await;

    match response {
        Ok(response) => {
            let job_id = response.job_id;
            listen_for_changes(&job_id).await;
        }
        Err(e) => println!("An unexpected error occurred : {}", e),
    }

    Ok(())
}

async fn get_task() -> Result<()> {
    if !is_authenticated() {
        anyhow::bail!("You must authenticate before creating tasks");
    }

    let job_id = Text::new("Please enter the job ID to retrieve the result:")
        .with_validator(required!())
        .prompt()?;

    let response = process_get_job(&job_id).await;

    match response {
        Ok(response) => handle_job_response(response),
        Err(e) => println!("An unexpected error occurred : {}", e),
    }
    Ok(())
}
