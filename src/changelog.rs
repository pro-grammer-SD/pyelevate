use crate::models::Changelog;
use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;

pub struct ChangelogFetcher {
    client: Client,
    cache: HashMap<String, Option<Changelog>>,
}

impl ChangelogFetcher {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            cache: HashMap::new(),
        }
    }

    pub async fn fetch_changelog(&mut self, package: &str, version: &str) -> Result<Option<Changelog>> {
        let cache_key = format!("{}-{}", package, version);
        
        if let Some(cached) = self.cache.get(&cache_key) {
            return Ok(cached.clone());
        }

        let changelog = self.fetch_from_pypi(package, version).await
            .or_else(|_| futures::executor::block_on(self.fetch_from_github(package, version)))
            .ok();

        self.cache.insert(cache_key, changelog.clone());
        Ok(changelog)
    }

    async fn fetch_from_pypi(&self, package: &str, version: &str) -> Result<Changelog> {
        let url = format!("https://pypi.org/pypi/{}/{}/json", package, version);
        let response = self.client.get(&url).send().await?;
        let data: serde_json::Value = response.json().await?;

        let _home_page = data
            .get("info")
            .and_then(|i| i.get("home_page"))
            .and_then(|h| h.as_str())
            .unwrap_or("");

        let summary = data
            .get("info")
            .and_then(|i| i.get("summary"))
            .and_then(|s| s.as_str())
            .unwrap_or("No description available");

        Ok(Changelog {
            version: version.to_string(),
            release_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            changes: vec![summary.to_string()],
            breaking_changes: detect_breaking_changes(summary),
            deprecated: detect_deprecated(summary),
            security_fixes: detect_security_fixes(summary),
        })
    }

    async fn fetch_from_github(&self, _package: &str, _version: &str) -> Result<Changelog> {
        Err(anyhow::anyhow!("GitHub fetch not yet implemented"))
    }
}

fn detect_breaking_changes(text: &str) -> Vec<String> {
    let keywords = vec![
        "breaking change",
        "breaking changes",
        "removed",
        "incompatible",
        "deprecated in favor of",
    ];

    keywords
        .iter()
        .filter(|k| text.to_lowercase().contains(**k))
        .map(|k| format!("Detected: {}", k))
        .collect()
}

fn detect_deprecated(text: &str) -> Vec<String> {
    let keywords = vec!["deprecated", "will be removed"];

    keywords
        .iter()
        .filter(|k| text.to_lowercase().contains(**k))
        .map(|k| format!("Deprecated: {}", k))
        .collect()
}

fn detect_security_fixes(text: &str) -> Vec<String> {
    let keywords = vec![
        "security",
        "cve",
        "vulnerability",
        "fix vulnerability",
        "patch vulnerability",
    ];

    keywords
        .iter()
        .filter(|k| text.to_lowercase().contains(**k))
        .map(|k| format!("Security fix: {}", k))
        .collect()
}

impl Default for ChangelogFetcher {
    fn default() -> Self {
        Self::new()
    }
}
