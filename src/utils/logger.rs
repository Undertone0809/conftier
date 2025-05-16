/// Initialize the logger
pub fn init() {
    env_logger::init();
}

// Re-export log macros
pub use log::debug;
pub use log::error;
pub use log::info;
pub use log::trace;
pub use log::warn;
