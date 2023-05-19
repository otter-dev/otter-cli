use serde::{Deserialize, Serialize};
use serde_json::json;

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

pub fn print_pretty_output(task_type: &str, output: Option<serde_json::Value>) {
    let output = output.unwrap_or(json!(""));
    match task_type {
        "cargo_build" | "cargo_clippy" | "build_bpf" => {
            let result: TaskResult = serde_json::from_value(output).unwrap();
            if result.success {
                println!("Task succeeded!");
                println!("{:#?}", result.output.unwrap());
            } else {
                println!("Task failed!")
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
                println!("Build succeeded! Hash: {}", result.output.unwrap())
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
