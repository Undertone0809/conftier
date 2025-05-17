use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use log::error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_yaml;

/// Schema type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaType {
    /// Struct type
    Struct,
    /// HashMap type
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
                // Convert HashMap<String, Value> to serde_yaml::Value::Mapping
                let mapping = serde_yaml::Mapping::from_iter(
                    data_map
                        .into_iter()
                        .map(|(k, v)| (serde_yaml::Value::String(k), v)),
                );

                match serde_yaml::from_value(serde_yaml::Value::Mapping(mapping)) {
                    Ok(value) => value,
                    Err(e) => {
                        error!("Failed to create model: {}", e);
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

        // Convert HashMap<String, Value> to serde_yaml::Value::Mapping
        let mapping = serde_yaml::Mapping::from_iter(
            updated
                .into_iter()
                .map(|(k, v)| (serde_yaml::Value::String(k), v)),
        );

        if let Ok(new_model) = serde_yaml::from_value(serde_yaml::Value::Mapping(mapping)) {
            self.model = new_model;
        }
    }

    /// Merge with another ConfigModel
    pub fn merge(&self, other: &Self) -> Self {
        let base_dict = self.to_dict();
        let other_dict = other.to_dict();

        let merged_dict = deep_update(base_dict, other_dict);

        // Convert HashMap<String, Value> to serde_yaml::Value::Mapping
        let mapping = serde_yaml::Mapping::from_iter(
            merged_dict
                .into_iter()
                .map(|(k, v)| (serde_yaml::Value::String(k), v)),
        );

        let new_model = match serde_yaml::from_value(serde_yaml::Value::Mapping(mapping)) {
            Ok(value) => value,
            Err(_) => self.model.clone(),
        };

        ConfigModel {
            schema_type: self.schema_type,
            model: new_model,
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

        let manager = ConfigManager {
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
        };

        manager
    }

    /// Initialize after construction
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
            .map_or(false, |p| p.exists());

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
    pub fn config(&mut self) -> &T {
        if self.config.is_none() {
            return self.load();
        }
        self.config.as_ref().unwrap().model()
    }

    /// Get the default configuration
    pub fn get_default_config(&mut self) -> &T {
        if self.default_config.is_none() {
            self.default_config = Some(ConfigModel::from_schema(None));
        }
        self.default_config.as_ref().unwrap().model()
    }

    /// Helper method to load configuration from a path
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
    pub fn get_user_config(&mut self) -> Option<&ConfigModel<T>> {
        if self.user_config.is_none() {
            self.user_config = self.load_config_from_path(&self.user_config_path);
        }
        self.user_config.as_ref()
    }

    /// Get the project-level configuration
    pub fn get_project_config(&mut self) -> Option<&ConfigModel<T>> {
        if self.project_config.is_none() && self.project_config_path.is_some() {
            if let Some(path) = &self.project_config_path {
                self.project_config = self.load_config_from_path(path);
            }
        }
        self.project_config.as_ref()
    }

    /// Update a configuration file
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
    pub fn update_user_config(&mut self, config_update: HashMap<String, serde_yaml::Value>) {
        let user_config_path = self.user_config_path.clone();
        let user_config = self.user_config.as_ref();

        let updated = self.update_config_file(&user_config_path, user_config, config_update);
        self.user_config = Some(updated);

        // Reset merged config to force reload
        self.config = None;
    }

    /// Update the project-level configuration
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
        match find_project_root() {
            Some(root) => root,
            None => return None,
        }
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
pub fn deep_update(
    base_dict: HashMap<String, serde_yaml::Value>,
    update_dict: HashMap<String, serde_yaml::Value>,
) -> HashMap<String, serde_yaml::Value> {
    let mut result = base_dict;

    for (key, value) in update_dict {
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
