use crate::{ErrorResponse, Query, QueryResponse};
use reqwest::{Client, Url};
use std::error::Error;
use std::time::Instant;

pub async fn resolve(server: &str, query: &Query) -> Result<QueryResponse, Box<dyn Error>> {
    let url = Url::parse(server)?;
    let client = Client::new();
    let request_builder = client
        .get(url)
        .header("Accept", "application/dns-json")
        .query(&query);
    let start_instant = Instant::now();
    let body = request_builder.send().await?;

    let status = body.status().as_u16();
    let text = body.text().await?;

    match status {
        200 => {
            match serde_json::from_str::<QueryResponse>(&text) {
                Ok(mut response) => {
                    response.duration = start_instant.elapsed();
                    Ok(response)
                }
                Err(e) => Err(format!("decode error: {}\nbody: {}", e, text).into()),
            }
        }
        404 => {
            Err("invalid doh dns server".into())
        },
        400..599 => {
            match serde_json::from_str::<ErrorResponse>(&text) {
                Ok(response) => Err(response.error.into()),
                Err(e) => Err(format!("could not parse server error response: {}\nbody: {}", e, text).into()),
            }
        },
        _ => Err(text.into()),
    }
}
