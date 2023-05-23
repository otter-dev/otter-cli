use serde::{Deserialize, Serialize};
use serde_json::json;
use strum::Display;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Blockchain {
    Solana,
    Aptos,
    Ethereum,
}

pub trait BlockchainTaskCommand {
    fn get_blockchain() -> Blockchain;
}

#[derive(Debug, Display, clap::ValueEnum, Clone, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SolanaTaskCommand {
    FormalVerification,
}

impl BlockchainTaskCommand for SolanaTaskCommand {
    fn get_blockchain() -> Blockchain {
        Blockchain::Solana
    }
}

pub fn get_task_command_list_from_vec<T>(tasks: Vec<T>) -> Vec<serde_json::Value>
where
    T: BlockchainTaskCommand + Serialize,
{
    let blockchain_name = T::get_blockchain().to_string();
    tasks
        .into_iter()
        .map(|s| json!({"blockchain": blockchain_name, "data": s}))
        .collect()
}
