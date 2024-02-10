use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct KeywordParams {
    keyword: String,
    topic: Option<String>,
}

pub async fn find_keywords(opts: KeywordParams) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut query_params = vec![("ml", opts.keyword)];
    if let Some(topic) = opts.topic {
        query_params.push(("topics", topic));
    }

    let response = client
        .get("https://api.datamuse.com/words")
        .query(&query_params)
        .send()
        .await?;

    if response.status() != reqwest::StatusCode::OK {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("HTTP error! status: {}", response.status()),
        )));
    }

    let data: Vec<Value> = response.json().await?;
    let keywords = data
        .into_iter()
        .take(10)
        .filter_map(|item| {
            item.get("word")
                .and_then(|word| word.as_str())
                .map(String::from)
        })
        .collect();

    Ok(keywords)
}
