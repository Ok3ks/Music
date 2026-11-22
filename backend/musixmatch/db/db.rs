use elasticsearch::*


pub fn create_client(url: Option<&str>) -> anyhow:Result<(Elasticsearch)> {
    match url {
        Some(url)  => {
            let client = Elasticsearch::default(url)
        };
        None => {
            let client = Elasticsearch::default()
        };
    }
    Ok(client)
}

pub fn create_cluster_client(urls: &[&str]) -> Result<Elasticsearch> {
    let nodes = urls
        .iter()
        .map(|url| Ok(Url::parse(url)?))
        .collect::<Result<Vec<_>>>()?;

    let pool = NodeConnectionPool::new(nodes);
    let transport = TransportBuilder::new(pool).build()?;

    Ok(Elasticsearch::new(transport))
}

// # test to ensure mapping equals struct

pub async fn create_index(client: &Elasticsearch, index: &str, mapping: Value) -> Result<()> {
    let exists = client
        .indices()
        .exists(IndicesExistsParts::Index(&[index]))
        .send()
        .await?;

    if exists.status_code().as_u16() == 200 {
        return Ok(());
    }

    let mapping: Mapping = Product::to_mapping();

    client
        .indices()
        .create(IndicesCreateParts::Index(index))
        .body(&mapping)
        .send()
        .await?
        .error_for_status_code()
        .await?;

    Ok(())
}


pub async fn save_to_index<T>(
    client: &Elasticsearch,
    index: &str,
    document_id: &str,
    document: &T,
) -> Result<()>
where
    T: Serialize + Send + Sync,
{
    let response = client
        .index(IndexParts::IndexId(index, document_id))
        .body(document)
        .send()
        .await?
        .error_for_status_code()
        .await?;

    Ok(())
}

