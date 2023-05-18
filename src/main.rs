use std::println;

use anyhow::Result;

use blockchains::solana::SolanaTaskCommand;
use clap::{CommandFactory, Parser};
use clap_args::{CliArgs, Commands, CreateTaskCommands};
use client::process_get_job;
use endpoints::Endpoint;
use inquire::{error::InquireResult, required, Text};
use serde_json::json;

use crate::{
    auth::{authenticate, is_authenticated},
    blockchains::solana::get_task_command_list_from_vec,
    blockchains::{select_blockchain, Blockchain},
    client::listen_for_changes,
    client::process_create_task,
    endpoints::select_endpoint,
};

mod auth;
mod blockchains;
mod clap_args;
mod client;
mod endpoints;
mod models;

#[tokio::main(flavor = "current_thread")]
async fn main() -> InquireResult<()> {
    tracing_subscriber::fmt::init();
    let cli = CliArgs::parse();

    if cli.interactive {
        interactive_mode().await?;
        Ok(())
    } else {
        clap_mode(cli).await
    }
}

async fn clap_mode(cli: CliArgs) -> InquireResult<()> {
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
                        Err(e) => println!("Error : {}", e),
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
                Err(e) => println!("Error : {}", e),
            }
            Ok(())
        }
        Some(Commands::Get(args)) => {
            let response = process_get_job(&args.id).await;

            match response {
                Ok(response) => {
                    println!("Job : {:#?}", response);
                }
                Err(e) => println!("Error : {}", e),
            }

            Ok(())
        }
        None => {
            println!("Please provide either a command or use the interactive flag.");
            CliArgs::command().print_help()?;
            Ok(())
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

    let git_repo = Text::new("Enter git repo url:")
        .with_validator(required!())
        .prompt()?;

    let branch_or_hash = Text::new("Enter git branch or commit hash:")
        .with_validator(required!())
        .prompt()?;

    let repo_cmds = chain.select_repo_builder_commands();
    let task_cmd_list = chain.get_task_command_list();

    let response =
        process_create_task(chain, git_repo, branch_or_hash, repo_cmds, task_cmd_list).await;

    match response {
        Ok(response) => {
            println!("Job created : {:#?}", response);
        }
        Err(e) => println!("Error : {}", e),
    }

    Ok(())
}

async fn get_task() -> Result<()> {
    if !is_authenticated() {
        anyhow::bail!("You must authenticate before creating tasks");
    }

    let job_id = Text::new("Enter job id:")
        .with_validator(required!())
        .prompt()?;

    let response = process_get_job(&job_id).await;

    match response {
        Ok(response) => {
            println!("{:#?}", response);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
