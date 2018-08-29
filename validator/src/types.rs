use std::{self, fmt};
use std::borrow::Cow;
use std::collections::HashMap;

use serde_json::{Value, to_value};
use serde::ser::Serialize;


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValidationError {
  pub code: Cow<'static, str>,
  pub message: Option<Cow<'static, str>>,
  pub params: HashMap<Cow<'static, str>, Value>,
}

impl ValidationError {
    pub fn new(code: &'static str) -> ValidationError {
        ValidationError {
            code: Cow::from(code),
            message: None,
            params: HashMap::new(),
        }
    }

    pub fn add_param<T: Serialize>(&mut self, name: Cow<'static, str>, val: &T) {
        self.params.insert(name, to_value(val).unwrap());
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Validation error: {} [{:?}]", self.code, self.params)
    }
}

impl std::error::Error for ValidationError {
  fn description(&self) -> &str { &self.code }
  fn cause(&self) -> Option<&std::error::Error> { None }
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ValidationErrors(HashMap<&'static str, Vec<ValidationError>>);


impl ValidationErrors {
    pub fn new() -> ValidationErrors {
        ValidationErrors(HashMap::new())
    }

    pub fn inner(self) -> HashMap<&'static str, Vec<ValidationError>> {
        self.0
    }

    pub fn add(&mut self, field: &'static str, error: ValidationError) {
        self.0.entry(field).or_insert_with(|| vec![]).push(error);
    }

    pub fn merge(mut self, other: ValidationErrors) -> Self {
        self.0.extend(other.inner());
        self
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl std::error::Error for ValidationErrors {
    fn description(&self) -> &str { "Validation failed" }
    fn cause(&self) -> Option<&std::error::Error> { None }
}

impl fmt::Display for ValidationErrors {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, fmt)
    }
}
