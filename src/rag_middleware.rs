use rig::vector_store::{VectorSearchRequest, VectorStoreIndex};
use crate::rag_builder::UniswapDoc;
use crate::rig_agent::RigAgent;

const SEPARATOR: &str = "\n\n---\n\n";

pub trait RagMiddleware {
    async fn query_rag(&self, prompt: &str) -> anyhow::Result<String>;
}

impl RagMiddleware for RigAgent {
    async fn query_rag(&self, query: &str) -> anyhow::Result<String> {
        let req = VectorSearchRequest::builder().query(query).samples(1).build()?;
        let search_results: Vec<(f64, String, UniswapDoc)> =  self.index.top_n(req).await?;

        // If there were no relevant results, just leave the original query
        if search_results.is_empty() {
            return Ok(String::from(query));
        }

        let context_chunks: Vec<String> = search_results
            .iter()
            .map(|(_, name, doc)| {
                format!("Source: {}\nContent: {}", name, doc.content.join(SEPARATOR))
            })
            .collect();

        // Consolidate chunks
        let context = context_chunks.join(SEPARATOR);

        // Attach relevant information to the query for the Agent to use
        Ok(format!("You have access to the following relevant documentation: \n\n{context}\n\n --- \n\nUser: {query}"))
    }
}