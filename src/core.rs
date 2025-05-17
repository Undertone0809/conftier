use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use log::error;
use serde::{de::DeserializeOwned, Serialize};

/// Schema type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaType {
    /// Struct type
    Struct,
    /// HashMap type
    #[allow(dead_code)]
    HashMap,
}

/// ConfigModel represents a unified configuration model that wraps structs and hashmaps.
///
/// This struct provides a consistent interface for different types of configuration models,
/// handling serialization, and nested structure management.
#[derive(Debug, Clone)]
pub struct ConfigModel<T: Serialize + DeserializeOwned + Clone> {
    /// The schema type of this configuration model
    pub schema_type: SchemaType,
    /// The actual model instance
    model: T,
}

impl<T> ConfigModel<T>
where
    T: Serialize + DeserializeOwned + Clone + Default,
{
    /// Create a new ConfigModel from a schema type and data
    pub fn from_schema(data: Option<HashMap<String, serde_yaml::Value>>) -> Self {
        let model = match data {
            Some(data_map) => {
                // For serialization, convert the HashMap to a YAML string first
                let yaml_str = match serde_yaml::to_string(&data_map) {
                    Ok(s) => s,
                    Err(e) => {
                        error!("Failed to serialize data to YAML: {}", e);
                        return ConfigModel {
                            schema_type: SchemaType::Struct,
                            model: T::default(),
                        };
                    }
                };

                // Then deserialize from the YAML string
                match serde_yaml::from_str::<T>(&yaml_str) {
                    Ok(value) => value,
                    Err(e) => {
                        error!("Failed to create model from YAML: {}", e);
                        T::default()
                    }
                }
            }
            None => T::default(),
        };

        ConfigModel {
            schema_type: SchemaType::Struct,
            model,
        }
    }

    /// Get the underlying model instance
    pub fn model(&self) -> &T {
        &self.model
    }

    /// Convert the model to a HashMap
    pub fn to_dict(&self) -> HashMap<String, serde_yaml::Value> {
        match serde_yaml::to_value(&self.model) {
            Ok(serde_yaml::Value::Mapping(map)) => map
                .into_iter()
                .filter_map(|(k, v)| {
                    if let serde_yaml::Value::String(key_str) = k {
                        Some((key_str, v))
                    } else {
                        None
                    }
                })
                .collect(),
            _ => HashMap::new(),
        }
    }

    /// Update the model with new data
    pub fn update(&mut self, data: HashMap<String, serde_yaml::Value>) {
        let current = self.to_dict();
        let updated = deep_update(current, data);

        // Convert to YAML string and back
        if let Ok(yaml_str) = serde_yaml::to_string(&updated) {
            if let Ok(new_model) = serde_yaml::from_str::<T>(&yaml_str) {
                self.model = new_model;
            }
        }
    }

    /// Merge with another ConfigModel
    pub fn merge(&self, other: &Self) -> Self {
        let base_dict = self.to_dict();
        let other_dict = other.to_dict();

        let merged_dict = deep_update(base_dict, other_dict);

        // Convert to YAML string and back
        let model = match serde_yaml::to_string(&merged_dict) {
            Ok(yaml_str) => match serde_yaml::from_str::<T>(&yaml_str) {
                Ok(value) => value,
                Err(_) => self.model.clone(),
            },
            Err(_) => self.model.clone(),
        };

        ConfigModel {
            schema_type: self.schema_type,
            model,
        }
    }
}

/// ConfigManager handles loading, merging, and accessing configurations.
#[derive(Debug)]
pub struct ConfigManager<T>
where
    T: Serialize + DeserializeOwned + Clone + Default,
{
    config_name: String,
    #[allow(dead_code)]
    version: String,
    auto_create_user: bool,
    auto_create_project: bool,

    // Configuration instances
    config: Option<ConfigModel<T>>,
    default_config: Option<ConfigModel<T>>,
    user_config: Option<ConfigModel<T>>,
    project_config: Option<ConfigModel<T>>,

    // Configuration file paths
    user_config_path: PathBuf,
    #[allow(dead_code)]
    project_root: Option<PathBuf>,
    project_config_path: Option<PathBuf>,
}

