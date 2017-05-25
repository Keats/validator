#[macro_use]
extern crate validator_derive;
extern crate validator;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;
use validator::{Validate, ValidationError};


fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
    if username == "xXxShad0wxXx" {
        return Err(ValidationError::new("terrible_username"));
    }

    Ok(())
}

fn validate_signup(data: &SignupData) -> Result<(), ValidationError> {
    if data.mail.ends_with("gmail.com") && data.age == 18 {
        return Err(ValidationError::new("stupid_rule"));
    }

    Ok(())
}

#[derive(Debug, Validate, Deserialize)]
#[validate(schema(function = "validate_signup", skip_on_field_errors = "false"))]
struct SignupData {
    #[validate(email)]
    mail: String,
    #[validate(url)]
    site: String,
    #[validate(length(min = "1"), custom = "validate_unique_username")]
    #[serde(rename = "firstName")]
    first_name: String,
    #[validate(range(min = "18", max = "20"))]
    age: u32,
}


#[test]
fn is_fine_with_many_valid_validations() {
    let signup = SignupData {
        mail: "bob@bob.com".to_string(),
        site: "http://hello.com".to_string(),
        first_name: "Bob".to_string(),
        age: 18,
    };

    assert!(signup.validate().is_ok());
}

#[test]
fn failed_validation_points_to_original_field_name() {
    let signup = SignupData {
        mail: "bob@bob.com".to_string(),
        site: "http://hello.com".to_string(),
        first_name: "".to_string(),
        age: 18,
    };
    let res = signup.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("firstName"));
    assert_eq!(errs["firstName"].len(), 1);
    assert_eq!(errs["firstName"][0].code, "length");
}

#[test]
fn test_can_validate_option_fields_with_lifetime() {
    lazy_static! {
        static ref RE2: Regex = Regex::new(r"[a-z]{2}").unwrap();
    }

    #[derive(Debug, Validate)]
    struct PutStruct<'a> {
        #[validate(length(min = "1", max = "10"))]
        name: Option<&'a str>,
        #[validate(range(min = "1", max = "10"))]
        range: Option<usize>,
        #[validate(email)]
        email: Option<&'a str>,
        #[validate(url)]
        url: Option<&'a str>,
        #[validate(contains = "@")]
        text: Option<&'a str>,
        #[validate(regex = "RE2")]
        re: Option<&'a str>,
        #[validate(custom = "check_str")]
        custom: Option<&'a str>,
    }

    fn check_str(_: &str) -> Result<(), ValidationError> {
        Ok(())
    }

    let s = PutStruct {
        name: Some("al"),
        range: Some(2),
        email: Some("hi@gmail.com"),
        url: Some("http://google.com"),
        text: Some("@someone"),
        re: Some("hi"),
        custom: Some("hey"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn test_can_validate_option_fields_without_lifetime() {
    lazy_static! {
        static ref RE2: Regex = Regex::new(r"[a-z]{2}").unwrap();
    }

    #[derive(Debug, Validate)]
    struct PutStruct {
        #[validate(length(min = "1", max = "10"))]
        name: Option<String>,
        #[validate(length(min = "1", max = "10"))]
        ids: Option<Vec<usize>>,
        #[validate(range(min = "1", max = "10"))]
        range: Option<usize>,
        #[validate(email)]
        email: Option<String>,
        #[validate(url)]
        url: Option<String>,
        #[validate(contains = "@")]
        text: Option<String>,
        #[validate(regex = "RE2")]
        re: Option<String>,
        #[validate(custom = "check_str")]
        custom: Option<String>,
    }

    fn check_str(_: &str) -> Result<(), ValidationError> {
        Ok(())
    }

    let s = PutStruct {
        name: Some("al".to_string()),
        ids: Some(vec![1, 2, 3]),
        range: Some(2),
        email: Some("hi@gmail.com".to_string()),
        url: Some("http://google.com".to_string()),
        text: Some("@someone".to_string()),
        re: Some("hi".to_string()),
        custom: Some("hey".to_string()),
    };
    assert!(s.validate().is_ok());
}
