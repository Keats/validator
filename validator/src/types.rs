use std::{self, fmt};
use std::collections::HashMap;

type Field = String;

#[derive(Debug)]
pub struct Errors(HashMap<Field, Vec<Error>>);

impl Errors {
    pub fn new() -> Errors {
        Errors(HashMap::new())
    }

    pub fn inner(self) -> HashMap<Field, Vec<Error>> {
        self.0
    }

    pub fn add(
        &mut self,
        field: &str,
        validator: &str,
        err: &str
    ) {
        let error = Error::new(validator, err);
        self.0.entry(field.to_string())
            .or_insert_with(|| vec![])
            .push(error);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for Errors {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Validation failed:\n")?;
        for (field, errs) in &self.0 {
            write!(fmt, "    {}: [", field)?;

            let last = errs.len() - 1;
            for (index, err) in errs.iter().enumerate() {
                write!(fmt, "{}", err)?;
                if index < last { write!(fmt, ", ")? }
            }
            write!(fmt, "]\n")?;
        }
        Ok(())
    }
}

impl std::error::Error for Errors {
    fn description(&self) -> &str {
        "validation failed"
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

#[derive(Debug, PartialEq)]
pub struct Error {
    validator: String,
    message: String,
}

impl Error {
    pub fn new<T: Into<String>>(
        validator: T,
        message: T
    ) -> Error {
        Error {
            validator: validator.into(),
            message: message.into(),
        }
    }

    pub fn validator(&self) -> &str {
        &self.validator
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Error({}): {}", self.validator, self.message)?;
        Ok(())
    }
}

pub trait Validate {
    fn validate(&self) -> Result<(), Errors>;
}

/// Contains all the validators that can be used
///
/// In this crate as it's not allowed to export more than the proc macro
/// in a proc macro crate
#[derive(Debug, Clone)]
pub enum Validator {
    // String is the path to the function
    Custom(String),
    // String is the name of the field to match
    MustMatch(String),
    // value is a &str
    Email,
    // value is a &str
    Url,
    // value is a &str or a HashMap<String, ..>
    Contains(String),
    // value is a &str
    Regex(String),
    // value is a number
    Range {
        min: f64,
        max: f64,
    },
    // value is anything that impl HasLen
    Length {
        min: Option<u64>,
        max: Option<u64>,
        equal: Option<u64>,
    },
}
