use crate::models::{Package, UpgradeStats, VersionStatus};
use crate::pypi::PyPIClient;
use crate::security::SecurityChecker;
use crate::changelog::ChangelogFetcher;
use crate::popularity::PopularityChecker;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    Loading,
    Display,
    Search,
    Confirm,
    Upgrading,
    Done,
    GraphView,
    ChangelogView,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortBy {
    Name,
    Status,
    Current,
    Latest,
    Popularity,
}

pub struct App {
    pub mode: AppMode,
    pub requirements_path: String,
    pub packages: Vec<Package>,
    pub filtered_packages: Vec<usize>,
    pub selected_index: usize,
    pub search_query: String,
    pub stats: UpgradeStats,
    pub sort_by: SortBy,
    pub dry_run: bool,
    pub loading_message: String,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub pypi_client: PyPIClient,
    pub security_checker: SecurityChecker,
    pub changelog_fetcher: ChangelogFetcher,
    pub popularity_checker: PopularityChecker,
    pub backup_path: Option<String>,
    pub lock_file_path: Option<String>,
}

impl App {
    pub fn new(requirements_path: String) -> Self {
        Self {
            mode: AppMode::Loading,
            requirements_path,
            packages: Vec::new(),
            filtered_packages: Vec::new(),
            selected_index: 0,
            search_query: String::new(),
            stats: UpgradeStats {
                total: 0,
                patch_available: 0,
                minor_available: 0,
                major_available: 0,
                up_to_date: 0,
                errors: 0,
                vulnerable: 0,
                conflicts: 0,
            },
            sort_by: SortBy::Status,
            dry_run: false,
            loading_message: "Parsing requirements.txt...".to_string(),
            error_message: None,
            success_message: None,
            pypi_client: PyPIClient::new(),
            security_checker: SecurityChecker::new(),
            changelog_fetcher: ChangelogFetcher::new(),
            popularity_checker: PopularityChecker::new(),
            backup_path: None,
            lock_file_path: None,
        }
    }

    pub fn set_packages(&mut self, packages: Vec<Package>) {
        self.packages = packages;
        self.refresh_filtered_packages();
        self.update_stats();
    }

    pub fn refresh_filtered_packages(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_packages = (0..self.packages.len()).collect();
        } else {
            let matcher = SkimMatcherV2::default();
            self.filtered_packages = self
                .packages
                .iter()
                .enumerate()
                .filter_map(|(idx, pkg)| {
                    if matcher.fuzzy_match(&pkg.name, &self.search_query).is_some() {
                        Some(idx)
                    } else {
                        None
                    }
                })
                .collect();
        }

        self.selected_index = 0;
    }

    pub fn update_stats(&mut self) {
        self.stats = UpgradeStats::new(&self.packages);
    }

    pub fn apply_sort(&mut self) {
        match self.sort_by {
            SortBy::Name => {
                self.packages.sort_by(|a, b| a.name.cmp(&b.name));
            }
            SortBy::Status => {
                self.packages.sort_by(|a, b| {
                    let a_priority = a.status.priority();
                    let b_priority = b.status.priority();
                    a_priority.cmp(&b_priority)
                });
            }
            SortBy::Current => {
                self.packages
                    .sort_by(|a, b| a.current_version.cmp(&b.current_version));
            }
            SortBy::Latest => {
                let default_version = "0.0.0".to_string();
                self.packages.sort_by(|a, b| {
                    let a_latest = a.latest_version.as_ref().unwrap_or(&default_version);
                    let b_latest = b.latest_version.as_ref().unwrap_or(&default_version);
                    a_latest.cmp(b_latest)
                });
            }
            SortBy::Popularity => {
                self.packages.sort_by(|a, b| {
                    let a_pop = a.popularity.as_ref().map(|p| p.weekly_downloads).unwrap_or(0);
                    let b_pop = b.popularity.as_ref().map(|p| p.weekly_downloads).unwrap_or(0);
                    b_pop.cmp(&a_pop)
                });
            }
        }
        self.refresh_filtered_packages();
    }

    pub fn get_selected_package(&mut self) -> Option<&mut Package> {
        self.filtered_packages
            .get(self.selected_index)
            .and_then(|&idx| self.packages.get_mut(idx))
    }

    pub fn get_selected_package_ref(&self) -> Option<&Package> {
        self.filtered_packages
            .get(self.selected_index)
            .and_then(|&idx| self.packages.get(idx))
    }

    pub fn toggle_selected(&mut self) {
        if let Some(pkg) = self.get_selected_package() {
            pkg.selected = !pkg.selected;
        }
    }

    pub fn select_all(&mut self) {
        for idx in self.filtered_packages.clone() {
            if let Some(pkg) = self.packages.get_mut(idx) {
                if pkg.latest_version.is_some() {
                    pkg.selected = true;
                }
            }
        }
    }

    pub fn deselect_all(&mut self) {
        for pkg in &mut self.packages {
            pkg.selected = false;
        }
    }

    pub fn select_all_major(&mut self) {
        for idx in self.filtered_packages.clone() {
            if let Some(pkg) = self.packages.get_mut(idx) {
                if pkg.status == VersionStatus::Major {
                    pkg.selected = true;
                }
            }
        }
    }

    pub fn select_all_minor(&mut self) {
        for idx in self.filtered_packages.clone() {
            if let Some(pkg) = self.packages.get_mut(idx) {
                if pkg.status == VersionStatus::Minor {
                    pkg.selected = true;
                }
            }
        }
    }

    pub fn select_all_patch(&mut self) {
        for idx in self.filtered_packages.clone() {
            if let Some(pkg) = self.packages.get_mut(idx) {
                if pkg.status == VersionStatus::Patch {
                    pkg.selected = true;
                }
            }
        }
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_index < self.filtered_packages.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    pub fn page_up(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(10);
    }

    pub fn page_down(&mut self) {
        let max = self.filtered_packages.len().saturating_sub(1);
        self.selected_index = std::cmp::min(self.selected_index + 10, max);
    }

    pub fn home(&mut self) {
        self.selected_index = 0;
    }

    pub fn end(&mut self) {
        self.selected_index = self.filtered_packages.len().saturating_sub(1);
    }

    pub fn count_selected(&self) -> usize {
        self.packages.iter().filter(|p| p.selected).count()
    }

    pub fn has_upgradable_packages(&self) -> bool {
        self.packages.iter().any(|p| p.latest_version.is_some())
    }

    pub fn get_selected_packages(&self) -> Vec<&Package> {
        self.packages.iter().filter(|p| p.selected).collect()
    }

    pub fn clear_messages(&mut self) {
        self.error_message = None;
        self.success_message = None;
    }

    pub fn set_error(&mut self, error: String) {
        self.error_message = Some(error);
    }

    pub fn set_success(&mut self, message: String) {
        self.success_message = Some(message);
    }
}
