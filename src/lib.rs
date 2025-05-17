pub mod cli;
pub mod core;
pub mod utils;

pub use core::{ConfigManager, ConfigModel, SchemaType};
pub use utils::logger;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