impl<T> ConfigManager<T>
where
    T: Serialize + DeserializeOwned + Clone + Default,
{
    /// Initialize the configuration manager
    ///
    /// # Arguments
    /// * `config_name` - Framework name, used to determine config file paths
    /// * `version` - Configuration schema version
    /// * `auto_create_user` - Whether to automatically create user config file if not exists
    /// * `auto_create_project` - Whether to automatically create project config file if not exists
    #[allow(dead_code)]
    pub fn new(
        config_name: &str,
        version: &str,
        auto_create_user: bool,
        auto_create_project: bool,
    ) -> Self {
        let user_config_path = get_user_config_path(config_name);
        let project_root = find_project_root();
        let project_config_path = get_project_config_path(
            config_name,
            project_root.as_ref().map(|p| p.to_str().unwrap()),
        );

        ConfigManager {
            config_name: config_name.to_owned(),
            version: version.to_owned(),
            auto_create_user,
            auto_create_project,

            config: None,
            default_config: None,
            user_config: None,
            project_config: None,

            user_config_path,
            project_root,
            project_config_path: project_config_path.clone(),
        }
    }

    /// Initialize after construction
    #[allow(dead_code)]
    pub fn initialize(&mut self) {
        if self.auto_create_user {
            self.create_user_config_template();
        }

        if self.auto_create_project && self.project_config_path.is_some() {
            self.create_project_config_template(None);
        }
    }

    /// Get default configuration as a HashMap
    fn get_default_dict(&self) -> HashMap<String, serde_yaml::Value> {
        let default_model = ConfigModel::<T>::from_schema(None);
        default_model.to_dict()
    }

    /// Load and merge all configuration levels
    pub fn load(&mut self) -> &T {
        let default_config_model: ConfigModel<T> = ConfigModel::from_schema(None);

        let user_config_exists = self.user_config_path.exists();
        let project_config_exists = self
            .project_config_path
            .as_ref()
            .is_some_and(|p| p.exists());

        if !user_config_exists && self.auto_create_user {
            self.create_user_config_template();
        }

        if !project_config_exists && self.auto_create_project && self.project_config_path.is_some()
        {
            self.create_project_config_template(None);
        }

        if !user_config_exists && !project_config_exists {
            let mut error_message = String::from("Configuration files not found. ");

            if let Some(path) = &self.project_config_path {
                error_message.push_str(&format!("Project config missing at {}. ", path.display()));
            }

            error_message.push_str(&format!(
                "User config missing at {}. ",
                self.user_config_path.display()
            ));
            error_message.push_str("Use create_user_config_template() or create_project_config_template() to create them, ");
            error_message.push_str("or set auto_create_user=True or auto_create_project=True.");

            panic!("{}", error_message);
        }

        let user_config_model = self.load_config_from_path(&self.user_config_path);
        let project_config_model = match &self.project_config_path {
            Some(path) => self.load_config_from_path(path),
            None => None,
        };

        let mut merged_config_model = default_config_model.clone();
        if let Some(user_config) = &user_config_model {
            merged_config_model = merged_config_model.merge(user_config);
        }

        if let Some(project_config) = &project_config_model {
            merged_config_model = merged_config_model.merge(project_config);
        }

        // Store ConfigModel instances
        self.default_config = Some(default_config_model);
        self.user_config = user_config_model;
        self.project_config = project_config_model;
        self.config = Some(merged_config_model);

        self.config.as_ref().unwrap().model()
    }

    /// Get the current merged configuration
    #[allow(dead_code)]
    pub fn config(&mut self) -> &T {
        if self.config.is_none() {
            return self.load();
        }
        self.config.as_ref().unwrap().model()
    }

    /// Get the default configuration
    #[allow(dead_code)]
    pub fn get_default_config(&mut self) -> &T {
        if self.default_config.is_none() {
            self.default_config = Some(ConfigModel::from_schema(None));
        }
        self.default_config.as_ref().unwrap().model()
    }

    /// Helper method to load configuration from a path
    #[allow(dead_code)]
    fn load_config_from_path(&self, config_path: &Path) -> Option<ConfigModel<T>> {
        if !config_path.exists() {
            return None;
        }

        let mut file = match File::open(config_path) {
            Ok(file) => file,
            Err(e) => {
                error!(
                    "Failed to open config file {}: {}",
                    config_path.display(),
                    e
                );
                return None;
            }
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            error!(
                "Failed to read config file {}: {}",
                config_path.display(),
                e
            );
            return None;
        }

        let config_dict: HashMap<String, serde_yaml::Value> = match serde_yaml::from_str(&contents)
        {
            Ok(dict) => dict,
            Err(e) => {
                error!(
                    "Failed to parse config file {}: {}",
                    config_path.display(),
                    e
                );
                return None;
            }
        };

        Some(ConfigModel::from_schema(Some(config_dict)))
    }

    /// Get the user-level configuration
    #[allow(dead_code)]
    pub fn get_user_config(&mut self) -> Option<&ConfigModel<T>> {
        if self.user_config.is_none() {
            self.user_config = self.load_config_from_path(&self.user_config_path);
        }
        self.user_config.as_ref()
    }

    /// Get the project-level configuration
    #[allow(dead_code)]
    pub fn get_project_config(&mut self) -> Option<&ConfigModel<T>> {
        if self.project_config.is_none() && self.project_config_path.is_some() {
            if let Some(path) = &self.project_config_path {
                self.project_config = self.load_config_from_path(path);
            }
        }
        self.project_config.as_ref()
    }

    /// Update a configuration file
    #[allow(dead_code)]
    fn update_config_file(
        &self,
        config_path: &Path,
        config_model: Option<&ConfigModel<T>>,
        config_update: HashMap<String, serde_yaml::Value>,
    ) -> ConfigModel<T> {
        // Load or create config model if not provided
        let mut current_model = match config_model {
            Some(model) => model.clone(),
            None => {
                let mut existing_config = HashMap::new();
                if config_path.exists() {
                    if let Ok(mut file) = File::open(config_path) {
                        let mut contents = String::new();
                        if file.read_to_string(&mut contents).is_ok() {
                            if let Ok(parsed) = serde_yaml::from_str::<
                                HashMap<String, serde_yaml::Value>,
                            >(&contents)
                            {
                                existing_config = parsed;
                            }
                        }
                    }
                }
                ConfigModel::from_schema(Some(existing_config))
            }
        };

        current_model.update(config_update);
        let updated_config = current_model.to_dict();

        // Ensure directory exists
        if let Some(parent) = config_path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent) {
                    error!("Failed to create directory {}: {}", parent.display(), e);
                    return current_model;
                }
            }
        }

        // Write the file
        match File::create(config_path) {
            Ok(mut file) => {
                if let Ok(yaml_str) = serde_yaml::to_string(&updated_config) {
                    if let Err(e) = file.write_all(yaml_str.as_bytes()) {
                        error!(
                            "Failed to write to config file {}: {}",
                            config_path.display(),
                            e
                        );
                    }
                }
            }
            Err(e) => {
                error!(
                    "Failed to create config file {}: {}",
                    config_path.display(),
                    e
                );
            }
        }

        current_model
    }

    /// Update the user-level configuration
    #[allow(dead_code)]
    pub fn update_user_config(&mut self, config_update: HashMap<String, serde_yaml::Value>) {
        let user_config_path = self.user_config_path.clone();
        let user_config = self.user_config.as_ref();

        let updated = self.update_config_file(&user_config_path, user_config, config_update);
        self.user_config = Some(updated);

        // Reset merged config to force reload
        self.config = None;
    }

    /// Update the project-level configuration
    #[allow(dead_code)]
    pub fn update_project_config(&mut self, config_update: HashMap<String, serde_yaml::Value>) {
        if self.project_config_path.is_none() {
            panic!("No project root found. Cannot update project configuration.");
        }

        if let Some(project_path) = self.project_config_path.clone() {
            let project_config = self.project_config.as_ref();

            let updated = self.update_config_file(&project_path, project_config, config_update);
            self.project_config = Some(updated);

            // Reset merged config to force reload
            self.config = None;
        }
    }

    /// Create a user configuration template if it doesn't exist
    #[allow(dead_code)]
    pub fn create_user_config_template(&self) -> PathBuf {
        if let Some(parent) = self.user_config_path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent) {
                    error!("Failed to create directory {}: {}", parent.display(), e);
                }
            }
        }

        if !self.user_config_path.exists() {
            let default_config = self.get_default_dict();
            if let Ok(yaml_str) = serde_yaml::to_string(&default_config) {
                if let Ok(mut file) = File::create(&self.user_config_path) {
                    if let Err(e) = file.write_all(yaml_str.as_bytes()) {
                        error!("Failed to write user config template: {}", e);
                    }
                }
            }
        }

        self.user_config_path.clone()
    }

    /// Create a project configuration template
    #[allow(dead_code)]
    pub fn create_project_config_template(&self, path: Option<&str>) -> PathBuf {
        let project_path = match path {
            Some(p) => PathBuf::from(p),
            None => std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        };

        let config_dir = project_path.join(format!(".{}", self.config_name));
        let config_file = config_dir.join("config.yaml");

        if !config_dir.exists() {
            if let Err(e) = fs::create_dir_all(&config_dir) {
                error!("Failed to create directory {}: {}", config_dir.display(), e);
            }
        }

        if !config_file.exists() {
            let default_config = self.get_default_dict();
            if let Ok(yaml_str) = serde_yaml::to_string(&default_config) {
                if let Ok(mut file) = File::create(&config_file) {
                    if let Err(e) = file.write_all(yaml_str.as_bytes()) {
                        error!("Failed to write project config template: {}", e);
                    }
                }
            }
        }

        config_file
    }
}

