use std::io::Read;

use anyhow::Result;

use client::process_get_job;
use endpoints::Endpoint;
use inquire::{error::InquireResult, required, InquireError, Text};

use crate::{
    blockchains::select_blockchain, client::process_create_task, endpoints::select_endpoint,
};

mod blockchains;
mod client;
mod endpoints;

const CLIENT_ID: &str = "Iv1.4de4d4a1d7ba2f81";

#[tokio::main(flavor = "current_thread")]
async fn main() -> InquireResult<()> {
    tracing_subscriber::fmt::init();
    let task = select_endpoint()?;

    let res = match task {
        Endpoint::Authenticate => authenticate().await,
        Endpoint::CreateTask => create_tasks().await,
        Endpoint::GetTask => get_task().await,
    };

    if let Err(e) = res {
        tracing::error!("{:?}", e);
    }

    Ok(())
}

fn is_authenticated() -> bool {
    otter_auth_client::get_config().is_ok()
}

async fn authenticate() -> Result<()> {
    let auth = otter_auth_client::get_github_auth_code(CLIENT_ID)
        .await
        .map_err(|e| InquireError::Custom(Box::new(e)))?;

    println!("Please go to the following url: {}", &auth.verification_uri);
    println!("Enter the following code: {}", &auth.user_code);
    println!("Press enter when you have completed authentication");
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
    otter_auth_client::save_config(&auth).map_err(|e| InquireError::Custom(Box::new(e)))?;
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

    let response = process_get_job(job_id).await;

    match response {
        Ok(response) => {
            println!("{:#?}", response);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
