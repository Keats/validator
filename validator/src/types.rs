use std::{self, fmt};
use std::borrow::Cow;
use std::collections::HashMap;

use serde_json::{Value, to_value};
use serde::ser::Serialize;


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub path: Vec<String>,
    pub code: Cow<'static, str>,
    pub message: Option<Cow<'static, str>>,
    pub params: HashMap<Cow<'static, str>, Value>,
}

impl ValidationError {
    pub fn new(code: &'static str) -> ValidationError {
        ValidationError {
            path: Vec::new(),
            code: Cow::from(code),
            message: None,
            params: HashMap::new(),
        }
    }

    pub fn set_path(mut self, path: Vec<String>) -> Self {
        self.path.extend(path.into_iter().clone());
        self
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
pub struct ValidationErrors(HashMap<String, Vec<ValidationError>>, Vec<String>);


impl ValidationErrors {
    pub fn merge_results(a: Result<(), ValidationErrors>, b: Result<(), ValidationErrors>) -> Result<(), ValidationErrors> {
        match a {
            Ok(()) => b,
            Err(a_errors) => match b {
                Ok(()) => Err(a_errors),
                Err(b_errors) => Err(b_errors.merge(a_errors))
            }
        }
    }

    pub fn new() -> ValidationErrors {
        ValidationErrors(HashMap::new(), Vec::new())
    }

    pub fn set_path(mut self, path: &FieldPath) -> Self {
        self.1.extend(path.to_vec());
        self
    }

    pub fn inner(self) -> HashMap<String, Vec<ValidationError>> {
        self.0
    }

    pub fn add(&mut self, field: &'static str, error: ValidationError) {
        let path = FieldPath::concat(Some(String::from(field)), Some(&self.1));
        self.0.entry(path.join(".")).or_insert_with(|| vec![]).push(error.set_path(path));
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

pub struct FieldPath(Vec<String>);

impl FieldPath {
    pub fn concat(field: Option<String>, path: Option<&Vec<String>>) -> Vec<String> {
        let mut vec = path.map_or(Vec::new(), |p| p.to_vec());
        if let Some(f) = field {
            vec.push(f);
        }
        vec
    }

    pub fn new(field: Option<String>, path: Option<&FieldPath>) -> FieldPath {
        FieldPath(FieldPath::concat(field, path.map(|p| &p.0)))
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.0.to_vec()
    }
}