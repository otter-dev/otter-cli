use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::blockchains::SolanaTaskCommand;

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub id: String,
    pub repo: String,
    pub branch_or_hash: Option<String>,
    pub blockchain: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct JobStatus {
    pub job_id: String,
    pub job_state: String,
    pub repo_cmd_results: Option<Value>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub job_id: String,
    pub task_id: String,
    pub task_type: String,
    pub task_state: String,
    pub task_result: Option<Value>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct JobRespose {
    pub job: Job,
    pub job_status: JobStatus,
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "blockchain", content = "data", rename_all = "snake_case")]
pub enum TaskCommand {
    Solana(SolanaTaskCommand),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateJobResponse {
    pub job_id: String,
    pub blockchain: String,
    pub builder_cmds: Vec<Value>,
    pub repo_cmds: Vec<Value>,
    pub tasks: Vec<TaskCommand>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskError {
    pub error: String,
}
