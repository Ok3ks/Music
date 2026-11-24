use anyhow::Context;
use elasticsearch::http::transport::{MultiNodeConnectionPool, Transport, TransportBuilder};
use elasticsearch::http::Url;
use elasticsearch::indices::{IndicesCreateParts, IndicesExistsParts};
use elasticsearch::{Elasticsearch, IndexParts};
use serde::Serialize;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

pub fn create_client(url: Option<&str>) -> anyhow::Result<Elasticsearch> {
    let client = match url {
        Some(url) => Elasticsearch::new(Transport::single_node(url)?),
        None => Elasticsearch::new(Transport::single_node("http://localhost:9200")?),
    };
    Ok(client)
}

pub fn create_cluster_client(urls: &[&str]) -> anyhow::Result<Elasticsearch> {
    let nodes = urls
        .iter()
        .map(|url| Ok(Url::parse(url)?))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let pool = MultiNodeConnectionPool::round_robin(nodes, None);
    let transport = TransportBuilder::new(pool).build()?;

    Ok(Elasticsearch::new(transport))
}

pub async fn run_migrations(client: &Elasticsearch) -> anyhow::Result<()> {
    let migrations_path = Path::new("db/migrations/elasticsearch");
    let mut files: Vec<PathBuf> = fs::read_dir(migrations_path)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
        .collect();

    files.sort();

    for file_path in files {
        let content = fs::read_to_string(&file_path)?;
        let object: Value = serde_json::from_str(&content)?;

        let index = object["index"]
            .as_str()
            .with_context(|| format!("{} has no \"index\"", file_path.display()))?;
        let alias = object["alias"]
            .as_str()
            .with_context(|| format!("{} has no \"alias\"", file_path.display()))?;

        let body = json!({
            "settings": object["settings"],
            "mappings": object["mappings"],
            "aliases": { alias: {} }
        });

        create_index(client, index, body).await?;
        println!("migrated {index}");
    }
    Ok(())
}

pub async fn create_index(client: &Elasticsearch, index: &str, mapping: Value) -> anyhow::Result<()> {
    let exists = client
        .indices()
        .exists(IndicesExistsParts::Index(&[index]))
        .send()
        .await?;

    if exists.status_code().as_u16() == 200 {
        return Ok(());
    }

    client
        .indices()
        .create(IndicesCreateParts::Index(index))
        .body(&mapping)
        .send()
        .await?
        .error_for_status_code()?;

    Ok(())
}


pub async fn save_to_index<T>(
    client: &Elasticsearch,
    index: &str,
    document_id: &str,
    document: &T,
) -> anyhow::Result<()>
where
    T: Serialize + Send + Sync,
{
    client
        .index(IndexParts::IndexId(index, document_id))
        .body(document)
        .send()
        .await?
        .error_for_status_code()?;

    Ok(())
}

/// Blocking wrapper so the (blocking) scraper can write to Elasticsearch.
pub struct Indexer {
    client: Elasticsearch,
    runtime: tokio::runtime::Runtime,
}

impl Indexer {
    pub fn new(client: Elasticsearch) -> anyhow::Result<Self> {
        let runtime = tokio::runtime::Runtime::new()?;
        Ok(Indexer { client, runtime })
    }

    pub fn run_migrations(&self) -> anyhow::Result<()> {
        self.runtime.block_on(run_migrations(&self.client))
    }

    pub fn save<T>(&self, index: &str, document_id: &str, document: &T)
    where
        T: Serialize + Send + Sync,
    {
        let result = self
            .runtime
            .block_on(save_to_index(&self.client, index, document_id, document));
        if let Err(e) = result {
            eprintln!("failed to index {index}/{document_id}: {e}");
        }
    }
}

