use std::sync::Arc;

use anyhow::{anyhow, Error};
use regex::Regex;

use crate::ros_introspection::Field;
use crate::ros_introspection::Type;

#[derive(Debug, Clone)]
pub struct Message {
    msg_type: Type,
    fields: Vec<Field>,
}

impl Message {
    /// Creates a new `Message` instance from a definition string.
    ///
    /// # Arguments
    ///
    /// * `def` - A string slice that holds the definition of the message.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Error>` - A result containing the new `Message` instance or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The regular expression for parsing the message definition fails to compile.
    /// - The message type cannot be extracted from the definition.
    /// - A field cannot be created from the definition.
    pub fn new(def: &str) -> Result<Self, Error> {
        let mut msg_type = Type::new("")?;
        let mut fields = Vec::new();

        let re = Regex::new(r"(^\s*$|^\s*#)")?;

        let lines: Vec<&str> = def.lines().collect();
        for line in lines {
            if re.is_match(line) {
                continue;
            }

            let line = line.trim();

            if line.starts_with("MSG:") {
                let line = &line[("MSG:".len() + 1)..];
                msg_type = Type::new(line)?;
            } else {
                let new_field = Field::new_with_definition(line)?;
                fields.push(new_field);
            }
        }

        Ok(Self { msg_type, fields })
    }

    /// Returns a reference to the type of the message.
    ///
    /// # Returns
    ///
    /// * `&Type` - A reference to the `Type` of the message.
    pub fn type_(&self) -> &Type {
        &self.msg_type
    }

    /// Sets the type of the message.
    ///
    /// # Arguments
    ///
    /// * `new_type` - The new `Type` of the message.
    pub fn set_type(&mut self, new_type: Type) {
        self.msg_type = new_type;
    }

    /// Returns a reference to the fields of the message.
    ///
    /// # Returns
    ///
    /// * `&Vec<Field>` - A reference to the vector of `Field` instances.
    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }

    /// Returns a mutable reference to the fields of the message.
    ///
    /// # Returns
    ///
    /// * `&mut Vec<Field>` - A mutable reference to the vector of `Field` instances.
    pub fn fields_mut(&mut self) -> &mut Vec<Field> {
        &mut self.fields
    }
}

/// Splits a string containing multiple message definitions into individual message definitions.
///
/// # Arguments
///
/// * `multi_def` - A string slice that holds the multiple message definitions.
///
/// # Returns
///
/// * `Vec<String>` - A vector containing individual message definitions.
fn split_multiple_message_definitions(multi_def: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut part = String::new();

    for line in multi_def.lines() {
        let line = line.trim();
        if line.starts_with("========") {
            parts.push(part.clone());
            part.clear();
        } else {
            part.push_str(line);
            part.push('\n');
        }
    }
    parts.push(part);

    parts
}

