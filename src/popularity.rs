use crate::models::PopularityData;
use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;

pub struct PopularityChecker {
    client: Client,
    cache: HashMap<String, Option<PopularityData>>,
}

impl PopularityChecker {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            cache: HashMap::new(),
        }
    }

    pub async fn fetch_popularity(&mut self, package: &str) -> Result<Option<PopularityData>> {
        if let Some(cached) = self.cache.get(package) {
            return Ok(cached.clone());
        }

        let popularity = self.fetch_from_pypi_stats(package).await.ok();
        self.cache.insert(package.to_string(), popularity.clone());
        Ok(popularity)
    }

    async fn fetch_from_pypi_stats(&self, package: &str) -> Result<PopularityData> {
        let url = format!("https://pypistats.org/api/packages/{}/recent", package);
        let response = self.client.get(&url).send().await?;
        let data: serde_json::Value = response.json().await?;

        let mut trend = Vec::new();
        if let Some(rows) = data.get("data").and_then(|d| d.as_array()) {
            for row in rows.iter().take(7) {
                if let (Some(date), Some(count)) = (
                    row.get("date").and_then(|d| d.as_str()),
                    row.get("downloads").and_then(|c| c.as_u64()),
                ) {
                    trend.push((date.to_string(), count));
                }
            }
        }

        let weekly = trend.iter().map(|(_, c)| c).sum::<u64>();

        Ok(PopularityData {
            downloads_last_month: weekly * 4,
            downloads_trend: trend,
            weekly_downloads: weekly,
            package_rank: None,
        })
    }
}

impl Default for PopularityChecker {
    fn default() -> Self {
        Self::new()
    }
}
