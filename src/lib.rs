use std::collections::HashMap;
use std::sync::Arc;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use regex::Regex;

use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ROSType {
    base_name: String,
    pkg_name: String,
    msg_name: String,
    id: BuiltinType,
    hash: u64,
}

impl ROSType {
    pub fn new(name: &str) -> Self {
        let mut pos = None;
        for (i, c) in name.chars().enumerate() {
            if c == '/' {
                pos = Some(i);
                break;
            }
        }

        let (pkg_name, msg_name) = if let Some(pos) = pos {
            let pkg_name = name[..pos].to_string();
            let msg_name = name[pos + 1..].to_string();
            if msg_name.starts_with("msg/") || msg_name.starts_with("srv/") {
                (pkg_name, msg_name[4..].to_string())
            } else {
                (pkg_name, msg_name)
            }
        } else {
            (String::new(), name.to_string())
        };

        let id = to_builtin_type(&msg_name);
        let hash = calculate_hash(name);

        ROSType {
            base_name: name.to_string(),
            pkg_name,
            msg_name,
            id,
            hash,
        }
    }

    pub fn set_pkg_name(&mut self, new_pkg: &str) {
        assert!(self.pkg_name.is_empty());

        self.base_name = format!("{}/{}", new_pkg, self.base_name);
        self.pkg_name = new_pkg.to_string();
        self.msg_name = self.base_name[new_pkg.len() + 1..].to_string();
        self.hash = calculate_hash(&self.base_name);
    }

    pub fn pkg_name(&self) -> &str {
        &self.pkg_name
    }

    pub fn msg_name(&self) -> &str {
        &self.msg_name
    }
}

impl PartialEq for ROSType {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Eq for ROSType {}

impl std::hash::Hash for ROSType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl std::fmt::Display for ROSType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base_name)
    }
}

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
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
    Time,
    Duration,
    String,
    Other,
}

fn to_builtin_type(s: &str) -> BuiltinType {
    match s {
        "bool" => BuiltinType::Bool,
        "byte" => BuiltinType::Byte,
        "char" => BuiltinType::Char,
        "uint8" => BuiltinType::Uint8,
        "uint16" => BuiltinType::Uint16,
        "uint32" => BuiltinType::Uint32,
        "uint64" => BuiltinType::Uint64,
        "int8" => BuiltinType::Int8,
        "int16" => BuiltinType::Int16,
        "int32" => BuiltinType::Int32,
        "int64" => BuiltinType::Int64,
        "float32" => BuiltinType::Float32,
        "float64" => BuiltinType::Float64,
        "time" => BuiltinType::Time,
        "duration" => BuiltinType::Duration,
        "string" => BuiltinType::String,
        _ => BuiltinType::Other,
    }
}

#[derive(Debug, Clone)]
pub struct ROSField {
    fieldname: String,
    field_type: ROSType,
    is_array: bool,
    array_size: isize,
    is_constant: bool,
    value: String,
}

impl ROSField {
    pub fn new_with_type(field_type: ROSType, name: &str) -> Self {
        ROSField {
            fieldname: name.to_string(),
            field_type: field_type,
            is_array: false,
            array_size: 1,
            is_constant: false,
            value: String::new(),
        }
    }

