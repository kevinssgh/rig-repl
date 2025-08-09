const ENV_SERVER_ADDRESS: &str = "MCP_SERVER_ADDRESS";
const ENV_SERVER_PORT: &str = "MCP_SERVER_PORT";
const ENV_ANTHROPIC_API_KEY: &str = "ANTHROPIC_API_KEY";
const ENV_OPENAI_API_KEY: &str = "OPENAI_API_KEY";
const ENV_RIG_PREAMBLE: &str = "RIG_PREAMBLE";
const ENV_UNISWAP_DOCS_DIR_V2: &str = "UNISWAP_DOCS_DIR_V2";
const ENV_UNISWAP_DOCS_DIR_V3: &str = "UNISWAP_DOCS_DIR_V3";
const ENV_UNISWAP_SOURCE_DIR_V2: &str = "UNISWAP_SOURCE_DIR_V2";
const ENV_UNISWAP_SOURCE_DIR_V3: &str = "UNISWAP_SOURCE_DIR_V3";

pub fn get_env_var(name: &str) -> anyhow::Result<String> {
    let var = std::env::var(name)?;
    Ok(var)
}

pub fn get_bind_address() -> anyhow::Result<String> {
    let addr = get_env_var(ENV_SERVER_ADDRESS)?;
    let port = get_env_var(ENV_SERVER_PORT)?;
    Ok(format!("{addr}:{port}"))
}

#[derive(Clone)]
pub struct Config {
    pub server_bind_address: String,
    pub api_key: String,
    pub openai_api_key: String,
    pub preamble: String,
    pub rag_directories: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        // Get directories for RAG system
        let rag_directories = vec![
            get_env_var(ENV_UNISWAP_DOCS_DIR_V2).expect("ENV_UNISWAP_DOCS_DIR_V2 not set"),
            get_env_var(ENV_UNISWAP_DOCS_DIR_V3).expect("ENV_UNISWAP_DOCS_DIR_V3 not set"),
            get_env_var(ENV_UNISWAP_SOURCE_DIR_V2).expect("ENV_UNISWAP_SOURCE_DIR_V2 not set"),
            get_env_var(ENV_UNISWAP_SOURCE_DIR_V3).expect("ENV_UNISWAP_SOURCE_DIR_V3 not set"),
        ];

        Self {
            server_bind_address: get_bind_address().expect("get bind address failed"),
            api_key: get_env_var(ENV_ANTHROPIC_API_KEY).expect("failed to get anthropic api key"),
            openai_api_key: get_env_var(ENV_OPENAI_API_KEY).expect("failed to get openai api key"),
            preamble: get_env_var(ENV_RIG_PREAMBLE).expect("failed to set preamble"),
            rag_directories,
        }
    }
}