/// Get the path to the user-level configuration file
pub fn get_user_config_path(config_name: &str) -> PathBuf {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    home_dir
        .join(".zeeland")
        .join(config_name)
        .join("config.yaml")
}

/// Get the path to the project-level configuration file
pub fn get_project_config_path(config_name: &str, project_path: Option<&str>) -> Option<PathBuf> {
    let project_root = if let Some(path) = project_path {
        PathBuf::from(path)
    } else {
        find_project_root()?
    };

    Some(
        project_root
            .join(format!(".{}", config_name))
            .join("config.yaml"),
    )
}

/// Find the project root directory by looking for common project files
pub fn find_project_root() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;

    let indicators = [
        ".git",
        "pyproject.toml",
        "setup.py",
        "package.json",
        "Cargo.toml",
    ];

    let mut current = cwd;
    loop {
        for indicator in &indicators {
            if current.join(indicator).exists() {
                return Some(current);
            }
        }

        if !current.pop() {
            break;
        }
    }

    None
}

/// Recursively update a hashmap
#[allow(dead_code)]
pub fn deep_update(
    base_dict: HashMap<String, serde_yaml::Value>,
    update_dict: HashMap<String, serde_yaml::Value>,
) -> HashMap<String, serde_yaml::Value> {
    let mut result = base_dict;

    for (key, value) in update_dict {
        // Skip empty string values to preserve original values
        if let serde_yaml::Value::String(s) = &value {
            if s.is_empty() {
                continue;
            }
        }

        match result.get(&key) {
            Some(serde_yaml::Value::Mapping(base_map)) if value.is_mapping() => {
                // Convert to HashMap for recursive update
                let base_hash: HashMap<String, serde_yaml::Value> = base_map
                    .iter()
                    .filter_map(|(k, v)| {
                        if let serde_yaml::Value::String(key_str) = k {
                            Some((key_str.clone(), v.clone()))
                        } else {
                            None
                        }
                    })
                    .collect();

                let update_hash: HashMap<String, serde_yaml::Value> = match &value {
                    serde_yaml::Value::Mapping(update_map) => update_map
                        .iter()
                        .filter_map(|(k, v)| {
                            if let serde_yaml::Value::String(key_str) = k {
                                Some((key_str.clone(), v.clone()))
                            } else {
                                None
                            }
                        })
                        .collect(),
                    _ => HashMap::new(),
                };

                // Recursively update nested dictionaries
                let updated = deep_update(base_hash, update_hash);

                // Convert back to Mapping
                let mapping = serde_yaml::Mapping::from_iter(
                    updated
                        .into_iter()
                        .map(|(k, v)| (serde_yaml::Value::String(k), v)),
                );

                result.insert(key, serde_yaml::Value::Mapping(mapping));
            }
            _ => {
                // Direct update for simple values or new keys
                result.insert(key, value);
            }
        }
    }

    result
}

