use rand::Rng;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;

const NATS: [&str; 12] = [
    "AU", "CA", "FR", "IN", "IR", "MX", "NL", "NO", "NZ", "RS", "TR", "US",
];

#[derive(Debug, Serialize, Deserialize)]
pub struct NameParams {
    gender: Option<String>,
    nat: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RandomUserName {
    first: String,
    last: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RandomUserResult {
    name: RandomUserName,
}

#[derive(Debug, Serialize, Deserialize)]
struct RandomUserResponse {
    results: Vec<RandomUserResult>,
}

pub async fn get_random_name(params: NameParams) -> Result<String, Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let nat_set: HashSet<&str> = NATS.iter().cloned().collect();

    let nat = match params.nat {
        Some(ref nat) if nat_set.contains(nat.as_str()) => nat.clone(),
        _ => NATS[rng.gen_range(0..NATS.len())].to_string(),
    };

    let client = reqwest::Client::new();
    let mut query_params = vec![];
    if let Some(gender) = params.gender {
        query_params.push(("gender", gender));
    }
    query_params.push(("nat", nat));

    let response = client
        .get("https://randomuser.me/api/")
        .query(&query_params)
        .send()
        .await?;

    if response.status() != reqwest::StatusCode::OK {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("HTTP error! status: {}", response.status()),
        )));
    }
    let data: RandomUserResponse = response.json().await?;
    let random_user_result = data.results.get(0).ok_or("No user found")?;
    let first_name = random_user_result.name.first.clone();
    let last_name = random_user_result.name.last.clone();
    Ok(format!("{} {}", first_name, last_name))
}
