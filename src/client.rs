use std::println;

use anyhow::{Context, Result};
use otter_auth_client::get_config;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;
use tokio::time::{sleep, Duration};

use crate::blockchains::Blockchain;

use self::{
    models::{CreateJobResponse, JobRespose},
    output::print_pretty_output,
};

pub mod auth;
pub mod models;
pub mod output;

const API_URL: &str = "https://api.osec.io";

fn create_client() -> Result<reqwest::Client> {
    let config = get_config()?;
    let mut headers = HeaderMap::new();
    headers.insert(
        "Github-Device-Id",
        HeaderValue::from_str(&config.device_code)?,
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    Ok(client)
}

pub async fn process_create_task(
    chain: Blockchain,
    git_repo: String,
    branch_or_hash: String,
    repo_cmds: Vec<serde_json::Value>,
    task_cmds: Vec<serde_json::Value>,
) -> Result<CreateJobResponse> {
    let client = create_client()?;
    let payload = json!({
        "repo": git_repo,
        "branch_or_hash": branch_or_hash,
        "blockchain": chain,
        "repo_cmds": repo_cmds,
        "tasks": task_cmds
    });

    let res = client
        .post(format!("{API_URL}/job"))
        .json(&payload)
        .send()
        .await
        .context("request error")?
        .json::<CreateJobResponse>()
        .await
        .context("error reading response..")?;

    tracing::debug!("{:?}", &res);
    Ok(res)
}

pub async fn process_get_job(job_id: &str) -> Result<JobRespose> {
    let client = create_client()?;
    client
        .get(format!("{API_URL}/job?id={}", job_id))
        .send()
        .await
        .context("Error sending request")?
        .json::<JobRespose>()
        .await
        .context("Error getting response body")
}

pub async fn listen_for_changes(job_id: &str) {
    println!("Job created with ID: {}", job_id);
    println!(
        "Alternatively, you can close this window and run `otr get -i {}` to view the result later",
        job_id
    );
    println!("Waiting for job to process...");
    loop {
        let response = process_get_job(job_id).await.unwrap();
        if response.job_status.job_state == "success" {
            println!("Job completed!");
            for task in response.tasks {
                print_pretty_output(&task.task_type, task.task_result);
            }
            break;
        } else if response.job_status.job_state == "failure" {
            println!("Job failed!");
            break;
        }
        sleep(Duration::from_secs(2)).await;
    }
}
