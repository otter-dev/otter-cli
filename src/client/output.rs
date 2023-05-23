use serde_json::json;

use crate::client::models::TaskResult;

use super::models::TaskError;

pub fn pretty_print_output(task_type: &str, output: Option<serde_json::Value>) {
    let output = output.unwrap_or(json!(""));
    #[allow(clippy::single_match)]
    match task_type {
        "formal_verification" => {
            let result: TaskResult = serde_json::from_value(output).unwrap();
            if result.success {
                println!("Formal verification succeeded!");
            } else {
                println!("Formal verification failed!");
            }
            println!("{}", result.output.unwrap());
        }
        _ => {}
    }
}

pub fn pretty_print_error(task_type: &str, error: Option<serde_json::Value>) {
    let error: TaskError = serde_json::from_value(error.unwrap_or(json!(""))).unwrap();
    println!("Task {} Failed with error: {}", task_type, error.error);
}
