use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ROSType {
    pkg_name: String,
    msg_name: String,
}

impl ROSType {
    pub fn new(pkg_name: &str, msg_name: &str) -> Self {
        ROSType {
            pkg_name: pkg_name.to_string(),
            msg_name: msg_name.to_string(),
        }
    }

    pub fn pkg_name(&self) -> &str {
        &self.pkg_name
    }

    pub fn msg_name(&self) -> &str {
        &self.msg_name
    }
}

#[derive(Clone)]
pub struct ROSField {
    field_type: ROSType,
}

impl ROSField {
    pub fn new(field_type: ROSType) -> Self {
        ROSField { field_type }
    }

    pub fn type_(&self) -> &ROSType {
        &self.field_type
    }

    pub fn change_type(&mut self, new_type: ROSType) {
        self.field_type = new_type;
    }
}

#[derive(Clone)]
pub struct ROSMessage {
    msg_type: ROSType,
    fields: Vec<ROSField>,
}

impl ROSMessage {
    pub fn new(def: &str) -> Self {
        // Placeholder for actual parsing logic
        ROSMessage {
            msg_type: ROSType::new("", ""),
            fields: vec![],
        }
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

    let no_type = ROSType::new("", "");

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

    for mut msg in &mut parsed_msgs {
        for mut field in Arc::get_mut(msg).unwrap().fields_mut() {
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
