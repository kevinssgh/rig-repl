//! Rig Agent module.
//!
//! Provides a REPL interface to interact with Claude via the `rig` framework,
//! enhanced with RAG (retrieval-augmented generation) context and MCP tools.
//!
//! Responsibilities:
//! - Connects to Anthropic's API with a configured model.
//! - Builds a vector index from local documentation sources for contextual responses.
//! - Connects to an MCP server and registers available tools with the agent.
//! - Runs an interactive REPL loop to process natural language queries.
//!
//! Key types:
//! - [`RigAgent`]: Wraps an [`Agent`] instance with tool integration and RAG context.
//!
//! Key methods:
//! - [`RigAgent::new`]: Builds the agent with configured model, RAG index, and MCP tools.
//! - [`RigAgent::start_repl`]: Starts an interactive loop to send prompts and display replies.
//! - [`RigAgent::get_mcp_tools`]: Connects to MCP server, retrieves tool list, and returns the client.
//!
//! The REPL supports multi-turn conversations, tool use, and local knowledge base lookups.
//!

use mcp_core::client::{Client, ClientBuilder};
use mcp_core::transport::ClientSseTransport;
use mcp_core::types::ToolsListResponse;
use rig::agent::Agent;
use rig::client::CompletionClient;
use rig::completion::{Prompt, PromptError};
use rig::message::Message;
use rig::providers::anthropic;
use rig::providers::openai::EmbeddingModel;
use rig::vector_store::in_memory_store::InMemoryVectorIndex;
use rustyline::DefaultEditor;

use crate::common::Config;
use crate::rag_builder::{
    RAGBuilder,
    UniswapDoc,
};
use crate::rag_middleware::RagMiddleware;

const PROCESSING_MESSAGE: &str = "Claude: Processing Request...";

pub struct RigAgent {
    agent: Agent<anthropic::completion::CompletionModel>,
    pub(crate) index: InMemoryVectorIndex<EmbeddingModel, UniswapDoc>,
}

/// Implement Rig Agent
impl RigAgent {
    /// Creates a new `RigAgent`.
    ///
    /// Builds an Anthropic client with the given API key and model,
    /// sets up a RAG vector index from local docs,
    /// connects to the MCP server to load available tools,
    /// and configures the agent with preamble and dynamic context.
    pub async fn new(cfg: Config, model: &str) -> anyhow::Result<Self> {
        tracing::info!("Creating new Agent");
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
            //.dynamic_context(3, index)
            .build();

        Ok(Self { agent, index })
    }

    /// Starts the interactive REPL loop.
    ///
    /// Reads user input lines, sends prompts to the agent,
    /// handles multi-turn conversations with tool usage,
    /// and displays responses or errors.
    pub async fn start_repl(&self) -> anyhow::Result<()> {
        tracing::info!("Starting interactive REPL...");
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

                    // Process through RAG middleware
                    let query = self.query_rag(&line).await?;

                    // Process input through agent
                    match self
                        .agent
                        .prompt(query)
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

    /// Connects to the MCP server at the given address.
    ///
    /// Opens an SSE transport client,
    /// initializes the connection,
    /// retrieves the list of available tools,
    /// and returns both the tools and client.
    pub async fn get_mcp_tools(
        bind_addr: &str,
    ) -> anyhow::Result<(ToolsListResponse, Client<ClientSseTransport>)> {
        tracing::info!("Retrieving MCP tools from Server");
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

    /// Displays the agent’s reply in a user-friendly format.
    pub fn display_response(reply: &str) {
        println!("\nClaude:\n{reply}\n");
    }

    /// Displays the agent’s reply in a user-friendly format.
    pub fn display_prompt_err(err: PromptError) {
        println!("Sorry there was an error processing your input: {err}");
    }
}
