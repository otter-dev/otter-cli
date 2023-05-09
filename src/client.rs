use anyhow::{Context, Result};
use otter_auth_client::get_config;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;

use crate::blockchains::Blockchain;

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
) -> Result<serde_json::Value> {
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
        .text()
        .await
        .context("error reading response")?;

    tracing::debug!("{:?}", &res);
    serde_json::from_str(&res).context("error converting to json")
}

pub async fn process_get_job(job_id: String) -> Result<serde_json::Value> {
    let client = create_client()?;
    client
        .get(format!("{API_URL}/job?id={}", job_id))
        .send()
        .await
        .context("Error sending request")?
        .json()
        .await
        .context("Error getting response body")
}
