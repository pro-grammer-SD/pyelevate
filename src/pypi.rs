use crate::models::{Package, VersionStatus, compare_versions};
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

const PYPI_API_BASE: &str = "https://pypi.org/pypi";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PyPIRelease {
    info: PyPIInfo,
    releases: HashMap<String, Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PyPIInfo {
    name: String,
    version: String,
    #[serde(default)]
    yanked: bool,
}

pub struct PyPIClient {
    client: Client,
    cache: Arc<RwLock<HashMap<String, String>>>,
}

impl PyPIClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn fetch_latest_version(&self, package_name: &str) -> Result<String> {
        let cache = self.cache.read().await;
        if let Some(cached) = cache.get(package_name) {
            return Ok(cached.clone());
        }
        drop(cache);

        let url = format!("{}/{}/json", PYPI_API_BASE, package_name);
        
        match self.client.get(&url).send().await {
            Ok(response) => {
                if let Ok(data) = response.json::<PyPIRelease>().await {
                    let latest = data.info.version.clone();
                    self.cache.write().await.insert(package_name.to_string(), latest.clone());
                    Ok(latest)
                } else {
                    Err(anyhow::anyhow!("Failed to parse PyPI response"))
                }
            }
            Err(e) => Err(anyhow::anyhow!("PyPI request failed: {}", e)),
        }
    }

    pub async fn fetch_package_info(&self, package_name: &str) -> Result<(String, bool)> {
        let url = format!("{}/{}/json", PYPI_API_BASE, package_name);
        
        match self.client.get(&url).send().await {
            Ok(response) => {
                if let Ok(data) = response.json::<PyPIRelease>().await {
                    Ok((data.info.version.clone(), data.info.yanked))
                } else {
                    Err(anyhow::anyhow!("Failed to parse PyPI response"))
                }
            }
            Err(e) => Err(anyhow::anyhow!("PyPI request failed: {}", e)),
        }
    }

    pub async fn update_packages(&self, packages: &mut [Package]) {
        let mut handles = vec![];

        for package in packages.iter_mut() {
            let client = self.client.clone();
            let cache = self.cache.clone();
            let name = package.name.clone();

            let handle = tokio::spawn(async move {
                let cache_read = cache.read().await;
                if let Some(cached) = cache_read.get(&name) {
                    return (name.clone(), Ok(cached.clone()));
                }
                drop(cache_read);

                let url = format!("{}/{}/json", PYPI_API_BASE, &name);
                match client.get(&url).send().await {
                    Ok(response) => {
                        if let Ok(data) = response.json::<PyPIRelease>().await {
                            cache.write().await.insert(name.clone(), data.info.version.clone());
                            (name, Ok(data.info.version))
                        } else {
                            (name, Err(anyhow::anyhow!("Parse error")))
                        }
                    }
                    Err(e) => (name, Err(anyhow::anyhow!("Request failed: {}", e))),
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            if let Ok((name, result)) = handle.await {
                if let Some(pkg) = packages.iter_mut().find(|p| p.name == name) {
                    match result {
                        Ok(latest) => {
                            pkg.latest_version = Some(latest.clone());
                            pkg.status = compare_versions(&pkg.current_version, &latest);
                            pkg.error = None;
                        }
                        Err(e) => {
                            pkg.error = Some(e.to_string());
                            pkg.status = VersionStatus::Error;
                        }
                    }
                }
            }
        }
    }

    pub async fn update_package(&self, package: &mut Package) -> Result<()> {
        if let Some(cached) = self.cache.read().await.get(&package.name) {
            package.latest_version = Some(cached.clone());
            package.status = compare_versions(&package.current_version, cached);
            return Ok(());
        }

        let url = format!("{}/{}/json", PYPI_API_BASE, &package.name);
        
        let response = self.client.get(&url).send().await?;
        let data: PyPIRelease = response.json().await?;
        let latest = data.info.version.clone();

        self.cache.write().await.insert(package.name.clone(), latest.clone());
        package.latest_version = Some(latest.clone());
        package.status = compare_versions(&package.current_version, &latest);
        package.error = None;

        Ok(())
    }

    pub fn clear_cache(&self) {
        let cache = self.cache.blocking_write();
        std::mem::drop(cache);
    }
}

impl Default for PyPIClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pypi_client_creation() {
        let client = PyPIClient::new();
        assert_eq!(client.cache.read().await.len(), 0);
    }
}
