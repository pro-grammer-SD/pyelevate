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
        }
    }

    pub fn color_code(&self) -> &'static str {
        match self {
            Self::UpToDate => "\x1b[32m",
            Self::Patch => "\x1b[33m",
            Self::Minor => "\x1b[36m",
            Self::Major => "\x1b[31m",
            Self::Prerelease => "\x1b[35m",
            Self::Unknown => "\x1b[90m",
            Self::Error => "\x1b[91m",
        }
    }

    pub fn priority(&self) -> u8 {
        match self {
            Self::Error => 0,
            Self::Major => 1,
            Self::Minor => 2,
            Self::Prerelease => 3,
            Self::Patch => 4,
            Self::Unknown => 5,
            Self::UpToDate => 6,
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

#[derive(Debug)]
pub struct UpgradeStats {
    pub total: usize,
    pub patch_available: usize,
    pub minor_available: usize,
    pub major_available: usize,
    pub up_to_date: usize,
    pub errors: usize,
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
        };

        for pkg in packages {
            match pkg.status {
                VersionStatus::Patch => stats.patch_available += 1,
                VersionStatus::Minor => stats.minor_available += 1,
                VersionStatus::Major => stats.major_available += 1,
                VersionStatus::UpToDate => stats.up_to_date += 1,
                VersionStatus::Error => stats.errors += 1,
                _ => {}
            }
        }

        stats
    }

    pub fn total_upgradable(&self) -> usize {
        self.patch_available + self.minor_available + self.major_available
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
