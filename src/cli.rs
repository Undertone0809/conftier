use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use clap::{Parser, Subcommand};

use crate::core::{find_project_root, get_project_config_path, get_user_config_path};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize project configuration template
    InitProject {
        /// Name of the configuration to initialize
        config_name: String,

        /// Project path
        #[arg(short, long)]
        path: Option<PathBuf>,
    },

    /// Show current effective configuration and its sources
    ShowConfig {
        /// Name of the configuration to show
        config_name: String,
    },

    /// Set a configuration value
    SetConfig {
        /// Name of the configuration to modify
        config_name: String,

        /// Config key to set (dot notation)
        #[arg(short, long)]
        key: String,

        /// Value to set
        #[arg(short, long)]
        value: String,

        /// Update project config instead of user config
        #[arg(short, long)]
        project: bool,
    },
}

/// Initialize project configuration template
pub fn init_project(
    config_name: &str,
    path: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let project_path = path.unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let config_dir = project_path.join(format!(".{}", config_name));
    let config_file = config_dir.join("config.yaml");

    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)?;
        println!("Created directory: {}", config_dir.display());
    }

    if !config_file.exists() {
        // Simple empty config as template
        let empty_config = HashMap::<String, serde_yaml::Value>::new();
        let yaml_str = serde_yaml::to_string(&empty_config)?;
        std::fs::write(&config_file, yaml_str)?;
        println!("Created project config template: {}", config_file.display());
    } else {
        println!("Project config already exists: {}", config_file.display());
    }

    Ok(())
}

/// Show current effective configuration and its sources
pub fn show_config(config_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let user_path = get_user_config_path(config_name);
    let project_root = find_project_root();
    let project_path = get_project_config_path(
        config_name,
        project_root.as_ref().map(|p| p.to_str().unwrap()),
    );

    if !user_path.exists() && (project_path.is_none() || !project_path.as_ref().unwrap().exists()) {
        println!(
            "No configuration files found for framework '{}'",
            config_name
        );
        return Ok(());
    }

    if user_path.exists() {
        println!("User config ({}):", user_path.display());
        let contents = std::fs::read_to_string(&user_path)?;
        match serde_yaml::from_str::<serde_yaml::Value>(&contents) {
            Ok(config) => {
                println!(
                    "{}",
                    serde_yaml::to_string(&config).unwrap_or_else(|_| "Invalid YAML".to_string())
                );
            }
            Err(_) => {
                println!("(empty or invalid YAML)");
            }
        }
    } else {
        println!("No user config found at {}", user_path.display());
    }

    if let Some(project_path) = project_path {
        if project_path.exists() {
            println!("Project config ({}):", project_path.display());
            let contents = std::fs::read_to_string(&project_path)?;
            match serde_yaml::from_str::<serde_yaml::Value>(&contents) {
                Ok(config) => {
                    println!(
                        "{}",
                        serde_yaml::to_string(&config)
                            .unwrap_or_else(|_| "Invalid YAML".to_string())
                    );
                }
                Err(_) => {
                    println!("(empty or invalid YAML)");
                }
            }
        } else {
            println!("No project config found");
        }
    } else {
        println!("No project config found");
    }

    Ok(())
}

/// Parse the input value to an appropriate serde_yaml::Value
fn parse_value(value: &str) -> serde_yaml::Value {
    if value.eq_ignore_ascii_case("true") {
        serde_yaml::Value::Bool(true)
    } else if value.eq_ignore_ascii_case("false") {
        serde_yaml::Value::Bool(false)
    } else if let Ok(num) = i64::from_str(value) {
        serde_yaml::Value::Number(num.into())
    } else if let Ok(num) = f64::from_str(value) {
        // Try to convert via serialization to avoid precision issues
        match serde_yaml::to_value(num) {
            Ok(yaml_value) => yaml_value,
            Err(_) => serde_yaml::Value::String(value.to_string()),
        }
    } else {
        serde_yaml::Value::String(value.to_string())
    }
}

/// Set a configuration value
pub fn set_config(
    config_name: &str,
    key: &str,
    value: &str,
    project: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = if project {
        let project_root = match find_project_root() {
            Some(root) => root,
            None => {
                println!("No project root found. Cannot update project configuration.");
                return Ok(());
            }
        };

        let config_path =
            match get_project_config_path(config_name, Some(project_root.to_str().unwrap())) {
                Some(path) => path,
                None => {
                    println!("No project configuration path could be determined.");
                    return Ok(());
                }
            };

        if let Some(parent) = config_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        config_path
    } else {
        let config_path = get_user_config_path(config_name);
        if let Some(parent) = config_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        config_path
    };

    let mut existing_config: HashMap<String, serde_yaml::Value> = if config_path.exists() {
        match std::fs::read_to_string(&config_path) {
            Ok(contents) => serde_yaml::from_str(&contents).unwrap_or_else(|_| HashMap::new()),
            Err(_) => HashMap::new(),
        }
    } else {
        HashMap::new()
    };

    // Handle nested keys with dot notation
    let key_parts: Vec<&str> = key.split('.').collect();

    // Create a nested HashMap structure based on the key parts
    if key_parts.len() == 1 {
        // Direct update for simple keys
        existing_config.insert(key.to_string(), parse_value(value));
    } else {
        // Handle nested structure recursively
        let parsed_value = parse_value(value);
        update_nested_value(&mut existing_config, &key_parts, parsed_value);
    }

    // Write back to file
    let yaml_str = serde_yaml::to_string(&existing_config)?;
    std::fs::write(&config_path, yaml_str)?;

    let config_type = if project { "project" } else { "user" };
    println!("Updated {} config: {} = {}", config_type, key, value);

    Ok(())
}

// Helper function to update a nested value in the configuration
fn update_nested_value(
    config: &mut HashMap<String, serde_yaml::Value>,
    key_parts: &[&str],
    value: serde_yaml::Value,
) {
    if key_parts.is_empty() {
        return;
    }

    if key_parts.len() == 1 {
        config.insert(key_parts[0].to_string(), value);
        return;
    }

    let current_key = key_parts[0].to_string();
    let remaining_keys = &key_parts[1..];

    // Get or create the nested map
    let nested = match config.get_mut(&current_key) {
        Some(serde_yaml::Value::Mapping(mapping)) => {
            // Convert existing mapping to HashMap
            let mut hashmap = HashMap::new();
            for (k, v) in mapping.iter() {
                if let serde_yaml::Value::String(key_str) = k {
                    hashmap.insert(key_str.clone(), v.clone());
                }
            }
            hashmap
        }
        _ => {
            // Create new HashMap
            HashMap::new()
        }
    };

    let mut nested_map = nested;
    update_nested_value(&mut nested_map, remaining_keys, value);

    // Convert back to serde_yaml::Value
    let mapping = serde_yaml::Mapping::from_iter(
        nested_map
            .into_iter()
            .map(|(k, v)| (serde_yaml::Value::String(k), v)),
    );
    config.insert(current_key, serde_yaml::Value::Mapping(mapping));
}

/// Run the CLI application
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::InitProject { config_name, path } => {
            init_project(config_name, path.clone())?;
        }
        Commands::ShowConfig { config_name } => {
            show_config(config_name)?;
        }
        Commands::SetConfig {
            config_name,
            key,
            value,
            project,
        } => {
            set_config(config_name, key, value, *project)?;
        }
    }

    Ok(())
}
