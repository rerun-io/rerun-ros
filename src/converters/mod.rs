mod builtin_interfaces;
mod geometry_msgs;
mod std_msgs;
mod traits;

use crate::converters::traits::Converter;

use std::collections::HashMap;
use std::sync::Arc;

use std::io::Cursor;

use rerun;

use anyhow::{Error, Result};

pub struct ConverterRegistry {
    converters: HashMap<String, Arc<dyn Converter>>,
}

/// A registry for managing and using different types of converters.
///
/// The `ConverterRegistry` struct provides methods to register, retrieve, and process
/// converters. Converters are used to transform CDR representations of ROS messages
/// into the rerun system.
///
/// # Methods
///
/// - `new() -> Self`
///
///   Creates a new, empty `ConverterRegistry`.
///
/// - `register(&mut self, name: &str, converter: Arc<dyn Converter>)`
///
///   Registers a new converter with the given name. The converter is stored in the registry
///   and can be retrieved or used later.
///
/// - `get(&self, name: &str) -> Option<&Arc<dyn Converter>>`
///
///   Retrieves a reference to a converter by its name. Returns `None` if the converter
///   is not found.
///
/// - `process(&self, rec: &Arc<rerun::RecordingStream>, topic: &str, frame_id: &Option<String>, entity_path: &str, ros_type: &str, message: &mut Cursor<Vec<u8>>) -> Result<(), Error>`
///
///   Processes a message using the converter associated with the given ROS type. The converter
///   transforms the message read from a `Cursor`.
///
/// - `load_configuration() -> Self`
///
///   Loads a predefined set of converters into the registry. This method registers converters
///   for various standard ROS message types, such as `Int8`, `Int16`, `Float32`, `Transform`,
///   and `Quaternion`.
///
/// # Example
///
/// ```rust
/// let registry = ConverterRegistry::load_configuration();
/// let converter = registry.get("std_msgs/msg/Int8").unwrap();
/// // Use the converter...
/// ```
impl ConverterRegistry {
    pub fn new() -> Self {
        Self {
            converters: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, converter: Arc<dyn Converter>) {
        self.converters.insert(name.to_string(), converter);
    }

    pub fn get(&self, name: &str) -> Option<&Arc<dyn Converter>> {
        self.converters.get(name)
    }

    pub fn process(
        &self,
        rec: &Arc<rerun::RecordingStream>,
        topic: &str,
        frame_id: &Option<String>,
        entity_path: &str,
        ros_type: &str,
        message: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Error> {
        let converter = self.get(ros_type).unwrap();
        converter.convert(rec, topic, frame_id, entity_path, message)?;
        Ok(())
    }

    pub fn load_configuration() -> Self {
        let mut registry = ConverterRegistry::new();
        registry.register("std_msgs/msg/Int8", Arc::new(std_msgs::Int8Converter {}));
        registry.register("std_msgs/msg/Int16", Arc::new(std_msgs::Int16Converter {}));
        registry.register("std_msgs/msg/Int32", Arc::new(std_msgs::Int32Converter {}));
        registry.register("std_msgs/msg/Int64", Arc::new(std_msgs::Int64Converter {}));
        registry.register("std_msgs/msg/UInt8", Arc::new(std_msgs::UInt8Converter {}));
        registry.register(
            "std_msgs/msg/UInt16",
            Arc::new(std_msgs::UInt16Converter {}),
        );
        registry.register(
            "std_msgs/msg/UInt32",
            Arc::new(std_msgs::UInt32Converter {}),
        );
        registry.register(
            "std_msgs/msg/UInt64",
            Arc::new(std_msgs::UInt64Converter {}),
        );
        registry.register(
            "std_msgs/msg/Float32",
            Arc::new(std_msgs::Float32Converter {}),
        );
        registry.register(
            "std_msgs/msg/Float64",
            Arc::new(std_msgs::Float64Converter {}),
        );
        registry.register(
            "geometry_msgs/msg/Transform",
            Arc::new(geometry_msgs::TransformConverter {}),
        );
        registry.register(
            "geometry_msgs/msg/Quaternion",
            Arc::new(geometry_msgs::QuaternionConverter {}),
        );
        registry
    }
}
