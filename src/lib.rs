pub mod app;
pub mod models;
pub mod parser;
pub mod pypi;
pub mod styles;
pub mod ui;
pub mod upgrade;

pub use app::App;
pub use models::{Package, VersionStatus};
pub use parser::parse_requirements;
pub use pypi::PyPIClient;
pub use upgrade::{UpgradeManager, UpgradeResult};

pub const APP_NAME: &str = "PyElevate";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
