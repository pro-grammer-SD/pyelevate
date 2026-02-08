use crate::models::{Package, RequirementsFile, VersionConstraint, VersionStatus};
use anyhow::{anyhow, Result};
use regex::Regex;
use std::fs;
use std::path::Path;

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
    })
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
    } else if let Some(caps) = Regex::new(r"^(\d+)\.(\d+)$")
        .unwrap()
        .captures(version)
    {
        format!("{}.{}.0", &caps[1], &caps[2])
    } else if let Some(caps) = Regex::new(r"^(\d+)$").unwrap().captures(version) {
        format!("{}.0.0", &caps[1])
    } else {
        version.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pinned_version() {
        let pkg = parse_requirement_line("requests==2.28.1").unwrap();
        assert_eq!(pkg.name, "requests");
        assert_eq!(pkg.current_version, "2.28.1");
    }

    #[test]
    fn test_parse_with_extras() {
        let pkg = parse_requirement_line("requests[security,socks]==2.28.1").unwrap();
        assert_eq!(pkg.name, "requests");
        assert_eq!(pkg.extras.len(), 2);
    }

    #[test]
    fn test_parse_with_comment() {
        let pkg = parse_requirement_line("django==3.2  # Web framework").unwrap();
        assert_eq!(pkg.name, "django");
        assert_eq!(pkg.current_version, "3.2.0");
    }
}
