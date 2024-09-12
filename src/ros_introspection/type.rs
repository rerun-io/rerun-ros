use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use anyhow::{anyhow, Error, Result};

#[derive(Debug, Clone)]
pub struct Type {
    base_name: String,
    pkg_name: String,
    msg_name: String,
    id: BuiltinType,
    hash: u64,
}

impl Type {
    /// Creates a new `Type` instance with the given name and parent package name.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the type.
    /// * `parent_pkg_name` - A string slice that holds the name of the parent package.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Error>` - A result containing the new `Type` instance or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The regular expression for parsing the message datatype fails to compile.
    /// - The message name cannot be extracted from the given name.
    pub fn new_with_parent_package(name: &str, parent_pkg_name: &str) -> Result<Self, Error> {
        let msg_datatype_regex =
            regex::Regex::new(r"([a-zA-Z][a-zA-Z0-9_]+)/(msg/)?([a-zA-Z][a-zA-Z0-9_]+)")?;

        let (pkg_name, msg_name, id) = {
            let id = to_builtin_type(name);

            if let Some(what) = msg_datatype_regex.captures(name) {
                let pkg_name = what
                    .get(1)
                    .ok_or(anyhow!("Could not extract message name from {}", name))?
                    .as_str()
                    .to_owned();

                let msg_name = what
                    .get(3)
                    .ok_or(anyhow!("Could not extract message name from {}", name))?
                    .as_str()
                    .to_owned();
                (pkg_name, msg_name, id)
            } else if id == BuiltinType::Other {
                (parent_pkg_name.to_owned(), name.to_owned(), id)
            } else {
                (String::default(), name.to_owned(), id)
            }
        };

        let hash = calculate_hash(name);

        Ok(Self {
            base_name: name.to_owned(),
            pkg_name,
            msg_name,
            id,
            hash,
        })
    }

    /// Creates a new `Type` instance with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the type.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Error>` - A result containing the new `Type` instance or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the message name cannot be extracted from the given name.
    pub fn new(name: &str) -> Result<Self> {
        Self::new_with_parent_package(name, "")
    }

    /// Returns the package name of the type.
    ///
    /// # Returns
    ///
    /// * `&str` - A string slice that holds the package name of the type.
    pub fn pkg_name(&self) -> &str {
        &self.pkg_name
    }

    /// Returns the message name of the type.
    ///
    /// # Returns
    ///
    /// * `&str` - A string slice that holds the message name of the type.
    pub fn msg_name(&self) -> &str {
        &self.msg_name
    }

    /// Returns the ID of the type.
    ///
    /// # Returns
    ///
    /// * `&BuiltinType` - A reference to the `BuiltinType` of the type.
    pub fn id(&self) -> &BuiltinType {
        &self.id
    }

    /// Returns the base name of the type.
    ///
    /// # Returns
    ///
    /// * `&str` - A string slice that holds the base name of the type.
    pub fn name(&self) -> &str {
        &self.base_name
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Eq for Type {}

impl std::hash::Hash for Type {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base_name)
    }
}

/// Calculates the hash value for a given string.
///
/// # Arguments
///
/// * `s` - A string slice that holds the input string.
///
/// # Returns
///
/// * `u64` - The hash value of the input string.
fn calculate_hash(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltinType {
    Bool,
    Byte,
    Char,
    Float32,
    Float64,
    Int8,
    Uint8,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
    String,
    WString,
    Other,
}

/// Converts a string to a `BuiltinType`.
///
/// # Arguments
///
/// * `s` - A string slice that holds the input string.
///
/// # Returns
///
/// * `BuiltinType` - The corresponding `BuiltinType` for the input string.
fn to_builtin_type(s: &str) -> BuiltinType {
    match s {
        "bool" => BuiltinType::Bool,
        "byte" => BuiltinType::Byte,
        "char" => BuiltinType::Char,
        "float32" => BuiltinType::Float32,
        "float64" => BuiltinType::Float64,
        "int8" => BuiltinType::Int8,
        "uint8" => BuiltinType::Uint8,
        "int16" => BuiltinType::Int16,
        "uint16" => BuiltinType::Uint16,
        "int32" => BuiltinType::Int32,
        "uint32" => BuiltinType::Uint32,
        "int64" => BuiltinType::Int64,
        "uint64" => BuiltinType::Uint64,
        "string" => BuiltinType::String,
        "wstring" => BuiltinType::WString,
        _ => BuiltinType::Other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ros_type = Type::new("std_msgs/msg/String").unwrap();
        assert_eq!(ros_type.pkg_name(), "std_msgs");
        assert_eq!(ros_type.msg_name(), "String");
        assert_eq!(ros_type.id(), &BuiltinType::Other);
        assert_eq!(ros_type.name(), "std_msgs/msg/String");

        let ros_type = Type::new("std_msgs/String").unwrap();
        assert_eq!(ros_type.pkg_name(), "std_msgs");
        assert_eq!(ros_type.msg_name(), "String");
        assert_eq!(ros_type.id(), &BuiltinType::Other);
        assert_eq!(ros_type.name(), "std_msgs/String");

        let ros_type = Type::new("String").unwrap();
        assert_eq!(ros_type.pkg_name(), "");
        assert_eq!(ros_type.msg_name(), "String");
        assert_eq!(ros_type.id(), &BuiltinType::Other);
        assert_eq!(ros_type.name(), "String");

        let ros_type = Type::new("unknown_type").unwrap();
        assert_eq!(ros_type.pkg_name(), "");
        assert_eq!(ros_type.msg_name(), "unknown_type");
        assert_eq!(ros_type.id(), &BuiltinType::Other);
        assert_eq!(ros_type.name(), "unknown_type");

        let ros_type = Type::new("int32").unwrap();
        assert_eq!(ros_type.pkg_name(), "");
        assert_eq!(ros_type.msg_name(), "int32");
        assert_eq!(ros_type.id(), &BuiltinType::Int32);
        assert_eq!(ros_type.name(), "int32");
    }

    #[test]
    fn test_getters() {
        let ros_type = Type::new("std_msgs/msg/String").unwrap();

        assert_eq!(ros_type.pkg_name(), "std_msgs");
        assert_eq!(ros_type.msg_name(), "String");
        assert_eq!(ros_type.id(), &BuiltinType::Other);
        assert_eq!(ros_type.name(), "std_msgs/msg/String");
    }

    #[test]
    fn test_partial_eq() {
        let ros_type1 = Type::new("std_msgs/msg/String").unwrap();
        let ros_type2 = Type::new("std_msgs/msg/String").unwrap();
        let ros_type3 = Type::new("std_msgs/msg/Int32").unwrap();

        assert_eq!(ros_type1, ros_type2);
        assert_ne!(ros_type1, ros_type3);
    }

    #[test]
    fn test_hash() {
        let ros_type = Type::new("std_msgs/msg/String").unwrap();
        let expected_hash = calculate_hash("std_msgs/msg/String");

        assert_eq!(ros_type.hash, expected_hash);
    }

    #[test]
    fn test_display() {
        let ros_type = Type::new("std_msgs/msg/String").unwrap();
        assert_eq!(format!("{ros_type}"), "std_msgs/msg/String");
    }
}
