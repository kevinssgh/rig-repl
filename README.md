# rig-repl: An interactive REPL using rig-core and Anthropic

## Descriptions
This is a simple REPL client that runs an agent using CLAUDE_3_5_SONNET. 

* Loads tools dynamically from an MCP server providing interfaces to blockchain and web services.
* Provides a RAG (Retrieval-augmented generation) system using an embedded openai model.
* Ingests documents into the RAG system using an in memory vector store.

## Requirements
Below are the environment variables required to run the agent.

* `MCP_SERVER_ADDRESS`: binding address of the mcp server ex. "0.0.0.0"
* `MCP_SERVER_PORT`: mcp server port 
* `ANTHROPIC_API_KEY`: anthropic api key used to interact with the model
* `OPENAI_API_KEY`: openai api key used for the embedded model to perform index searches 
* `RIG_PREAMBLE`: preamble to the claude agent, an initial context string
* `UNISWAP_DOCS_DIR_V2`: dir to uniswap v2 docs
* `UNISWAP_DOCS_DIR_V3`: dir to uniswap v3 docs
* `UNISWAP_SOURCE_DIR_V2`: dir to uniswap v2 source code
* `UNISWAP_SOURCE_DIR_V3`: dir to uniswap v3 source code

These will be added to a configuration file in the future. Directories for the RAG system in particular will be changed to a list for more dynamic content.