/// Parses multiple message definitions and returns a vector of `Message` instances.
///
/// # Arguments
///
/// * `multi_def` - A string slice that holds the multiple message definitions.
/// * `root_type` - A reference to the root `Type`.
///
/// # Returns
///
/// * `Result<Vec<Arc<Message>>, Error>` - A result containing a vector of `Message` instances or an error.
///
/// # Errors
///
/// This function will return an error if:
/// - The message type is invalid.
/// - A mutable reference to a message cannot be obtained.
/// - The message type is unspecified.
pub fn parse_message_definitions(
    multi_def: &str,
    root_type: &Type,
) -> Result<Vec<Arc<Message>>, Error> {
    let parts = split_multiple_message_definitions(multi_def);
    let mut known_type = Vec::new();
    let mut parsed_msgs = Vec::new();

    let no_type = Type::new("")?;

    for i in (0..parts.len()).rev() {
        let mut msg = Arc::new(Message::new(&parts[i])?);

        if i == 0 {
            if msg.type_() == &no_type && root_type != &no_type {
                Arc::get_mut(&mut msg)
                    .ok_or(anyhow!("Could not get mutable reference to message",))?
                    .set_type(root_type.clone());
            } else if msg.type_() == &no_type && root_type == &no_type {
                panic!("Message type unspecified");
            }
        }

        parsed_msgs.push(msg.clone());
        known_type.push(msg.type_().clone());
    }

    for msg in &mut parsed_msgs {
        let msg =
            Arc::get_mut(msg).ok_or(anyhow!("Could not get mutable reference to message",))?;
        for field in msg.fields_mut() {
            if field.type_().pkg_name().is_empty() {
                let mut guessed_type = Vec::new();

                for known_type in &known_type {
                    if field.type_().msg_name() == known_type.msg_name() {
                        guessed_type.push(known_type.clone());
                    }
                }

                if !guessed_type.is_empty() {
                    let mut better_match = false;

                    for guessed_type in &guessed_type {
                        if guessed_type.pkg_name() == root_type.pkg_name() {
                            field.change_type(guessed_type.clone());
                            better_match = true;
                            break;
                        }
                    }

                    if !better_match {
                        field.change_type(guessed_type[0].clone());
                    }
                }
            }
        }
    }

    parsed_msgs.reverse();
    Ok(parsed_msgs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ros_introspection::Type;

    #[test]
    fn test_new() -> Result<(), Error> {
        let def = r#"
            MSG: std_msgs/String
            string data
        "#;
        let msg = Message::new(def)?;

        assert_eq!(msg.type_().name(), "std_msgs/String");
        assert_eq!(msg.fields().len(), 1);
        assert_eq!(msg.fields()[0].name(), "data");
        assert_eq!(msg.fields()[0].type_().name(), "string");
        Ok(())
    }

    #[test]
    fn test_getters() -> Result<(), Error> {
        let def = r#"
            MSG: std_msgs/String
            string data
        "#;
        let msg = Message::new(def)?;

        assert_eq!(msg.type_().name(), "std_msgs/String");
        assert_eq!(msg.fields().len(), 1);
        assert_eq!(msg.fields()[0].name(), "data");
        assert_eq!(msg.fields()[0].type_().name(), "string");

        Ok(())
    }

    #[test]
    fn test_set_type() {
        let def = r#"
            string data
        "#;
        let mut msg = Message::new(def).unwrap();
        let new_type = Type::new("std_msgs/String").unwrap();
        msg.set_type(new_type.clone());

        assert_eq!(msg.type_(), &new_type);
    }

    #[test]
    fn test_split_multiple_message_definitions() {
        let multi_def = r#"
            std_msgs/String
            string data
            ========
            std_msgs/Int32
            int32 data
        "#;
        let parts = split_multiple_message_definitions(multi_def);

        assert_eq!(parts.len(), 2);
        assert!(parts[0].contains("std_msgs/String"));
        assert!(parts[1].contains("std_msgs/Int32"));
    }

    #[test]
    fn test_parse_message_definitions() {
        let multi_def = r#"
            MSG: std_msgs/String
            string data
            ========
            MSG: std_msgs/Int32
            int32 data
        "#;
        let root_type = Type::new("std_msgs/String").unwrap();
        let msgs = parse_message_definitions(multi_def, &root_type).unwrap();

        assert_eq!(msgs.len(), 2);
        assert_eq!(msgs[0].type_().name(), "std_msgs/String");
        assert_eq!(msgs[0].fields().len(), 1);
        assert_eq!(msgs[0].fields()[0].name(), "data");
        assert_eq!(msgs[0].fields()[0].type_().name(), "string");

        assert_eq!(msgs[1].type_().name(), "std_msgs/Int32");
        assert_eq!(msgs[1].fields().len(), 1);
        assert_eq!(msgs[1].fields()[0].name(), "data");
        assert_eq!(msgs[1].fields()[0].type_().name(), "int32");
    }
}
