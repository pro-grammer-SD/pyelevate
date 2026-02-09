use crate::models::{Package, SecurityAdvisory, SecurityStatus, Severity};
use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;

const OSV_API: &str = "https://api.osv.dev/v1/query";

pub struct SecurityChecker {
    client: Client,
    cache: HashMap<String, Vec<SecurityAdvisory>>,
}

impl SecurityChecker {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            cache: HashMap::new(),
        }
    }

    pub async fn check_package(&mut self, pkg: &mut Package) -> Result<()> {
        if !matches!(pkg.source, crate::models::DependencySource::PyPI) {
            return Ok(());
        }

        if let Some(cached) = self.cache.get(&pkg.name) {
            pkg.security_status = if cached.is_empty() {
                SecurityStatus::Safe
            } else {
                SecurityStatus::Vulnerable { cve_count: cached.len() }
            };
            return Ok(());
        }

        let advisories = self.fetch_advisories(&pkg.name, &pkg.current_version).await?;
        
        pkg.security_status = if advisories.is_empty() {
            SecurityStatus::Safe
        } else {
            SecurityStatus::Vulnerable { cve_count: advisories.len() }
        };

        self.cache.insert(pkg.name.clone(), advisories);
        Ok(())
    }

    async fn fetch_advisories(&self, package: &str, version: &str) -> Result<Vec<SecurityAdvisory>> {
        let query = serde_json::json!({
            "package": {
                "name": package,
                "ecosystem": "PyPI"
            },
            "version": version
        });

        match self.client.post(OSV_API).json(&query).send().await {
            Ok(response) => {
                if let Ok(data) = response.json::<serde_json::Value>().await {
                    if let Some(vulns) = data.get("vulns").and_then(|v| v.as_array()) {
                        let advisories = vulns
                            .iter()
                            .filter_map(|v| {
                                let id = v.get("id")?.as_str()?;
                                let summary = v.get("summary")?.as_str()?;
                                let severity_str = v
                                    .get("severity")
                                    .and_then(|s| s.as_str())
                                    .unwrap_or("MEDIUM");

                                Some(SecurityAdvisory {
                                    id: id.to_string(),
                                    title: summary.to_string(),
                                    severity: match severity_str {
                                        "CRITICAL" => Severity::Critical,
                                        "HIGH" => Severity::High,
                                        "MEDIUM" => Severity::Medium,
                                        _ => Severity::Low,
                                    },
                                    affected_versions: Vec::new(),
                                    fixed_version: None,
                                    url: format!("https://osv.dev/{}", id),
                                })
                            })
                            .collect();
                        return Ok(advisories);
                    }
                }
            }
            Err(_) => {}
        }

        Ok(Vec::new())
    }
}

impl Default for SecurityChecker {
    fn default() -> Self {
        Self::new()
    }
}
