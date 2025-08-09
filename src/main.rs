mod common;
mod rag_builder;
mod rig_agent;

use common::Config;
use rig::providers::anthropic;
use rig_agent::RigAgent;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    RigAgent::new(Config::new(), anthropic::CLAUDE_3_5_SONNET)
        .await?
        .start_repl()
        .await
}
