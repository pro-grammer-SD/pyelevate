use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub current_version: String,
    pub latest_version: Option<String>,
    pub status: VersionStatus,
    pub selected: bool,
    pub extras: Vec<String>,
    pub constraint: VersionConstraint,
    pub error: Option<String>,
    pub source: DependencySource,
    pub security_status: SecurityStatus,
    pub changelog: Option<Changelog>,
    pub popularity: Option<PopularityData>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DependencySource {
    PyPI,
    Git {
        url: String,
        ref_spec: Option<String>,
    },
    LocalPath {
        path: String,
        editable: bool,
    },
    Url {
        url: String,
    },
    Unknown,
}

impl DependencySource {
    pub fn source_type(&self) -> &'static str {
        match self {
            DependencySource::PyPI => "PyPI",
            DependencySource::Git { .. } => "Git",
            DependencySource::LocalPath { .. } => "Local",
            DependencySource::Url { .. } => "URL",
            DependencySource::Unknown => "Unknown",
        }
    }

    pub fn description(&self) -> String {
        match self {
            DependencySource::PyPI => "Python Package Index".to_string(),
            DependencySource::Git { url, ref_spec } => {
                format!(
                    "Git Repository: {}\n{}",
                    url,
                    ref_spec.as_ref().map(|r| format!("Branch/Tag: {}", r)).unwrap_or_default()
                )
            }
            DependencySource::LocalPath { path, editable } => {
                format!(
                    "Local Path: {}\n{}",
                    path,
                    if *editable { "Editable Install" } else { "Standard Install" }
                )
            }
            DependencySource::Url { url } => format!("URL: {}", url),
            DependencySource::Unknown => "Unknown Source".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VersionStatus {
    Patch,
    Minor,
    Major,
    Prerelease,
    Unknown,
    UpToDate,
    Error,
    Vulnerable,
}

impl VersionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Patch => "Patch",
            Self::Minor => "Minor",
            Self::Major => "Major",
            Self::Prerelease => "Prerelease",
            Self::Unknown => "Unknown",
            Self::UpToDate => "Up-to-date",
            Self::Error => "Error",
            Self::Vulnerable => "Vulnerable",
        }
    }

    pub fn priority(&self) -> u8 {
        match self {
            Self::Vulnerable => 0,
            Self::Error => 1,
            Self::Major => 2,
            Self::Minor => 3,
            Self::Prerelease => 4,
            Self::Patch => 5,
            Self::Unknown => 6,
            Self::UpToDate => 7,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersionConstraint {
    Pinned(String),
    GreaterEqual(String),
    Less(String),
    Range(String, String),
    Compatible(String),
    Unspecified,
}

impl VersionConstraint {
    pub fn as_str(&self) -> String {
        match self {
            Self::Pinned(v) => format!("=={}", v),
            Self::GreaterEqual(v) => format!(">={}", v),
            Self::Less(v) => format!("<{}", v),
            Self::Range(low, high) => format!(">={},<{}", low, high),
            Self::Compatible(v) => format!("~={}", v),
            Self::Unspecified => String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RequirementsFile {
    pub path: String,
    pub packages: Vec<Package>,
    pub raw_lines: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityStatus {
    Vulnerable { cve_count: usize },
    Safe,
    Unknown,
}

impl SecurityStatus {
    pub fn is_vulnerable(&self) -> bool {
        matches!(self, SecurityStatus::Vulnerable { .. })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAdvisory {
    pub id: String,
    pub title: String,
    pub severity: Severity,
    pub affected_versions: Vec<String>,
    pub fixed_version: Option<String>,
    pub url: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Critical => "CRITICAL",
            Severity::High => "HIGH",
            Severity::Medium => "MEDIUM",
            Severity::Low => "LOW",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Changelog {
    pub version: String,
    pub release_date: String,
    pub changes: Vec<String>,
    pub breaking_changes: Vec<String>,
    pub deprecated: Vec<String>,
    pub security_fixes: Vec<String>,
}

impl Changelog {
    pub fn has_breaking_changes(&self) -> bool {
        !self.breaking_changes.is_empty()
    }

    pub fn risk_level(&self) -> &'static str {
        if self.has_breaking_changes() {
            "HIGH"
        } else if !self.security_fixes.is_empty() {
            "LOW"
        } else if !self.deprecated.is_empty() {
            "MEDIUM"
        } else {
            "LOW"
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopularityData {
    pub downloads_last_month: u64,
    pub downloads_trend: Vec<(String, u64)>,
    pub weekly_downloads: u64,
    pub package_rank: Option<usize>,
}

#[derive(Debug)]
pub struct UpgradeStats {
    pub total: usize,
    pub patch_available: usize,
    pub minor_available: usize,
    pub major_available: usize,
    pub up_to_date: usize,
    pub errors: usize,
    pub vulnerable: usize,
    pub conflicts: usize,
}

impl UpgradeStats {
    pub fn new(packages: &[Package]) -> Self {
        let mut stats = Self {
            total: packages.len(),
            patch_available: 0,
            minor_available: 0,
            major_available: 0,
            up_to_date: 0,
            errors: 0,
            vulnerable: 0,
            conflicts: 0,
        };

        for pkg in packages {
            match pkg.status {
                VersionStatus::Patch => stats.patch_available += 1,
                VersionStatus::Minor => stats.minor_available += 1,
                VersionStatus::Major => stats.major_available += 1,
                VersionStatus::UpToDate => stats.up_to_date += 1,
                VersionStatus::Error => stats.errors += 1,
                VersionStatus::Vulnerable => stats.vulnerable += 1,
                _ => {}
            }
        }

        stats
    }

    pub fn total_upgradable(&self) -> usize {
        self.patch_available + self.minor_available + self.major_available
    }
}

#[derive(Debug, Clone)]
pub struct UpgradeSimulation {
    pub packages_to_upgrade: usize,
    pub major_changes: usize,
    pub conflicts_detected: usize,
    pub security_fixes: usize,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl RiskLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            RiskLevel::Low => "LOW",
            RiskLevel::Medium => "MEDIUM",
            RiskLevel::High => "HIGH",
            RiskLevel::Critical => "CRITICAL",
        }
    }
}

pub fn compare_versions(current: &str, latest: &str) -> VersionStatus {
    match (semver::Version::parse(current), semver::Version::parse(latest)) {
        (Ok(curr), Ok(latest_ver)) => {
            if latest_ver <= curr {
                VersionStatus::UpToDate
            } else if latest_ver.major > curr.major {
                VersionStatus::Major
            } else if latest_ver.minor > curr.minor {
                VersionStatus::Minor
            } else {
                VersionStatus::Patch
            }
        }
        _ => {
            if latest <= current {
                VersionStatus::UpToDate
            } else {
                VersionStatus::Unknown
            }
        }
    }
}
