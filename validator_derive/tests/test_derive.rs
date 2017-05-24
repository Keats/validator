#[macro_use] extern crate validator_derive;
extern crate validator;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate regex;
#[macro_use] extern crate lazy_static;

use validator::Validate;
use regex::Regex;
use validator::Error;

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

fn validate_signup(data: &SignupData) -> Option<(String, String)> {
    if data.mail.ends_with("gmail.com") && data.age == 18 {
        return Some(("all".to_string(), "stupid_rule".to_string()));
    }

    None
}

#[derive(Debug, Validate, Deserialize)]
#[validate(schema(function = "validate_signup2", skip_on_field_errors = "false"))]
struct SignupData2 {
    #[validate(email)]
    mail: String,
    #[validate(range(min = "18", max = "20"))]
    age: u32,
}

#[derive(Debug, Validate, Deserialize)]
#[validate(schema(function = "validate_signup3"))]
struct SignupData3 {
    #[validate(email, contains = "bob")]
    mail: String,
    #[validate(range(min = "18", max = "20"))]
    age: u32,
}

fn validate_signup2(data: &SignupData2) -> Option<(String, String)> {
    if data.mail.starts_with("bob") && data.age == 18 {
        return Some(("mail".to_string(), "stupid_rule".to_string()));
    }

    None
}

fn validate_signup3(_: &SignupData3) -> Option<(String, String)> {
    Some(("mail".to_string(), "stupid_rule".to_string()))
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
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("mail"));
    assert_eq!(errs["mail"], vec![Error::new("email", "not a valid e-mail address")]);
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
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("site"));
    assert_eq!(errs["site"], vec![Error::new("url", "not a valid URL")]);
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
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("firstName"));
    assert_eq!(errs["firstName"], vec![Error::new("length", "must be at least 1 characters long")]);
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
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("age"));
    assert_eq!(errs["age"], vec![Error::new("range", "must be between 18 and 20")]);
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
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("age"));
    assert!(errs.contains_key("firstName"));
    assert_eq!(errs["age"], vec![Error::new("range", "must be between 18 and 20")]);
    assert_eq!(errs["firstName"], vec![Error::new("length", "must be at least 1 characters long")]);
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
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("firstName"));
    assert_eq!(errs["firstName"], vec![Error::new("custom", "terrible_username")]);
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
    let res = data.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert_eq!(errs["password"], vec![Error::new("no_match", "field 'password' must match field 'password2'")]);
}

#[test]
fn test_can_fail_struct_validation_new_key() {
    let signup = SignupData {
        mail: "bob@gmail.com".to_string(),
        site: "https://hello.com".to_string(),
        first_name: "xXxShad0wxXx".to_string(),
        age: 18,
    };
    let res = signup.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("all"));
    assert_eq!(errs["all"], vec![Error::new("stupid_rule", "errorMessage")]);
}

#[test]
fn test_can_fail_struct_validation_existing_key() {
    let signup = SignupData2 {
        mail: "bob".to_string(),
        age: 18,
    };
    let res = signup.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("mail"));
    assert_eq!(errs["mail"], vec![Error::new("email", "not a valid e-mail address"), Error::new("stupid_rule", "errorMessage")]);
}

#[test]
fn test_skip_struct_validation_by_default_if_errors() {
    let signup = SignupData3 {
        mail: "bob".to_string(),
        age: 18,
    };
    let res = signup.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("mail"));
    assert_eq!(errs["mail"], vec![Error::new("email", "not a valid e-mail address")]);
}

#[test]
fn test_can_fail_contains_validation() {
    let signup = SignupData3 {
        mail: "bo@gmail.com".to_string(),
        age: 18,
    };
    let res = signup.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("mail"));
    assert_eq!(errs["mail"], vec![Error::new("contains", "errorMessage")]);
}

#[test]
fn test_can_check_regex_validator() {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[a-z]{2}").unwrap();
    }

    #[derive(Debug, Validate)]
    struct RegexStruct {
        #[validate(regex = "RE")]
        name: String,
    }
    let s = RegexStruct {name: "al".to_string()};
    assert!(s.validate().is_ok());
    let s2 = RegexStruct {name: "AL".to_string()};
    assert!(s2.validate().is_err());
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

    fn check_str(_: &str) -> Option<String> {
        None
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

    fn check_str(_: &str) -> Option<String> {
        None
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
