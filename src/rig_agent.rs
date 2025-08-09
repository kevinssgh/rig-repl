use mcp_core::client::{Client, ClientBuilder};
use mcp_core::transport::ClientSseTransport;
use mcp_core::types::ToolsListResponse;
use rig::agent::Agent;
use rig::client::CompletionClient;
use rig::completion::{Prompt, PromptError};
use rig::message::Message;
use rig::providers::anthropic;
use rustyline::DefaultEditor;

use crate::common::Config;
use crate::rag_builder::RAGBuilder;

const PROCESSING_MESSAGE: &str = "Claude: Processing Request...";

pub struct RigAgent {
    provider: Agent<anthropic::completion::CompletionModel>,
}

/// Implement Rig Agent
impl RigAgent {
    /// Construct new instance
    pub async fn new(cfg: Config, model: &str) -> anyhow::Result<Self> {
        let client = anthropic::ClientBuilder::new(&cfg.api_key)
            .build()
            .expect("returns provider client");

        // Read local filesystem for relevant documentation and create Vector Index
        let index = RAGBuilder::new(cfg.clone())
            .ingest_docs()? // Walks through each file specified by directories, and indexes chunks of data
            .build()
            .await?;

        let mut agent_builder = client.agent(model);

        let (tools_list, mcp_client) = Self::get_mcp_tools(&cfg.server_bind_address)
            .await
            .expect("returns client tools and connection");

        agent_builder = tools_list
            .tools
            .into_iter()
            .fold(agent_builder, |builder, tool| {
                builder.mcp_tool(tool, mcp_client.clone())
            });

        let agent = agent_builder
            .preamble(&cfg.preamble)
            .dynamic_context(3, index)
            .build();

        Ok(Self { provider: agent })
    }

    /// Start REPL loop
    pub async fn start_repl(&self) -> anyhow::Result<()> {
        let mut rl = DefaultEditor::new()?;
        let mut history: Vec<Message> = Vec::new();

        println!(
            "🔧 Claude REPL with Tools (type natural language, like 'Check ETH balance of Alice')"
        );

        loop {
            match rl.readline(">>> ") {
                Ok(line) => {
                    if line.trim() == "quit" {
                        break;
                    }

                    println!("{PROCESSING_MESSAGE}");

                    // Process input through agent
                    match self
                        .provider
                        .prompt(line)
                        .multi_turn(20)
                        .with_history(&mut history)
                        .await
                    {
                        Ok(reply) => {
                            Self::display_response(&reply);
                        }
                        Err(e) => {
                            Self::display_prompt_err(e);
                        }
                    }
                }
                Err(e) => {
                    println!("read line error: {e}")
                }
            }
        }
        Ok(())
    }

    /// Get MCP tools
    pub async fn get_mcp_tools(
        bind_addr: &str,
    ) -> anyhow::Result<(ToolsListResponse, Client<ClientSseTransport>)> {
        let sse_url = format!("http://{bind_addr}/sse");

        // Connect to MCP server via SSE transport
        let client_transport = mcp_core::transport::ClientSseTransportBuilder::new(sse_url).build();
        let mcp_client = ClientBuilder::new(client_transport).build();

        // Open Stream to MCP server
        mcp_client.open().await?;
        mcp_client.initialize().await?;

        // Get available mcp tools
        let tools_list_res = mcp_client.list_tools(None, None).await?;
        Ok((tools_list_res, mcp_client))
    }

    pub fn display_response(reply: &str) {
        println!("\nClaude:\n{reply}\n");
    }

    pub fn display_prompt_err(err: PromptError) {
        println!("Sorry there was an error processing your input: {err}");
    }
}
