use mcp_core::client::{Client, ClientBuilder};
use mcp_core::transport::ClientSseTransport;
use mcp_core::types::ToolsListResponse;
use rig::agent::Agent;
use rig::client::CompletionClient;
use rig::completion::Chat;
use rig::providers::anthropic;
use rustyline::DefaultEditor;
use tokio;


struct RigAgent {
    provider: Agent<anthropic::completion::CompletionModel>,
}

/// Implement Rig Agent
impl RigAgent {
    /// Construct new instance
    pub async fn new(api_key: &str, model: &str) -> Self {
        let client = anthropic::ClientBuilder::new(api_key)
            .build()
            .expect("returns provider client");
        let mut agent_builder = client.agent(model);

        let (tools_list, mcp_client) = Self::get_mcp_tools()
            .await
            .expect("returns client tools and connection");

        agent_builder = tools_list
            .tools
            .into_iter()
            .fold(agent_builder, |builder, tool| {
                builder.mcp_tool(tool, mcp_client.clone())
            });

        let agent = agent_builder.build();

        Self { provider: agent }
    }

    /// Start REPL loop
    pub async fn start_repl(&self) -> anyhow::Result<()> {
        let mut rl = DefaultEditor::new()?;
        let mut chat_history = Vec::new();
        println!(
            "🔧 Claude REPL with Tools (type natural language, like 'Check balance of vitalik.eth')"
        );

        loop {
            match rl.readline(">>> ") {
                Ok(line) => {
                    chat_history.push(rig::message::Message::from(&line));
                    let reply = self.provider.chat(&line, chat_history.clone()).await?;
                    println!("\nClaude:\n{}\n", reply);
                }
                Err(_) => break,
            }
        }
        Ok(())
    }

    /// Get MCP tools
    pub async fn get_mcp_tools() -> anyhow::Result<(ToolsListResponse, Client<ClientSseTransport>)>
    {
        // Connect to MCP server via SSE transport
        let client_transport = mcp_core::transport::ClientSseTransportBuilder::new(String::from(
            "http://0.0.0.0:3000/sse",
        ))
        .build();
        let mcp_client = ClientBuilder::new(client_transport).build();

        // Open Stream to MCP server
        mcp_client.open().await?;
        mcp_client.initialize().await?;

        // Get available mcp tools
        let tools_list_res = mcp_client.list_tools(None, None).await?;
        Ok((tools_list_res, mcp_client))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    RigAgent::new(API_KEY, anthropic::CLAUDE_3_5_SONNET)
        .await
        .start_repl()
        .await
}