    pub fn new_with_definition(definition: &str) -> Self {
        let type_regex =
            Regex::new(r"[a-zA-Z][a-zA-Z0-9_]*(/[a-zA-Z][a-zA-Z0-9_]*){0,1}(\[[0-9]*\]){0,1}")
                .unwrap();
        let field_regex = Regex::new(r"[a-zA-Z][a-zA-Z0-9_]*").unwrap();
        let array_regex = Regex::new(r"(.+)(\[(\d*)\])").unwrap();

        let mut begin = definition;
        let mut type_ = String::new();
        let mut fieldname = String::new();
        let mut is_array = false;
        let mut array_size = 1;
        let mut is_constant = false;
        let mut value = String::new();

        // Find type
        if let Some(what) = type_regex.find(begin) {
            type_ = what.as_str().to_string();
            begin = &begin[what.end()..];
        } else {
            panic!("Bad type when parsing field: {}", definition);
        }

        // Find field
        if let Some(what) = field_regex.find(begin) {
            fieldname = what.as_str().to_string();
            begin = &begin[what.end()..];
        } else {
            panic!("Bad field when parsing field: {}", definition);
        }

        // Find array size
        if let Some(what) = array_regex.captures(&type_) {
            let type_ = what[1].to_string();
            if let Some(size) = what.get(3) {
                array_size = if size.as_str().is_empty() {
                    -1
                } else {
                    isize::from_str(size.as_str()).unwrap()
                };
                is_array = true;
            } else {
                array_size = -1;
                is_array = true;
            }
        }

        // Find if constant or comment
        if let Some(what) = Regex::new(r"\S").unwrap().find(begin) {
            if what.as_str() == "=" {
                begin = &begin[what.end()..];
                // Copy constant
                if type_ == "string" {
                    value = begin.to_string();
                } else {
                    if let Some(what) = Regex::new(r"\s*#").unwrap().find(begin) {
                        value = begin[..what.start()].to_string();
                    } else {
                        value = begin.to_string();
                    }
                }
                value = value.trim().to_string();
                is_constant = true;
            } else if what.as_str() == "#" {
                // Ignore comment
            } else {
                if let Some(what) = Regex::new(r"\s*#").unwrap().find(begin) {
                    value = begin[..what.start()].to_string();
                } else {
                    value = begin.to_string();
                }
            }
        }

        ROSField {
            fieldname: fieldname,
            field_type: ROSType::new(type_.as_str()),
            is_array: is_array,
            array_size: array_size,
            is_constant: is_constant,
            value: value,
        }
    }

    pub fn type_(&self) -> &ROSType {
        &self.field_type
    }

    pub fn change_type(&mut self, new_type: ROSType) {
        self.field_type = new_type;
    }
}

#[derive(Debug, Clone)]
pub struct ROSMessage {
    msg_type: ROSType,
    fields: Vec<ROSField>,
}

impl ROSMessage {
    pub fn new(def: &str) -> Self {
        let mut msg_type = ROSType::new("");
        let mut fields = Vec::new();

        let re = Regex::new(r"(^\s*$|^\s*#)").unwrap();

        let lines: Vec<&str> = def.lines().collect();
        for line in lines {
            if re.is_match(&line) {
                continue;
            }

            let line = line.trim();

            if line.starts_with("MSG:") {
                let line = &line[5..];
                msg_type = ROSType::new(line);
            } else {
                let new_field = ROSField::new_with_definition(line);
                fields.push(new_field);
            }
        }

        ROSMessage { msg_type, fields }
    }

    pub fn type_(&self) -> &ROSType {
        &self.msg_type
    }

    pub fn set_type(&mut self, new_type: ROSType) {
        self.msg_type = new_type;
    }

    pub fn fields(&self) -> &Vec<ROSField> {
        &self.fields
    }

    pub fn fields_mut(&mut self) -> &mut Vec<ROSField> {
        &mut self.fields
    }
}

fn split_multiple_message_definitions(multi_def: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut part = String::new();

    for line in multi_def.lines() {
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

pub fn parse_message_definitions(multi_def: &str, root_type: &ROSType) -> Vec<Arc<ROSMessage>> {
    let parts = split_multiple_message_definitions(multi_def);
    let mut known_type = Vec::new();
    let mut parsed_msgs = Vec::new();

    let no_type = ROSType::new("");

    for i in (0..parts.len()).rev() {
        let mut msg = Arc::new(ROSMessage::new(&parts[i]));

        if i == 0 {
            if msg.type_() == &no_type && root_type != &no_type {
                Arc::get_mut(&mut msg).unwrap().set_type(root_type.clone());
            } else if msg.type_() == &no_type && root_type == &no_type {
                println!("{}", multi_def);
                panic!("Message type unspecified");
            }
        }

        parsed_msgs.push(msg.clone());
        known_type.push(msg.type_().clone());
    }

    for msg in &mut parsed_msgs {
        for field in Arc::get_mut(msg).unwrap().fields_mut() {
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
    parsed_msgs
}
