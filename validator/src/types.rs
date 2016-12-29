use std::collections::HashMap;


pub type Errors = HashMap<String, Vec<String>>;

pub trait Validate {
    //fn load_and_validate<T>(data: &str) -> Result<T, Errors>;
    fn validate(&self) -> Result<(), Errors>;
}

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
