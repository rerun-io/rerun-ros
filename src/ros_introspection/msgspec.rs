use crate::ros_introspection::{self, BuiltinType, Message, Type};
use anyhow::{anyhow, Error, Result};
use std::fs;
use std::sync::Arc;

/// Represents a ROS message specification.
pub struct MsgSpec {
    data: Arc<Message>,
    children: Vec<Arc<MsgSpec>>,
}

impl MsgSpec {
    /// Creates a new `MsgSpec` instance for the given topic type.
    ///
    /// # Arguments
    ///
    /// * `topic_type` - A string slice that holds the type of the topic.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Error>` - A result containing the new `MsgSpec` instance or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the message definition cannot be retrieved.
    pub fn new(topic_type: &str) -> Result<Self, Error> {
        Self::new_with_parent_package(topic_type, "")
    }

    /// Creates a new `MsgSpec` instance for the given topic type and parent package.
    ///
    /// # Arguments
    ///
    /// * `topic_type` - A string slice that holds the type of the topic.
    /// * `parent_package` - A string slice that holds the name of the parent package.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Error>` - A result containing the new `MsgSpec` instance or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the message definition cannot be retrieved.
    fn new_with_parent_package(topic_type: &str, parent_package: &str) -> Result<Self, Error> {
        let msg_def = Self::get_message_definition(topic_type, parent_package)?;
        let mut children = Vec::new();

        for field in msg_def.fields() {
            if field.type_().id() == &BuiltinType::Other {
                let child = Self::new_with_parent_package(
                    field.type_().name(),
                    msg_def.type_().pkg_name(),
                )?;
                children.push(Arc::new(child));
            }
        }

        Ok(Self {
            data: msg_def,
            children,
        })
    }

    /// Retrieves the message definition for the given topic type and parent package.
    ///
    /// # Arguments
    ///
    /// * `topic_type` - A string slice that holds the type of the topic.
    /// * `parent_package` - A string slice that holds the name of the parent package.
    ///
    /// # Returns
    ///
    /// * `Result<Arc<Message>, Error>` - A result containing the message definition or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the message type is invalid, the package share directory cannot be found, or the message file cannot be read.
    fn get_message_definition(
        topic_type: &str,
        parent_package: &str,
    ) -> Result<Arc<Message>, Error> {
        let message_type = {
            let message_type = Type::new(topic_type)?;
            if message_type.pkg_name().is_empty() && message_type.id() == &BuiltinType::Other {
                Type::new_with_parent_package(topic_type, parent_package)?
            } else {
                message_type
            }
        };

        let ament_index = ament_rs::Ament::new()?;
        let mut msg_file_path = ament_index
            .get_package_share_directory(message_type.pkg_name())
            .ok_or(anyhow!(
                "Could not find package share directory for package: {}",
                message_type.pkg_name(),
            ))?;

        msg_file_path.push("msg");
        msg_file_path.push(format!("{}.msg", message_type.msg_name()));

        let contents = fs::read_to_string(msg_file_path)?;

        let msg_parsed = ros_introspection::parse_message_definitions(&contents, &message_type)?;

        let msg_def = Arc::clone(&msg_parsed[0]);
        Ok(msg_def)
    }

    /// Returns a reference to the message data.
    ///
    /// # Returns
    ///
    /// * `&Arc<Message>` - A reference to the message data.
    pub fn data(&self) -> &Arc<Message> {
        &self.data
    }

    /// Returns a reference to the children message specifications.
    ///
    /// # Returns
    ///
    /// * `&Vec<Arc<Self>>` - A reference to the vector of children message specifications.
    pub fn children(&self) -> &Vec<Arc<Self>> {
        &self.children
    }
}
