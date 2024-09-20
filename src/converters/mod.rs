mod scalars;
mod traits;
mod transforms;

use scalars::*;
use traits::*;
use transforms::*;

use std::collections::HashMap;
use std::sync::Arc;

use std::io::Cursor;

use rerun;

use anyhow::{Error, Result};

pub struct ConverterRegistry {
    converters: HashMap<String, Arc<dyn Converter>>,
}

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
        entity_path: &str,
        ros_type: &str,
        message: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Error> {
        let converter = self.get(ros_type).unwrap();
        converter.convert(rec, entity_path, message)?;
        Ok(())
    }

    pub fn load_configuration() -> Self {
        let mut registry = ConverterRegistry::new();
        registry.register("std_msgs/msg/Int8", Arc::new(Int8Converter {}));
        registry.register("std_msgs/msg/Int16", Arc::new(Int16Converter {}));
        registry.register("std_msgs/msg/Int32", Arc::new(Int32Converter {}));
        registry.register("std_msgs/msg/Int64", Arc::new(Int64Converter {}));
        registry.register("std_msgs/msg/UInt8", Arc::new(UInt8Converter {}));
        registry.register("std_msgs/msg/UInt16", Arc::new(UInt16Converter {}));
        registry.register("std_msgs/msg/UInt32", Arc::new(UInt32Converter {}));
        registry.register("std_msgs/msg/UInt64", Arc::new(UInt64Converter {}));
        registry.register("std_msgs/msg/Float32", Arc::new(Float32Converter {}));
        registry.register("std_msgs/msg/Float64", Arc::new(Float64Converter {}));
        registry.register(
            "geometry_msgs/msg/Transform",
            Arc::new(TransformConverter {}),
        );
        registry.register(
            "geometry_msgs/msg/Quaternion",
            Arc::new(QuaternionConverter {}),
        );
        registry
    }
}
