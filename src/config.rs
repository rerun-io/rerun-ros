use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Represents a single conversion configuration.
#[derive(Deserialize, Debug)]
struct Conversion {
    topic: String,
    frame_id: String,
    ros_type: String,
    converter: String,
}

/// Parses and holds conversion configurations.
pub struct ConfigParser {
    conversions: HashMap<(String, String), (String, String)>,
}

impl ConfigParser {
    /// Creates a new `ConfigParser` from the given configuration file.
    ///
    /// # Arguments
    ///
    /// * `config_file` - A string slice that holds the path to the configuration file.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The configuration file cannot be found or read.
    /// - The configuration file contains invalid TOML.
    /// - The configuration file does not contain the expected structure.
    pub fn new(config_file: &str) -> Result<Self> {
        let conversions = {
            let mut conversions = HashMap::new();
            let config_path = Path::new(config_file);
            let full_path = config_path.canonicalize()?;

            let config_str = fs::read_to_string(full_path)?;
            let config: HashMap<String, Vec<Conversion>> = toml::from_str(&config_str)?;

            for conversion in &config["conversion"] {
                conversions.insert(
                    (conversion.topic.clone(), conversion.frame_id.clone()),
                    (conversion.ros_type.clone(), conversion.converter.clone()),
                );
            }

            conversions
        };

        Ok(Self { conversions })
    }

    /// Returns a reference to the conversions hashmap.
    pub fn conversions(&self) -> &HashMap<(String, String), (String, String)> {
        &self.conversions
    }
}
