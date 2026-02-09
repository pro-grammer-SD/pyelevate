pub mod app;
pub mod models;
pub mod parser;
pub mod pypi;
pub mod styles;
pub mod ui;
pub mod upgrade;
pub mod security;
pub mod changelog;
pub mod popularity;
pub mod resolver;
pub mod simulator;
pub mod panels;

pub use app::App;
pub use models::{Package, VersionStatus};
pub use parser::parse_requirements;
pub use pypi::PyPIClient;
pub use upgrade::{UpgradeManager, UpgradeResult};
pub use security::SecurityChecker;
pub use changelog::ChangelogFetcher;
pub use popularity::PopularityChecker;
pub use resolver::DependencyResolver;
pub use simulator::UpgradeSimulator;

pub const APP_NAME: &str = "PyElevate";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_AUTHOR: &str = "Soumalya Das";
