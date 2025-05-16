mod cli;
mod core;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();

    // Run CLI
    cli::run()
}
