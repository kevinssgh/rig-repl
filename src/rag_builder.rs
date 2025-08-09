use crate::common::Config;

use rig::prelude::EmbeddingsClient;
use rig::providers::openai::client::Client;
use rig::providers::openai::{EmbeddingModel, TEXT_EMBEDDING_ADA_002};
use rig::vector_store::in_memory_store::{InMemoryVectorIndex, InMemoryVectorStore};
use rig::{Embed, embeddings::EmbeddingsBuilder};
use serde::Serialize;
use std::fs;
use text_splitter::CodeSplitter;
use walkdir::WalkDir;

const MD_EXTENSION: &str = "md";
const SOL_EXTENSION: &str = "sol";

#[derive(Embed, Serialize, Clone, Debug, Eq, PartialEq, Default)]
pub struct UniswapDoc {
    name: String,
    #[embed]
    content: Vec<String>,
}

impl UniswapDoc {
    fn new(name: String) -> Self {
        Self {
            name,
            content: Vec::new(),
        }
    }
}

pub struct RAGBuilder {
    docs: Vec<UniswapDoc>,
    cfg: Config,
}

impl RAGBuilder {
    pub fn new(cfg: Config) -> Self {
        Self {
            docs: Vec::new(),
            cfg,
        }
    }

    pub async fn build(self) -> anyhow::Result<InMemoryVectorIndex<EmbeddingModel, UniswapDoc>> {
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

    /// Ingest Docs found in provided directories
    ///
    ///  Note: Follows consumable self pattern for a builder:
    ///     let result = RAGBuilder::new(Config)
    ///                     .ingest_docs()
    ///                     .build()
    pub fn ingest_docs(mut self) -> anyhow::Result<Self> {
        // Walk through each directory and process relevant files
        for dir in self.cfg.rag_directories.clone() {
            for file in WalkDir::new(dir)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
            {
                let path = file.path();
                let name = format!("{:?}", file.file_name());

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

    fn ingest_md_file(mut self, file: String, name: String) -> anyhow::Result<Self> {
        let mut doc = UniswapDoc::new(name);
        let splitter = text_splitter::MarkdownSplitter::new(text_splitter::ChunkConfig::new(1000));
        // Using a text splitter specifically for markdown files to maintain headers and other tags
        for chunk in splitter.chunks(&file) {
            doc.content.push(String::from(chunk))
        }
        self.docs.push(doc);

        Ok(self)
    }

    fn ingest_solidity_file(mut self, file: String, name: String) -> anyhow::Result<Self> {
        let mut doc = UniswapDoc::new(name);
        let splitter = CodeSplitter::new(
            tree_sitter_solidity::LANGUAGE,
            text_splitter::ChunkConfig::new(1000),
        )?;

        for chunk in splitter.chunks(&file) {
            doc.content.push(String::from(chunk))
        }
        self.docs.push(doc);

        Ok(self)
    }
}
