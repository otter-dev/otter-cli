use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Blockchain {
    Solana,
    Aptos,
    Ethereum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
