use validator::{Validate, ValidationError};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CountryCode(pub String);

fn validate_country_code(_country_code: &Option<CountryCode>) -> Result<(), ValidationError> {
    if false {
        Ok(())
    } else {
        Err(ValidationError::new("not a valid ISO-3166-1 alpha-2 country code"))
    }
}

#[derive(Debug, Validate)]
pub struct Foo {
    #[validate(custom(function = "validate_country_code"))]
    pub country: Option<CountryCode>,
    pub country2: Option<std::option::Option<CountryCode>>,
    #[validate(range(min = 5, max = 10))]
    pub age: Option<i32>,
}
fn main() {}
