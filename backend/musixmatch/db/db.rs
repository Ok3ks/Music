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