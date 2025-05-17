/// Initialize the logger
#[allow(dead_code)]
pub fn init() {
    env_logger::init();
}

// Re-export log macros
#[allow(unused_imports)]
pub use log::debug;
#[allow(unused_imports)]
pub use log::error;
#[allow(unused_imports)]
pub use log::info;
#[allow(unused_imports)]
pub use log::trace;
#[allow(unused_imports)]
pub use log::warn;
