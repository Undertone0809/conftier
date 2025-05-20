use conftier::ConfigManager;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define configuration structure
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct AppConfig {
    // Application settings
    app: AppSettings,
    // Database settings
    database: DatabaseSettings,
    // Logging settings
    logging: LoggingSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct AppSettings {
    name: String,
    version: String,
    debug: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct DatabaseSettings {
    url: String,
    username: String,
    password: String,
    pool_size: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct LoggingSettings {
    level: String,
    file: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut config_manager: ConfigManager<AppConfig> = ConfigManager::new(
        "myapp", // Configuration name
        "1.0.0", // Version
        true,    // Auto-create user config
        true,    // Auto-create project config
    );

    config_manager.initialize();

    let config = config_manager.load();
    println!("Loaded config: {:?}", config);

    let mut user_updates = HashMap::new();
    user_updates.insert(
        "app".to_string(),
        serde_yaml::Value::Mapping(serde_yaml::Mapping::from_iter(vec![
            (
                serde_yaml::Value::String("name".to_string()),
                serde_yaml::Value::String("My Application".to_string()),
            ),
            (
                serde_yaml::Value::String("debug".to_string()),
                serde_yaml::Value::Bool(true),
            ),
        ])),
    );
    config_manager.update_user_config(user_updates);

    let mut project_updates = HashMap::new();
    project_updates.insert(
        "database".to_string(),
        serde_yaml::Value::Mapping(serde_yaml::Mapping::from_iter(vec![
            (
                serde_yaml::Value::String("url".to_string()),
                serde_yaml::Value::String("postgresql://localhost:5432/mydb".to_string()),
            ),
            (
                serde_yaml::Value::String("username".to_string()),
                serde_yaml::Value::String("admin".to_string()),
            ),
        ])),
    );
    config_manager.update_project_config(project_updates);

    let updated_config = config_manager.config();
    println!("Updated config: {:?}", updated_config);

    // Demonstrate how to access values in the configuration
    if let Some(db_url) = get_value::<String>(updated_config, "database.url") {
        println!("Database URL: {}", db_url);
    }

    if let Some(app_name) = get_value::<String>(updated_config, "app.name") {
        println!("Application Name: {}", app_name);
    }

    if let Some(debug_mode) = get_value::<bool>(updated_config, "app.debug") {
        println!("Debug Mode: {}", debug_mode);
    }

    Ok(())
}

// Helper function to get the value from the configuration at the specified path
fn get_value<T: for<'de> Deserialize<'de>>(config: &AppConfig, path: &str) -> Option<T> {
    let parts: Vec<&str> = path.split('.').collect();
    if parts.len() == 2 {
        let section = parts[0];
        let key = parts[1];

        let yaml_value = match section {
            "app" => match key {
                "name" => serde_yaml::to_value(&config.app.name).ok(),
                "version" => serde_yaml::to_value(&config.app.version).ok(),
                "debug" => serde_yaml::to_value(config.app.debug).ok(),
                _ => None,
            },
            "database" => match key {
                "url" => serde_yaml::to_value(&config.database.url).ok(),
                "username" => serde_yaml::to_value(&config.database.username).ok(),
                "password" => serde_yaml::to_value(&config.database.password).ok(),
                "pool_size" => serde_yaml::to_value(config.database.pool_size).ok(),
                _ => None,
            },
            "logging" => match key {
                "level" => serde_yaml::to_value(&config.logging.level).ok(),
                "file" => serde_yaml::to_value(&config.logging.file).ok(),
                _ => None,
            },
            _ => None,
        };

        if let Some(value) = yaml_value {
            return serde_yaml::from_value(value).ok();
        }
    }
    None
}
