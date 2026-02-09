use crate::models::{Package, RequirementsFile, VersionConstraint, VersionStatus, DependencySource};
use anyhow::{anyhow, Result};
use regex::Regex;
use std::fs;
use std::path::Path;
use url::Url;

pub fn parse_requirements<P: AsRef<Path>>(path: P) -> Result<RequirementsFile> {
    let content = fs::read_to_string(&path)?;
    let path_str = path.as_ref().to_string_lossy().to_string();
    
    let mut packages = Vec::new();
    let raw_lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    for line in content.lines() {
        let line = line.trim();
        
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Ok(package) = parse_requirement_line(line) {
            packages.push(package);
        }
    }

    packages.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(RequirementsFile {
        path: path_str,
        packages,
        raw_lines,
    })
}

fn parse_requirement_line(line: &str) -> Result<Package> {
    let line = line.split('#').next().unwrap_or(line).trim();
    
    if line.is_empty() {
        return Err(anyhow!("Empty line"));
    }

    if let Some(rest) = line.strip_prefix("git+") {
        return parse_git_requirement(rest);
    }

    if let Some(rest) = line.strip_prefix("-e") {
        return parse_editable_requirement(rest.trim());
    }

    if line.starts_with("http://") || line.starts_with("https://") || line.starts_with("file://") {
        return parse_url_requirement(line);
    }

    parse_pypi_requirement(line)
}

fn parse_pypi_requirement(line: &str) -> Result<Package> {
    let (name_part, version_spec) = extract_version_spec(line)?;
    let (name, extras) = extract_extras(&name_part);

    let (constraint, current_version) = parse_version_spec(&version_spec);

    Ok(Package {
        name: name.to_lowercase(),
        current_version,
        latest_version: None,
        status: VersionStatus::Unknown,
        selected: false,
        extras,
        constraint,
        error: None,
        source: DependencySource::PyPI,
        security_status: crate::models::SecurityStatus::Unknown,
        changelog: None,
        popularity: None,
        dependencies: Vec::new(),
    })
}

fn parse_git_requirement(rest: &str) -> Result<Package> {
    let parts: Vec<&str> = rest.split('@').collect();
    let url = parts[0].to_string();
    let ref_spec = parts.get(1).map(|s| s.to_string());

    let name = extract_package_name_from_git(&url)
        .unwrap_or_else(|| format!("git-{}", uuid::Uuid::new_v4().to_string()[0..8].to_string()));

    Ok(Package {
        name: name.to_lowercase(),
        current_version: "git-source".to_string(),
        latest_version: None,
        status: VersionStatus::Unknown,
        selected: false,
        extras: Vec::new(),
        constraint: VersionConstraint::Unspecified,
        error: None,
        source: DependencySource::Git { url, ref_spec },
        security_status: crate::models::SecurityStatus::Unknown,
        changelog: None,
        popularity: None,
        dependencies: Vec::new(),
    })
}

fn parse_editable_requirement(rest: &str) -> Result<Package> {
    let path = rest.trim_start_matches('-').trim();

    let name = Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("local-{}", uuid::Uuid::new_v4().to_string()[0..8].to_string()));

    Ok(Package {
        name: name.to_lowercase(),
        current_version: "local".to_string(),
        latest_version: None,
        status: VersionStatus::Unknown,
        selected: false,
        extras: Vec::new(),
        constraint: VersionConstraint::Unspecified,
        error: None,
        source: DependencySource::LocalPath {
            path: path.to_string(),
            editable: true,
        },
        security_status: crate::models::SecurityStatus::Unknown,
        changelog: None,
        popularity: None,
        dependencies: Vec::new(),
    })
}

