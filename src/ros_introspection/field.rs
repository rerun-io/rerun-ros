use crate::ros_introspection::Type;
use anyhow::Result;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Field {
    fieldname: String,
    field_type: Type,
    is_array: bool,
    array_size: isize,
    is_constant: bool,
    value: String,
}

impl Field {
    /// Creates a new `Field` instance with the given type and name.
    ///
    /// # Arguments
    ///
    /// * `field_type` - The `Type` of the field.
    /// * `name` - A string slice that holds the name of the field.
    ///
    /// # Returns
    ///
    /// * `Self` - The new `Field` instance.
    pub fn new_with_type(field_type: Type, name: &str) -> Self {
        Self {
            fieldname: name.to_owned(),
            field_type,
            is_array: false,
            array_size: 1,
            is_constant: false,
            value: String::new(),
        }
    }

    /// Creates a new `Field` instance from a definition string.
    ///
    /// # Arguments
    ///
    /// * `definition` - A string slice that holds the definition of the field.
    ///
    /// # Returns
    ///
    /// * `Result<Self>` - A result containing the new `Field` instance or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The regular expression for parsing the type, field, or array fails to compile.
    /// - The type, field, or array size cannot be extracted from the definition.
    /// - The array size is not a valid integer.
    pub fn new_with_definition(definition: &str) -> Result<Self> {
        let type_regex =
            Regex::new(r"[a-zA-Z][a-zA-Z0-9_]*(/[a-zA-Z][a-zA-Z0-9_]*){0,1}(\[[0-9]*\]){0,1}")?;
        let field_regex = Regex::new(r"[a-zA-Z][a-zA-Z0-9_]*")?;
        let array_regex = Regex::new(r"(.+)(\[(\d*)\])")?;

        let mut begin = definition;

        // Find type
        let mut type_ = if let Some(what) = type_regex.find(begin) {
            begin = &begin[what.end()..];
            what.as_str().to_owned()
        } else {
            return Err(anyhow::anyhow!("Bad type when parsing field: {definition}"));
        };

        // Find field
        let fieldname = if let Some(what) = field_regex.find(begin) {
            begin = &begin[what.end()..];
            what.as_str().to_owned()
        } else {
            return Err(anyhow::anyhow!(
                "Bad field when parsing field: {definition}"
            ));
        };

        // Find array size
        // Clone type_ to avoid borrowing issues
        let temp_type = type_.clone();
        let (is_array, array_size) = if let Some(what) = array_regex.captures(&temp_type) {
            type_ = what[1].to_string();
            if what.len() == 3 {
                (true, -1)
            } else if let Some(size) = what.get(3) {
                let array_size = if size.as_str().is_empty() {
                    -1
                } else {
                    isize::from_str(size.as_str())?
                };
                (true, array_size)
            } else {
                (true, -1)
            }
        } else {
            (false, 1)
        };

        // Find if constant or comment
        let (is_constant, value) = if let Some(what) = Regex::new(r"\S")?.find(begin) {
            if what.as_str() == "=" {
                begin = &begin[what.end()..];
                // Copy constant
                let value = if type_ == "string" {
                    begin.to_owned()
                } else if let Some(what) = Regex::new(r"\s*#")?.find(begin) {
                    begin[..what.start()].to_string()
                } else {
                    begin.to_owned()
                }
                .trim()
                .to_owned();
                (true, value)
            } else if what.as_str() == "#" {
                // Ignore comment
                (false, String::default())
            } else {
                let value = if let Some(what) = Regex::new(r"\s*#")?.find(begin) {
                    begin[..what.start()].to_string()
                } else {
                    begin.to_owned()
                };
                (false, value)
            }
        } else {
            (false, String::default())
        };

        Ok(Self {
            fieldname,
            field_type: Type::new(type_.as_str())?,
            is_array,
            array_size,
            is_constant,
            value,
        })
    }

    /// Returns a reference to the type of the field.
    ///
    /// # Returns
    ///
    /// * `&Type` - A reference to the `Type` of the field.
    pub fn type_(&self) -> &Type {
        &self.field_type
    }

    /// Changes the type of the field.
    ///
    /// # Arguments
    ///
    /// * `new_type` - The new `Type` of the field.
    pub fn change_type(&mut self, new_type: Type) {
        self.field_type = new_type;
    }

    /// Returns the name of the field.
    ///
    /// # Returns
    ///
    /// * `&str` - A string slice that holds the name of the field.
    pub fn name(&self) -> &str {
        &self.fieldname
    }

    /// Returns whether the field is an array.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the field is an array, `false` otherwise.
    pub fn is_array(&self) -> bool {
        self.is_array
    }

    /// Returns whether the field is a constant.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the field is a constant, `false` otherwise.
    pub fn is_constant(&self) -> bool {
        self.is_constant
    }

    /// Returns the array size of the field.
    ///
    /// # Returns
    ///
    /// * `isize` - The array size of the field.
    pub fn array_size(&self) -> isize {
        self.array_size
    }

    /// Returns the value of the field.
    ///
    /// # Returns
    ///
    /// * `&str` - A string slice that holds the value of the field.
    pub fn value(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ros_introspection::Type;

    #[test]
    fn test_new_with_type() {
        let field_type = Type::new("int32").unwrap();
        let field = Field::new_with_type(field_type.clone(), "test_field");

        assert_eq!(field.fieldname, "test_field");
        assert_eq!(field.field_type, field_type);
        assert!(!field.is_array);
        assert_eq!(field.array_size, 1);
        assert!(!field.is_constant);
        assert_eq!(field.value, "");
    }

    #[test]
    fn test_new_with_definition() {
        let field = Field::new_with_definition("int32 test_field").unwrap();
        assert_eq!(field.fieldname, "test_field");
        assert_eq!(field.field_type, Type::new("int32").unwrap());
        assert!(!field.is_array);
        assert_eq!(field.array_size, 1);
        assert!(!field.is_constant);
        assert_eq!(field.value, "");

        let field = Field::new_with_definition("string[10] test_array").unwrap();
        assert_eq!(field.fieldname, "test_array");
        assert_eq!(field.field_type, Type::new("string").unwrap());
        assert!(field.is_array);
        assert_eq!(field.array_size, 10);
        assert!(!field.is_constant);
        assert_eq!(field.value, "");

        let field = Field::new_with_definition("float64 PI = 3.14159").unwrap();
        assert_eq!(field.fieldname, "PI");
        assert_eq!(field.field_type, Type::new("float64").unwrap());
        assert!(!field.is_array);
        assert_eq!(field.array_size, 1);
        assert!(field.is_constant);
        assert_eq!(field.value, "3.14159");
    }

    #[test]
    fn test_getters() {
        let field = Field::new_with_type(Type::new("int32").unwrap(), "test_field");

        assert_eq!(field.type_(), &Type::new("int32").unwrap());
        assert_eq!(field.name(), "test_field");
        assert!(!field.is_array());
        assert!(!field.is_constant());
        assert_eq!(field.array_size(), 1);
        assert_eq!(field.value(), "");
    }

    #[test]
    fn test_change_type() {
        let mut field = Field::new_with_type(Type::new("int32").unwrap(), "test_field");
        let new_type = Type::new("float64").unwrap();
        field.change_type(new_type.clone());

        assert_eq!(field.type_(), &new_type);
    }
}
