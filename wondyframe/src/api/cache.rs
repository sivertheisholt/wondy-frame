use std::{collections::HashMap, time::{Duration, Instant}};
use serde::de::DeserializeOwned;
use tokio::sync::Mutex;
use tracing::debug;

pub type ApiResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// In-process TTL cache. The lock is held during refresh, so concurrent
/// requests cannot stampede the upstream API.
pub struct ApiCache {
    entries: Mutex<HashMap<String, (Instant, serde_json::Value)>>,
}

impl ApiCache {
    pub fn new() -> Self {
        Self { entries: Mutex::new(HashMap::new()) }
    }

    pub async fn fetch<T: DeserializeOwned>(
        &self,
        client: &reqwest::Client,
        url: &str,
        query: Option<&HashMap<String, String>>,
    ) -> ApiResult<T> {
        let key = match query {
            Some(params) => {
                let mut pairs: Vec<_> = params.iter().collect();
                pairs.sort_by_key(|(k, _)| *k);
                format!("{url}?{}", pairs.iter().map(|(k,v)| format!("{k}={v}")).collect::<Vec<_>>().join("&"))
            }
            None => url.to_owned(),
        };
        let mut entries = self.entries.lock().await;
        if let Some((created, value)) = entries.get(&key) {
            if created.elapsed() < Duration::from_secs(300) {
                debug!("API cache hit: {}", key);
                return Ok(serde_json::from_value(value.clone())?);
            }
        }

        debug!("API cache miss: {}", key);
        let mut request = client.get(url);
        if let Some(params) = query {
            request = request.query(params);
        }
        let value: serde_json::Value = request.send().await?.error_for_status()?.json().await?;
        // Only successful responses are cached.
        let result = serde_json::from_value(value.clone())?;
        entries.insert(key, (Instant::now(), value));
        // Expired entries are pruned on refresh to avoid unbounded growth.
        entries.retain(|_, (created, _)| created.elapsed() < Duration::from_secs(300));
        Ok(result)
    }
}
