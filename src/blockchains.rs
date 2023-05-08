use std::fmt::Display;

use inquire::{
    error::InquireResult, list_option::ListOption, validator::Validation, MultiSelect, Select, Text,
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

pub mod solana;

#[derive(Debug, Clone, Copy, EnumIter, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum Blockchain {
    Solana,
    Aptos,
    Ethereum,
}

pub(crate) fn select_blockchain() -> InquireResult<Blockchain> {
    Select::new("Blockchain:", Blockchain::iter().collect()).prompt()
}

fn collect_commands<C>(cmds: Vec<Box<dyn FnOnce() -> Option<C>>>) -> Vec<C> {
    cmds.into_iter().filter_map(|c| c()).collect()
}

impl Blockchain {
    pub fn select_repo_builder_commands(&self) -> Vec<serde_json::Value> {
        match self {
            Blockchain::Solana => collect_commands(solana::generate_repo_commands()),
            Blockchain::Aptos => todo!(),
            Blockchain::Ethereum => todo!(),
        }
    }

    pub fn get_task_command_list(&self) -> Vec<serde_json::Value> {
        match self {
            Blockchain::Solana => solana::get_task_command_list(),
            Blockchain::Aptos => todo!(),
            Blockchain::Ethereum => todo!(),
        }
    }
}

fn prompt_for_field(name: &str) -> Option<String> {
    Text::new(&format!("Enter {}", name))
        .with_help_message("If none press enter")
        .prompt_skippable()
        .unwrap()
        .filter(|i| !i.is_empty())
}

fn prompt_for_multiselect<T: Display>(text: &str, options: Vec<T>) -> InquireResult<Vec<T>> {
    let validator = &|a: &[ListOption<&T>]| {
        if a.is_empty() {
            Ok(Validation::Invalid(
                "Please select at least one task.".into(),
            ))
        } else {
            Ok(Validation::Valid)
        }
    };

    let selected_tasks = MultiSelect::new(text, options)
        .with_validator(validator)
        .prompt();
    selected_tasks
}
