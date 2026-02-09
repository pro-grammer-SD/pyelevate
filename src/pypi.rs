use crate::models::Package;
use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

const PYPI_API: &str = "https://pypi.org/pypi";
const PYPI_STATS: &str = "https://pypistats.org/api/packages";

pub struct PyPIClient {
    client: Client,
    cache: Arc<RwLock<HashMap<String, CachedPackage>>>,
}

#[derive(Clone, Debug)]
struct CachedPackage {
    name: String,
    latest: Option<String>,
    metadata: Option<PyPIMetadata>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct PyPIMetadata {
    pub name: String,
    pub version: String,
    pub summary: String,
    pub home_page: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    pub project_urls: Option<HashMap<String, String>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct PyPIResponse {
    pub info: PyPIMetadata,
    pub releases: HashMap<String, Vec<serde_json::Value>>,
}

impl PyPIClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn update_packages(&self, packages: &mut [Package]) {
        let mut handles = vec![];

        for pkg in packages.iter_mut() {
            if !matches!(pkg.source, crate::models::DependencySource::PyPI) {
                continue;
            }

            let client = self.client.clone();
            let name = pkg.name.clone();
            let cache = self.cache.clone();

            let handle = tokio::spawn(async move {
                if let Ok(cached) = Self::fetch_package_info(&client, &name).await {
                    let mut c = cache.write();
                    c.insert(
                        name.clone(),
                        CachedPackage {
                            name: name.clone(),
                            latest: Some(cached.version.clone()),
                            metadata: None,
                        },
                    );
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            let _ = handle.await;
        }

        let cache = self.cache.read();
        for pkg in packages.iter_mut() {
            if let Some(cached) = cache.get(&pkg.name) {
                if let Some(latest) = &cached.latest {
                    pkg.latest_version = Some(latest.clone());
                    pkg.status = crate::models::compare_versions(&pkg.current_version, latest);
                }
            }
        }
    }

    pub async fn fetch_latest_version(&self, package: &str) -> Result<Option<String>> {
        let cache = self.cache.read();
        if let Some(cached) = cache.get(package) {
            if let Some(latest) = &cached.latest {
                return Ok(Some(latest.clone()));
            }
        }
        drop(cache);

        if let Ok(response) = Self::fetch_package_info(&self.client, package).await {
            let version = response.version;
            let mut cache = self.cache.write();
            cache.insert(
                package.to_string(),
                CachedPackage {
                    name: package.to_string(),
                    latest: Some(version.clone()),
                    metadata: None,
                },
            );
            Ok(Some(version))
        } else {
            Ok(None)
        }
    }

    async fn fetch_package_info(client: &Client, package: &str) -> Result<PyPIMetadata> {
        let url = format!("{}/{}/json", PYPI_API, package);
        let response = client
            .get(&url)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await?;

        let data: PyPIResponse = response.json().await?;
        Ok(data.info)
    }

    pub async fn fetch_popularity(&self, package: &str) -> Result<Option<crate::models::PopularityData>> {
        let url = format!("{}/{}/recent", PYPI_STATS, package);
        
        match self
            .client
            .get(&url)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
        {
            Ok(response) => {
                if let Ok(data) = response.json::<serde_json::Value>().await {
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

                    return Ok(Some(crate::models::PopularityData {
                        downloads_last_month: weekly.saturating_mul(4),
                        downloads_trend: trend,
                        weekly_downloads: weekly,
                        package_rank: None,
                    }));
                }
            }
            Err(_) => {}
        }

        Ok(None)
    }
}

impl Default for PyPIClient {
    fn default() -> Self {
        Self::new()
    }
}
