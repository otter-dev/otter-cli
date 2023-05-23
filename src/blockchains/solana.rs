use serde::{Deserialize, Serialize};


#[derive(Debug, Display, Clone, Serialize, Deserialize)]
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
