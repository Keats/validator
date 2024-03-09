use validator::{Validate, ValidationError};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CountryCode(pub String);

impl CountryCode {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if true {
            Ok(())
        } else {
            Err(ValidationError::new("not a valid ISO-3166-1 alpha-2 country code"))
        }
    }
}

#[derive(Debug, Validate)]
pub struct Foo {
    #[validate(custom(function = "CountryCode::validate"))]
    pub country: Option<CountryCode>,
    pub country2: Option<std::option::Option<CountryCode>>,
    #[validate(range(min = 5, max = 10))]
    pub age: Option<i32>,
}
fn main() {}
