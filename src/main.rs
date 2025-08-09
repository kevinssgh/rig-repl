//! Application entry point.
//!
//! - Initializes structured logging via `tracing_subscriber`, reading level filters
//!   from the `RUST_LOG` environment variable.
//! - Constructs a [`RigAgent`] with configuration from [`common::Config`] and
//!   the Anthropic Claude 3.5 Sonnet model.
//! - Starts an interactive REPL session for processing user input.
//!
//! Modules:
//! - `common`: Shared configuration and utilities.
//! - `rag_builder`: RAG (retrieval-augmented generation) data preparation.
//! - `rig_agent`: Agent logic for handling REPL interactions.


mod common;
mod rag_builder;
mod rig_agent;
mod rag_middleware;

use common::Config;
use rig::providers::anthropic;
use tracing_subscriber::EnvFilter;
use rig_agent::RigAgent;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    RigAgent::new(Config::new(), anthropic::CLAUDE_3_5_SONNET)
        .await?
        .start_repl()
        .await
}
