use serde::{Deserialize, Serialize};
use serde_json::json;

use super::models::TaskError;

#[derive(Serialize, Deserialize)]
struct OverflowCheckResult {
    pub success: bool,
}

#[derive(Serialize, Deserialize)]
struct TaskResult {
    pub success: bool,
    pub output: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct VerifyGithubBuildIsOnchainResult {
    pub success: bool,
    program_id: Option<String>,
    last_deployed_slot: Option<u64>,
}

pub fn pretty_print_output(task_type: &str, output: Option<serde_json::Value>) {
    let output = output.unwrap_or(json!(""));
    match task_type {
        "cargo_build" | "cargo_clippy" | "build_bpf" => {
            let result: TaskResult = serde_json::from_value(output).unwrap();
            if result.success {
                println!("Task {} success!", task_type);
            } else {
                println!("Task {} failed!", task_type)
            }
        }
        "overflow_check" => {
            let result: OverflowCheckResult = serde_json::from_value(output).unwrap();
            println!(
                "Overflow checks are {}",
                if result.success {
                    "enabled"
                } else {
                    "disabled"
                }
            );
        }
        "build_and_return_hash" => {
            let result: TaskResult = serde_json::from_value(output).unwrap();
            if result.success {
                println!("Build success! Hash: {}", result.output.unwrap())
            } else {
                println!("Build failed!")
            }
        }
        "verify_github_build_is_onchain" => {
            let result: VerifyGithubBuildIsOnchainResult = serde_json::from_value(output).unwrap();
            if result.success {
                println!("Given build is onchain!");
                println!(
                    "Program ID: {}, and last deployed at Slot: {}",
                    result.program_id.unwrap(),
                    result.last_deployed_slot.unwrap()
                )
            } else {
                println!("Given build is not onchain!")
            }
        }
        "formal_verification" => {
            let result: TaskResult = serde_json::from_value(output).unwrap();
            if result.success {
                println!("Formal verification succeeded!");
                println!("{}", result.output.unwrap());
            } else {
                println!("Formal verification failed!");
                println!("{}", result.output.unwrap());
            }
        }
        _ => {}
    }
}

pub fn pretty_print_error(task_type: &str, error: Option<serde_json::Value>) {
    let error: TaskError = serde_json::from_value(error.unwrap_or(json!(""))).unwrap();
    println!("Task {} Failed with error: {}", task_type, error.error);
}
