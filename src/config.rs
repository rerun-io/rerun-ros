use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Represents a single conversion configuration.
#[derive(Deserialize, Debug)]
struct Conversion {
    topic: String,
    frame_id: Option<String>,
    ros_type: String,
    entity_path: String,
}

/// Parses and holds conversion configurations.
pub struct ConfigParser {
    conversions: HashMap<(String, Option<String>), (String, String)>,
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
                    (conversion.ros_type.clone(), conversion.entity_path.clone()),
                );
            }

            conversions
        };

        Ok(Self { conversions })
    }

    /// Returns a reference to the conversions hashmap.
    pub fn conversions(&self) -> &HashMap<(String, Option<String>), (String, String)> {
        &self.conversions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_config_parser_new_valid_file() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.toml");

        // Write a valid TOML configuration to the file
        let mut file = File::create(&file_path).unwrap();
        writeln!(
            file,
            r#"
            [[conversion]]
            topic = "topic1"
            frame_id = "frame1"
            ros_type = "type1"
            entity_path = "foo/bar1"
            [[conversion]]
            topic = "topic2"
            frame_id = "frame2"
            ros_type = "type2"
            entity_path = "foo/bar2"
            "#
        )
        .unwrap();

        // Create a ConfigParser instance
        let config_parser = ConfigParser::new(file_path.to_str().unwrap()).unwrap();

        // Check the conversions hashmap
        let conversions = config_parser.conversions();
        assert_eq!(conversions.len(), 2);
        assert_eq!(
            conversions.get(&("topic1".to_owned(), "frame1".to_owned())),
            Some(&("type1".to_owned(), "foo/bar1".to_owned()))
        );
        assert_eq!(
            conversions.get(&("topic2".to_owned(), "frame2".to_owned())),
            Some(&("type2".to_owned(), "foo/bar2".to_owned()))
        );
    }

    #[test]
    fn test_config_parser_new_invalid_file() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.toml");

        // Write an invalid TOML configuration to the file
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "invalid_toml").unwrap();

        // Attempt to create a ConfigParser instance and expect an error
        let result = ConfigParser::new(file_path.to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_config_parser_new_missing_file() {
        // Attempt to create a ConfigParser instance with a non-existent file
        let result = ConfigParser::new("non_existent_file.toml");
        assert!(result.is_err());
    }
}
