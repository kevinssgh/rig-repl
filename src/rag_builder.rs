//! RAG (Retrieval-Augmented Generation) builder module.
//!
//! Provides functionality to ingest and process documentation files,
//! specifically Markdown (`.md`) and Solidity (`.sol`) sources,
//! and build an in-memory vector index for use with OpenAI embeddings.
//!
//! This module enables the creation of a searchable knowledge base by:
//! - Walking configured directories to locate relevant files.
//! - Splitting file contents into manageable chunks while preserving structure.
//! - Embedding chunks into vector representations using OpenAI's embedding models.
//!
//! The resulting vector index supports retrieval tasks for enhanced language model context.

use crate::common::Config;

use rig::prelude::EmbeddingsClient;
use rig::providers::openai::client::Client;
use rig::providers::openai::{EmbeddingModel, TEXT_EMBEDDING_ADA_002};
use rig::vector_store::in_memory_store::{InMemoryVectorIndex, InMemoryVectorStore};
use rig::{Embed, embeddings::EmbeddingsBuilder};
use serde::{Serialize, Deserialize};
use std::fs;
use text_splitter::CodeSplitter;
use walkdir::WalkDir;

const MD_EXTENSION: &str = "md";
const SOL_EXTENSION: &str = "sol";

/// Represents a file with its data indexed into segments
#[derive(Embed, Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
pub struct UniswapChunk {
    file_name: String,
    #[embed]
    pub(crate) content: String,
}

impl UniswapChunk {
    fn new(file_name: &str) -> Self {
        Self {
            file_name: String::from(file_name),
            content: String::new(),
        }
    }
}

/// Builder for retrieval-augmented generation (RAG) vector index
/// that ingests documentation files and prepares an embedding index.
pub struct RAGBuilder {
    docs: Vec<UniswapChunk>,
    cfg: Config,
}

impl RAGBuilder {
    pub fn new(cfg: Config) -> Self {
        Self {
            docs: Vec::new(),
            cfg,
        }
    }

    /// Builds an in-memory vector index using OpenAI embeddings
    /// from the ingested documents.
    pub async fn build(self) -> anyhow::Result<InMemoryVectorIndex<EmbeddingModel, UniswapChunk>> {
        tracing::info!("Setting up Vector Index for Uniswap docs");
        // Create OpenAI client
        let openai_api_key = self.cfg.openai_api_key.clone();
        let client = Client::new(&openai_api_key);
        let embedding_model = client.embedding_model(TEXT_EMBEDDING_ADA_002);

        let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
            .documents(self.docs)?
            .build()
            .await?;

        let vector_store = InMemoryVectorStore::from_documents(embeddings);
        let index = vector_store.index(embedding_model);

        Ok(index)
    }

    /// Walks through configured directories and ingests relevant `.md` and `.sol` files,
    /// returning an updated builder for chaining.
    ///
    /// # Example
    /// ```
    /// let rag_index = RAGBuilder::new(config)
    ///     .ingest_docs()?
    ///     .build()
    ///     .await?;
    /// ```
    pub fn ingest_docs(mut self) -> anyhow::Result<Self> {
        tracing::debug!("Ingesting Uniswap docs and source code");
        // Walk through each directory and process relevant files
        for dir in self.cfg.rag_directories.clone() {
            for file in WalkDir::new(dir)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
            {
                let path = file.path();
                let name = format!("{:?}", file.file_name());
                tracing::debug!("Ingesting file: {name}");

                match file.path().extension().and_then(|ext| ext.to_str()) {
                    // Only read the file if matches with one of the extensions
                    Some(MD_EXTENSION) => {
                        let file_str = fs::read_to_string(path)?;
                        self = self.ingest_md_file(file_str, name.clone())?
                    }
                    Some(SOL_EXTENSION) => {
                        let file_str = fs::read_to_string(path)?;
                        self = self.ingest_solidity_file(file_str, name)?
                    }
                    _ => {
                        // Ignore other files for now
                    }
                }
            }
        }
        Ok(self)
    }

    /// Processes a Markdown file by splitting it into chunks
    /// and adding them to the document content.
    fn ingest_md_file(mut self, file: String, name: String) -> anyhow::Result<Self> {
        let splitter = text_splitter::MarkdownSplitter::new(text_splitter::ChunkConfig::new(1000));

        // Using a text splitter specifically for markdown files to maintain headers and other tags
        for chunk in splitter.chunks(&file) {
            let mut doc = UniswapChunk::new(&name);
            doc.content = String::from(chunk);
            self.docs.push(doc);
        }

        Ok(self)
    }

    /// Processes a Solidity source file by splitting it into code chunks
    /// using a language-aware splitter and adding them to the document content.
    fn ingest_solidity_file(mut self, file: String, name: String) -> anyhow::Result<Self> {
        let code_splitter = CodeSplitter::new(
            tree_sitter_solidity::LANGUAGE,
            text_splitter::ChunkConfig::new(1000),
        )?;

        for chunk in code_splitter.chunks(&file) {
            let mut doc = UniswapChunk::new(&name);
            doc.content = String::from(chunk);
            self.docs.push(doc);
        }

        Ok(self)
    }
}
