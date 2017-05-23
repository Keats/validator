use std::{self, fmt};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct Errors(HashMap<String, Vec<String>>);

impl Errors {
    pub fn new() -> Errors {
        Errors(HashMap::new())
    }

    pub fn inner(self) -> HashMap<String, Vec<String>> {
        self.0
    }

    pub fn add(&mut self, field: &str, err: &str) {
        self.0.entry(field.to_string()).or_insert_with(|| vec![]).push(err.to_string());
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
