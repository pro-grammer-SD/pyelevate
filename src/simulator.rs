use crate::models::{Package, UpgradeSimulation, RiskLevel, VersionStatus};
use crate::resolver::DependencyResolver;

pub struct UpgradeSimulator {
    resolver: DependencyResolver,
}

impl UpgradeSimulator {
    pub fn new() -> Self {
        Self {
            resolver: DependencyResolver::new(),
        }
    }

    pub fn simulate_upgrade(&self, packages: &[Package]) -> UpgradeSimulation {
        let selected = packages.iter().filter(|p| p.selected).collect::<Vec<_>>();
        
        let packages_to_upgrade = selected.len();
        
        let major_changes = selected
            .iter()
            .filter(|p| p.status == VersionStatus::Major)
            .count();

        let security_fixes = selected
            .iter()
            .filter(|p| matches!(p.status, VersionStatus::Vulnerable))
            .count();

        let conflicts = self.resolver.detect_conflicts(packages).len();

        let risk_level = calculate_risk_level(major_changes, conflicts, security_fixes, packages_to_upgrade);

        UpgradeSimulation {
            packages_to_upgrade,
            major_changes,
            conflicts_detected: conflicts,
            security_fixes,
            risk_level,
        }
    }

    pub fn generate_report(&self, packages: &[Package]) -> String {
        let simulation = self.simulate_upgrade(packages);
        
        let mut report = String::new();
        report.push_str("╔════════════════════════════════════════╗\n");
        report.push_str("║     UPGRADE SIMULATION REPORT          ║\n");
        report.push_str("╚════════════════════════════════════════╝\n\n");

        report.push_str(&format!(
            "📦 Packages to upgrade:     {}\n",
            simulation.packages_to_upgrade
        ));
        report.push_str(&format!(
            "🔴 Major changes:           {}\n",
            simulation.major_changes
        ));
        report.push_str(&format!(
            "⚠️  Conflicts detected:      {}\n",
            simulation.conflicts_detected
        ));
        report.push_str(&format!(
            "🔒 Security fixes:          {}\n",
            simulation.security_fixes
        ));
        report.push_str(&format!(
            "📊 Overall Risk:            {}\n\n",
            simulation.risk_level.as_str()
        ));

        report
    }
}

fn calculate_risk_level(major: usize, conflicts: usize, security: usize, total: usize) -> RiskLevel {
    if conflicts > 0 && major > 0 {
        RiskLevel::Critical
    } else if major > (total / 2) {
        RiskLevel::High
    } else if major > 0 || conflicts > 0 {
        RiskLevel::Medium
    } else if security > 0 {
        RiskLevel::Low
    } else {
        RiskLevel::Low
    }
}

impl Default for UpgradeSimulator {
    fn default() -> Self {
        Self::new()
    }
}
