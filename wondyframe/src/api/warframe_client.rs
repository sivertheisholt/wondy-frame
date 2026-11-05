use std::collections::HashMap;
use reqwest::Client;
use serde::de::DeserializeOwned;
use super::cache::{ApiCache, ApiResult};

pub struct WarframeClient {
    base_url: String,
    query_params: HashMap<String, String>,
    client: Client,
    cache: ApiCache,
}

impl WarframeClient {
    pub fn new(platform: &str) -> Self {
        Self {
            base_url: format!("https://api.warframestat.us/{}/", platform),
            query_params: HashMap::from([("language".to_string(), "en".to_string())]),
            client: Client::new(),
            cache: ApiCache::new(),
        }
    }

    pub async fn fetch<T: DeserializeOwned>(&self, endpoint: &str) -> ApiResult<T> {
        let url = if endpoint == "steelpath" {
            "https://api.warframestat.us/steelpath".to_string()
        } else {
            format!("{}{}", self.base_url, endpoint)
        };
        self.cache.fetch(&self.client, &url, Some(&self.query_params)).await
    }
}
