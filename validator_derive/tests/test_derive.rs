#![feature(attr_literals)]

#[macro_use] extern crate validator_derive;
extern crate validator;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use validator::Validate;


#[derive(Debug, Validate, Deserialize)]
struct SignupData {
    #[validate(email)]
    mail: String,
    #[validate(url)]
    site: String,
    #[validate(length(min = 1), custom = "validate_unique_username")]
    #[serde(rename = "firstName")]
    first_name: String,
    #[validate(range(min = 18, max = 20))]
    age: u32,
}

#[derive(Debug, Validate)]
struct PasswordData {
    #[validate(must_match = "password2")]
    password: String,
    password2: String,
}


fn validate_unique_username(username: &str) -> Option<String> {
    if username == "xXxShad0wxXx" {
        return Some("terrible_username".to_string());
    }

    None
}

#[test]
fn test_can_validate_ok() {
    let signup = SignupData {
        mail: "bob@bob.com".to_string(),
        site: "http://hello.com".to_string(),
        first_name: "Bob".to_string(),
        age: 18,
    };

    assert!(signup.validate().is_ok());
}

#[test]
fn test_bad_email_fails_validation() {
    let signup = SignupData {
        mail: "bob".to_string(),
        site: "http://hello.com".to_string(),
        first_name: "Bob".to_string(),
        age: 18,
    };
    let res = signup.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err();
    assert!(errs.contains_key("mail"));
    assert_eq!(errs["mail"], vec!["email".to_string()]);
}

#[test]
fn test_bad_url_fails_validation() {
    let signup = SignupData {
        mail: "bob@bob.com".to_string(),
        site: "//hello.com".to_string(),
        first_name: "Bob".to_string(),
        age: 18,
    };
    let res = signup.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err();
    assert!(errs.contains_key("site"));
    assert_eq!(errs["site"], vec!["url".to_string()]);
}

#[test]
fn test_bad_length_fails_validation_and_points_to_original_name() {
    let signup = SignupData {
        mail: "bob@bob.com".to_string(),
        site: "http://hello.com".to_string(),
        first_name: "".to_string(),
        age: 18,
    };
    let res = signup.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err();
    println!("{:?}", errs);
    assert!(errs.contains_key("firstName"));
    assert_eq!(errs["firstName"], vec!["length".to_string()]);
}


#[test]
fn test_bad_range_fails_validation() {
    let signup = SignupData {
        mail: "bob@bob.com".to_string(),
        site: "https://hello.com".to_string(),
        first_name: "Bob".to_string(),
        age: 1,
    };
    let res = signup.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err();
    assert!(errs.contains_key("age"));
    assert_eq!(errs["age"], vec!["range".to_string()]);
}

#[test]
fn test_can_have_multiple_errors() {
    let signup = SignupData {
        mail: "bob@bob.com".to_string(),
        site: "https://hello.com".to_string(),
        first_name: "".to_string(),
        age: 1,
    };
    let res = signup.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err();
    assert!(errs.contains_key("age"));
    assert!(errs.contains_key("firstName"));
    assert_eq!(errs["age"], vec!["range".to_string()]);
    assert_eq!(errs["firstName"], vec!["length".to_string()]);
}

#[test]
fn test_custom_validation_error() {
    let signup = SignupData {
        mail: "bob@bob.com".to_string(),
        site: "https://hello.com".to_string(),
        first_name: "xXxShad0wxXx".to_string(),
        age: 18,
    };
    let res = signup.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err();
    assert!(errs.contains_key("firstName"));
    assert_eq!(errs["firstName"], vec!["terrible_username".to_string()]);
}

#[test]
fn test_must_match_can_work() {
    let data = PasswordData {
        password: "passw0rd".to_string(),
        password2: "passw0rd".to_string(),
    };
    assert!(data.validate().is_ok())
}


#[test]
fn test_must_match_can_fail() {
    let data = PasswordData {
        password: "passw0rd".to_string(),
        password2: "password".to_string(),
    };
    assert!(data.validate().is_err())
}