fn parse_url_requirement(line: &str) -> Result<Package> {
    if let Ok(url) = Url::parse(line) {
        let name = url
            .path_segments()
            .and_then(|mut segments| segments.next())
            .and_then(|s| s.split('.').next())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("url-{}", uuid::Uuid::new_v4().to_string()[0..8].to_string()));

        Ok(Package {
            name: name.to_lowercase(),
            current_version: "url-source".to_string(),
            latest_version: None,
            status: VersionStatus::Unknown,
            selected: false,
            extras: Vec::new(),
            constraint: VersionConstraint::Unspecified,
            error: None,
            source: DependencySource::Url {
                url: line.to_string(),
            },
            security_status: crate::models::SecurityStatus::Unknown,
            changelog: None,
            popularity: None,
            dependencies: Vec::new(),
        })
    } else {
        Err(anyhow!("Invalid URL requirement"))
    }
}

fn extract_version_spec(line: &str) -> Result<(String, String)> {
    let operators = vec!["==", ">=", "<=", "~=", ">", "<", "!="];
    
    for op in operators {
        if let Some(pos) = line.find(op) {
            let name = line[..pos].trim().to_string();
            let spec = line[pos..].to_string();
            return Ok((name, spec));
        }
    }

    Ok((line.to_string(), String::new()))
}

fn extract_extras(name_part: &str) -> (String, Vec<String>) {
    if let Some(bracket_pos) = name_part.find('[') {
        let name = name_part[..bracket_pos].to_string();
        let extras: Vec<String> = name_part[bracket_pos + 1..]
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        (name, extras)
    } else {
        (name_part.to_string(), Vec::new())
    }
}

fn parse_version_spec(spec: &str) -> (VersionConstraint, String) {
    let spec = spec.trim();

    if spec.is_empty() {
        return (VersionConstraint::Unspecified, "0.0.0".to_string());
    }

    if let Some(version) = spec.strip_prefix("==") {
        let version = version.trim().to_string();
        (
            VersionConstraint::Pinned(version.clone()),
            normalize_version(&version),
        )
    } else if let Some(version) = spec.strip_prefix(">=") {
        let version = version.trim().to_string();
        (
            VersionConstraint::GreaterEqual(version.clone()),
            normalize_version(&version),
        )
    } else if let Some(version) = spec.strip_prefix("~=") {
        let version = version.trim().to_string();
        (
            VersionConstraint::Compatible(version.clone()),
            normalize_version(&version),
        )
    } else if let Some(version) = spec.strip_prefix("<") {
        let version = version.trim().to_string();
        (
            VersionConstraint::Less(version.clone()),
            "0.0.0".to_string(),
        )
    } else {
        (VersionConstraint::Unspecified, "0.0.0".to_string())
    }
}

fn normalize_version(version: &str) -> String {
    let re = Regex::new(r"^(\d+)\.(\d+)\.(\d+)(.*)$").unwrap();
    
    if let Some(caps) = re.captures(version) {
        format!(
            "{}.{}.{}{}",
            &caps[1], &caps[2], &caps[3],
            caps.get(4).map(|m| m.as_str()).unwrap_or("")
        )
    } else if let Some(caps) = Regex::new(r"^(\d+)\.(\d+)$").unwrap().captures(version) {
        format!("{}.{}.0", &caps[1], &caps[2])
    } else if let Some(caps) = Regex::new(r"^(\d+)$").unwrap().captures(version) {
        format!("{}.0.0", &caps[1])
    } else {
        version.to_string()
    }
}

fn extract_package_name_from_git(url: &str) -> Option<String> {
    url.split('/')
        .last()
        .and_then(|name| name.strip_suffix(".git"))
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pinned_version() {
        let pkg = parse_pypi_requirement("requests==2.28.1").unwrap();
        assert_eq!(pkg.name, "requests");
        assert_eq!(pkg.current_version, "2.28.1");
    }

    #[test]
    fn test_parse_with_extras() {
        let pkg = parse_pypi_requirement("requests[security,socks]==2.28.1").unwrap();
        assert_eq!(pkg.name, "requests");
        assert_eq!(pkg.extras.len(), 2);
    }

    #[test]
    fn test_parse_git() {
        let pkg = parse_git_requirement("https://github.com/user/repo.git@main").unwrap();
        assert_eq!(pkg.name, "repo");
        assert!(matches!(pkg.source, DependencySource::Git { .. }));
    }
}
