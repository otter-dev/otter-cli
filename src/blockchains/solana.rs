use serde::{Deserialize, Serialize};
use serde_json::json;
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::blockchains::{prompt_for_field, prompt_for_multiselect};

pub fn generate_repo_commands() -> Vec<Box<dyn FnOnce() -> Option<serde_json::Value>>> {
    let set_program_id = Box::new(|| {
        prompt_for_field("program id").map(|program_id| {
            json!({
                "set_program_id": {
                    "program_id": program_id
                }
            })
        })
    });

    let set_program_path = Box::new(|| {
        prompt_for_field("program id").map(|program_path| {
            json!({
                "set_program_path": {
                    "path": program_path
                }
            })
        })
    });

    let set_solana_version = Box::new(|| {
        prompt_for_field("solana version").map(|version| {
            json!({
                "set_solana_version": {
                    "version": version
                }
            })
        })
    });

    vec![set_program_id, set_program_path, set_solana_version]
}

#[derive(Display, EnumIter, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SolanaTaskCommand {
    CargoBuild,
    CargoClippy,
    OverflowCheck,
    BuildBpf,
    VerifyGithubBuildIsOnchain,
    BuildAndReturnHash,
    FormalVerification,
}

pub fn get_task_command_list() -> Vec<String> {
    let tasks = SolanaTaskCommand::iter().map(|c| serde_json::to_string(&c).unwrap()).collect();
    let ans = prompt_for_multiselect("Select tasks: ", tasks);
    ans.unwrap()
}
