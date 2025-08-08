const ENV_SERVER_ADDRESS: &str = "MCP_SERVER_ADDRESS";
const ENV_SERVER_PORT: &str = "MCP_SERVER_PORT";
const ENV_ANTHROPIC_API_KEY: &str = "ANTHROPIC_API_KEY";
const ENV_RIG_PREAMBLE: &str = "RIG_PREAMBLE";

pub fn get_env_var(name: &str) -> anyhow::Result<String> {
    let var = std::env::var(name)?;
    Ok(var)
}

pub fn get_bind_address() -> anyhow::Result<String> {
    let addr = get_env_var(ENV_SERVER_ADDRESS)?;
    let port = get_env_var(ENV_SERVER_PORT)?;
    Ok(format!("{addr}:{port}"))
}

pub struct Config {
    pub server_bind_address: String,
    pub api_key: String,
    pub preamble: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            server_bind_address: get_bind_address().expect("get bind address failed"),
            api_key: get_env_var(ENV_ANTHROPIC_API_KEY).expect("failed to get anthropic api key"),
            preamble: get_env_var(ENV_RIG_PREAMBLE).expect("failed to set preamble"),
        }
    }
}
