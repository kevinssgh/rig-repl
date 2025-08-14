use rig::vector_store::{VectorSearchRequest, VectorStoreIndex};
use crate::rag_builder::UniswapChunk;
use crate::rig_agent::RigAgent;

const SEPARATOR: &str = "\n\n---\n\n";
const DISTANCE: f64 = 0.9;
const MAX_CHAR_LEN: usize = 30000;

pub trait RagMiddleware {
    async fn query_rag(&mut self, prompt: &str) -> anyhow::Result<String>;
}

impl RagMiddleware for RigAgent {
    async fn query_rag(&mut self, query: &str) -> anyhow::Result<String> {
        let req = VectorSearchRequest::builder().query(query).samples(30).build()?;
        let search_results: Vec<(f64, String, UniswapChunk)> =  self.index.top_n(req).await?;

        // If there were no relevant results, just leave the original query
        if search_results.is_empty() {
            return Ok(String::from(query));
        }

        // Clear history before processing
        self.history.clear();

        let mut ctx_size = 0;
        let mut context = Vec::new();
        for (score, name, chunk) in search_results {
            if score > 0.9 {
                continue;
            }
            if chunk.content.len() + ctx_size > MAX_CHAR_LEN {
                continue;
            }
            ctx_size += chunk.content.len();
            context.push(format!("Source: {}\nContent: {}", name, chunk.content));
        }

        // Consolidate chunks
        let context = context.join(SEPARATOR);

        // Attach relevant information to the query for the Agent to use
        Ok(format!("You have access to the following relevant documentation: \n\n{context}\n\n --- \n\nUser: {query}"))
    }
}