/// Merge multiple configuration levels
#[allow(dead_code)]
pub fn merge_configs_dict(
    default_config: HashMap<String, serde_yaml::Value>,
    user_config: Option<HashMap<String, serde_yaml::Value>>,
    project_config: Option<HashMap<String, serde_yaml::Value>>,
) -> HashMap<String, serde_yaml::Value> {
    let mut result = default_config;

    // Apply user config over defaults
    if let Some(user_cfg) = user_config {
        result = deep_update(result, user_cfg);
    }

    // Apply project config over previous levels
    if let Some(project_cfg) = project_config {
        result = deep_update(result, project_cfg);
    }

    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use std::path::{Path, PathBuf};

    use serde::{Deserialize, Serialize};

    use super::*;

    // Simple test struct for diagnosis
    #[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
    struct SimpleConfig {
        name: String,
        value: i32,
    }

    // Test config struct
    #[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
    struct TestConfig {
        app: AppSettings,
        database: DbSettings,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
    struct AppSettings {
        name: String,
        debug: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
    struct DbSettings {
        url: String,
        username: String,
        password: String,
    }

    // Create test directories and files
    fn setup_test_dirs() -> (PathBuf, PathBuf) {
        let temp_dir = std::env::temp_dir().join("conftier_test");
        let user_config_dir = temp_dir.join("user");
        let project_config_dir = temp_dir.join("project");

        fs::create_dir_all(&user_config_dir).unwrap();
        fs::create_dir_all(&project_config_dir).unwrap();

        (user_config_dir, project_config_dir)
    }

    // Clean up test directories
    fn cleanup_test_dirs() {
        let temp_dir = std::env::temp_dir().join("conftier_test");
        let _ = fs::remove_dir_all(temp_dir);
    }

    // Create test config file
    fn create_test_config_file(path: &Path, content: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, content).unwrap();
    }

    // Added diagnostic test
    #[test]
    fn test_simple_config_model() {
        // Create a simple test struct
        let mut data = HashMap::new();
        data.insert(
            "name".to_string(),
            serde_yaml::Value::String("test_name".to_string()),
        );
        data.insert("value".to_string(), serde_yaml::Value::Number(42.into()));

        // Try to deserialize it directly
        let mapping = serde_yaml::Mapping::from_iter(
            data.iter()
                .map(|(k, v)| (serde_yaml::Value::String(k.clone()), v.clone())),
        );
        let yaml_value = serde_yaml::Value::Mapping(mapping);

        // Print the YAML value
        println!("YAML value: {:?}", yaml_value);

        // Try direct deserialization
        let direct_config: SimpleConfig = serde_yaml::from_value(yaml_value.clone()).unwrap();
        println!("Direct deserialization: {:?}", direct_config);
        assert_eq!(direct_config.name, "test_name");
        assert_eq!(direct_config.value, 42);

        // Try using ConfigModel
        let model: ConfigModel<SimpleConfig> = ConfigModel::from_schema(Some(data));
        println!("Model: {:?}", model);
        println!("Model.model: {:?}", model.model());

        // Test assertions
        assert_eq!(model.model().name, "test_name");
        assert_eq!(model.model().value, 42);
    }

    #[test]
    fn test_config_model_from_schema() {
        // 1. Test default values
        let model: ConfigModel<TestConfig> = ConfigModel::from_schema(None);
        assert_eq!(model.schema_type, SchemaType::Struct);
        assert_eq!(model.model().app.name, "");
        assert_eq!(model.model().database.url, "");

        // 2. Test loading values - Create properly formatted nested data
        // Create the test config directly
        let test_config = TestConfig {
            app: AppSettings {
                name: "TestApp".to_string(),
                debug: true,
            },
            database: DbSettings {
                url: "".to_string(),
                username: "".to_string(),
                password: "".to_string(),
            },
        };

        // Serialize to YAML
        let yaml_str = serde_yaml::to_string(&test_config).unwrap();
        println!("YAML string: {}", yaml_str);

        // Deserialize to HashMap
        let data: HashMap<String, serde_yaml::Value> = serde_yaml::from_str(&yaml_str).unwrap();
        println!("Deserialized HashMap: {:?}", data);

        // Create the model
        let model: ConfigModel<TestConfig> = ConfigModel::from_schema(Some(data));
        println!("Model: {:?}", model);
        assert_eq!(model.model().app.name, "TestApp");
        assert!(model.model().app.debug);
    }

    #[test]
    fn test_config_model_to_dict() {
        // Create a test config
        let test_config = TestConfig {
            app: AppSettings {
                name: "TestApp".to_string(),
                debug: true,
            },
            database: DbSettings {
                url: "test_url".to_string(),
                username: "user".to_string(),
                password: "pass".to_string(),
            },
        };

        // Serialize to YAML
        let yaml_str = serde_yaml::to_string(&test_config).unwrap();

        // Deserialize to HashMap
        let data: HashMap<String, serde_yaml::Value> = serde_yaml::from_str(&yaml_str).unwrap();

        // Create the model
        let model = ConfigModel::<TestConfig>::from_schema(Some(data));

        // Convert back to dictionary
        let dict = model.to_dict();

        // Verify results
        assert!(dict.contains_key("app"));
        if let serde_yaml::Value::Mapping(app_map) = &dict["app"] {
            let name_key = serde_yaml::Value::String("name".to_string());
            let debug_key = serde_yaml::Value::String("debug".to_string());

            if let Some(serde_yaml::Value::String(name)) = app_map.get(&name_key) {
                assert_eq!(name, "TestApp");
            } else {
                panic!("Expected app.name to be a string");
            }

            if let Some(serde_yaml::Value::Bool(debug)) = app_map.get(&debug_key) {
                assert!(*debug);
            } else {
                panic!("Expected app.debug to be a boolean");
            }
        } else {
            panic!("Expected app to be a mapping");
        }
    }

    #[test]
    fn test_config_model_update() {
        // Create initial model
        let mut model = ConfigModel::<TestConfig>::from_schema(None);

        // Prepare update data
        let mut update_data = HashMap::new();
        let mut app_data = HashMap::new();
        app_data.insert(
            "name".to_string(),
            serde_yaml::Value::String("UpdatedApp".to_string()),
        );
        app_data.insert("debug".to_string(), serde_yaml::Value::Bool(true));
        update_data.insert(
            "app".to_string(),
            serde_yaml::Value::Mapping(serde_yaml::Mapping::from_iter(
                app_data
                    .iter()
                    .map(|(k, v)| (serde_yaml::Value::String(k.clone()), v.clone())),
            )),
        );

        // Execute update
        model.update(update_data);

        // Verify update results
        assert_eq!(model.model().app.name, "UpdatedApp");
        assert!(model.model().app.debug);
    }

    #[test]
    fn test_config_model_merge() {
        // Create base config
        let base_config = TestConfig {
            app: AppSettings {
                name: "BaseApp".to_string(),
                debug: false,
            },
            database: DbSettings {
                url: "base_url".to_string(),
                username: "base_user".to_string(),
                password: "base_pass".to_string(),
            },
        };

        // Serialize to YAML and create base model
        let base_yaml = serde_yaml::to_string(&base_config).unwrap();
        let base_data: HashMap<String, serde_yaml::Value> =
            serde_yaml::from_str(&base_yaml).unwrap();
        let base_model = ConfigModel::<TestConfig>::from_schema(Some(base_data));

        // Create other config with some different values
        let other_config = TestConfig {
            app: AppSettings {
                name: "".to_string(), // Empty to keep base value
                debug: true,          // Override base value
            },
            database: DbSettings {
                url: "other_url".to_string(), // Override base value
                username: "".to_string(),     // Empty to keep base value
                password: "".to_string(),     // Empty to keep base value
            },
        };

        // Serialize to YAML and create other model
        let other_yaml = serde_yaml::to_string(&other_config).unwrap();
        let other_data: HashMap<String, serde_yaml::Value> =
            serde_yaml::from_str(&other_yaml).unwrap();
        let other_model = ConfigModel::<TestConfig>::from_schema(Some(other_data));

        // Merge models
        let merged_model = base_model.merge(&other_model);

        // Verify merge results
        assert_eq!(merged_model.model().app.name, "BaseApp"); // Not in other, should keep base value
        assert!(merged_model.model().app.debug); // In other, should use new value
        assert_eq!(merged_model.model().database.url, "other_url"); // In other, should use new value
        assert_eq!(merged_model.model().database.username, "base_user"); // Not in other, should keep base value
    }

    #[test]
    fn test_deep_update() {
        // Create base dictionary
        let mut base_dict = HashMap::new();

        // Create nested structure
        let mut app = HashMap::new();
        app.insert(
            "name".to_string(),
            serde_yaml::Value::String("BaseApp".to_string()),
        );
        app.insert("debug".to_string(), serde_yaml::Value::Bool(false));

        let mut database = HashMap::new();
        database.insert(
            "url".to_string(),
            serde_yaml::Value::String("base_url".to_string()),
        );
        database.insert(
            "username".to_string(),
            serde_yaml::Value::String("base_user".to_string()),
        );

        base_dict.insert(
            "app".to_string(),
            serde_yaml::Value::Mapping(serde_yaml::Mapping::from_iter(
                app.iter()
                    .map(|(k, v)| (serde_yaml::Value::String(k.clone()), v.clone())),
            )),
        );
        base_dict.insert(
            "database".to_string(),
            serde_yaml::Value::Mapping(serde_yaml::Mapping::from_iter(
                database
                    .iter()
                    .map(|(k, v)| (serde_yaml::Value::String(k.clone()), v.clone())),
            )),
        );

        // Create update dictionary
        let mut update_dict = HashMap::new();

        // Update partial values
        let mut app_update = HashMap::new();
        app_update.insert("debug".to_string(), serde_yaml::Value::Bool(true));

        let mut db_update = HashMap::new();
        db_update.insert(
            "url".to_string(),
            serde_yaml::Value::String("new_url".to_string()),
        );

        update_dict.insert(
            "app".to_string(),
            serde_yaml::Value::Mapping(serde_yaml::Mapping::from_iter(
                app_update
                    .iter()
                    .map(|(k, v)| (serde_yaml::Value::String(k.clone()), v.clone())),
            )),
        );
        update_dict.insert(
            "database".to_string(),
            serde_yaml::Value::Mapping(serde_yaml::Mapping::from_iter(
                db_update
                    .iter()
                    .map(|(k, v)| (serde_yaml::Value::String(k.clone()), v.clone())),
            )),
        );

        // Execute deep update
        let result = deep_update(base_dict, update_dict);

        // Verify results
        if let serde_yaml::Value::Mapping(app_map) = &result["app"] {
            let name_key = serde_yaml::Value::String("name".to_string());
            let debug_key = serde_yaml::Value::String("debug".to_string());

            if let Some(serde_yaml::Value::String(name)) = app_map.get(&name_key) {
                assert_eq!(name, "BaseApp"); // Not updated, should keep original value
            } else {
                panic!("Expected app.name to be a string");
            }

            if let Some(serde_yaml::Value::Bool(debug)) = app_map.get(&debug_key) {
                assert!(*debug); // Updated, should use new value
            } else {
                panic!("Expected app.debug to be a boolean");
            }
        } else {
            panic!("Expected app to be a mapping");
        }

        if let serde_yaml::Value::Mapping(db_map) = &result["database"] {
            let url_key = serde_yaml::Value::String("url".to_string());
            let user_key = serde_yaml::Value::String("username".to_string());

            if let Some(serde_yaml::Value::String(url)) = db_map.get(&url_key) {
                assert_eq!(url, "new_url"); // Updated, should be new value
            } else {
                panic!("Expected database.url to be a string");
            }

            if let Some(serde_yaml::Value::String(username)) = db_map.get(&user_key) {
                assert_eq!(username, "base_user"); // Not updated, should keep original value
            } else {
                panic!("Expected database.username to be a string");
            }
        } else {
            panic!("Expected database to be a mapping");
        }
    }

    #[test]
    fn test_config_manager_basic() {
        // Set test directories
        let (user_dir, project_dir) = setup_test_dirs();
        let _cleanup = scopeguard::guard((), |_| cleanup_test_dirs());

        // Create test ConfigManager
        let mut manager = ConfigManager::<TestConfig>::new(
            "test_app", "1.0.0", false, // Do not auto-create user config
            false, // Do not auto-create project config
        );

        // Replace paths with test paths
        manager.user_config_path = user_dir.join("test_app").join("config.yaml");
        manager.project_config_path = Some(project_dir.join(".test_app").join("config.yaml"));

        // Create user config
        let user_config = TestConfig {
            app: AppSettings {
                name: "UserApp".to_string(),
                debug: false,
            },
            database: DbSettings {
                url: "user_db_url".to_string(),
                username: "user".to_string(),
                password: "secret".to_string(),
            },
        };

        // Create project config (partial overrides)
        let project_config = TestConfig {
            app: AppSettings {
                name: "".to_string(), // Keep user value
                debug: true,          // Override user value
            },
            database: DbSettings {
                url: "project_db_url".to_string(), // Override user value
                username: "".to_string(),          // Keep user value
                password: "".to_string(),          // Keep user value
            },
        };

        // Manually create configuration files
        let user_yaml = serde_yaml::to_string(&user_config).unwrap();
        let project_yaml = serde_yaml::to_string(&project_config).unwrap();

        create_test_config_file(&manager.user_config_path, &user_yaml);
        create_test_config_file(manager.project_config_path.as_ref().unwrap(), &project_yaml);

        // Load configuration
        let config = manager.load();

        // Verify merge results
        assert_eq!(config.app.name, "UserApp"); // Retrieved from user config
        assert!(config.app.debug); // Retrieved from project config (overrides user config)
        assert_eq!(config.database.url, "project_db_url"); // Retrieved from project config (overrides user config)
        assert_eq!(config.database.username, "user"); // Retrieved from user config
        assert_eq!(config.database.password, "secret"); // Retrieved from user config
    }

    #[test]
    fn test_merge_configs_dict() {
        // Test config merge order and priority

        // Default config
        let mut default_config = HashMap::new();
        default_config.insert(
            "key1".to_string(),
            serde_yaml::Value::String("default1".to_string()),
        );
        default_config.insert(
            "key2".to_string(),
            serde_yaml::Value::String("default2".to_string()),
        );
        default_config.insert(
            "key3".to_string(),
            serde_yaml::Value::String("default3".to_string()),
        );

        // User config
        let mut user_config = HashMap::new();
        user_config.insert(
            "key1".to_string(),
            serde_yaml::Value::String("user1".to_string()),
        );
        user_config.insert(
            "key2".to_string(),
            serde_yaml::Value::String("user2".to_string()),
        );

        // Project config
        let mut project_config = HashMap::new();
        project_config.insert(
            "key1".to_string(),
            serde_yaml::Value::String("project1".to_string()),
        );

        // Merge configs
        let merged = merge_configs_dict(default_config, Some(user_config), Some(project_config));

        // Verify priority: project > user > default
        assert_eq!(
            merged["key1"],
            serde_yaml::Value::String("project1".to_string())
        ); // Project priority
        assert_eq!(
            merged["key2"],
            serde_yaml::Value::String("user2".to_string())
        ); // User priority
        assert_eq!(
            merged["key3"],
            serde_yaml::Value::String("default3".to_string())
        ); // Default value
    }
}
