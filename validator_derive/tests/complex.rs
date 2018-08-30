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
use validator::{Validate, ValidationError, ValidationErrors};


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
    #[validate]
    phone: Phone,
    #[validate]
    card: Option<Card>,
    #[validate]
    preferences: Vec<Preference>
}

#[derive(Debug, Validate, Deserialize)]
struct Phone {
    #[validate(phone)]
    number: String
}

#[derive(Debug, Validate, Deserialize)]
struct Card {
    #[validate(credit_card)]
    number: String,
    #[validate(range(min = "100", max = "9999"))]
    cvv: u32,
}

#[derive(Debug, Validate, Deserialize)]
struct Preference {
    #[validate(length(min = "4"))]
    name: String,
    value: bool,
}

#[test]
fn is_fine_with_many_valid_validations() {
    let signup = SignupData {
        mail: "bob@bob.com".to_string(),
        site: "http://hello.com".to_string(),
        first_name: "Bob".to_string(),
        age: 18,
        phone: Phone {
            number: "+14152370800".to_string()
        },
        card: Some(Card {
            number: "5236313877109142".to_string(),
            cvv: 123
        }),
        preferences: vec![
            Preference {
                name: "marketing".to_string(),
                value: false
            },
        ]
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
        phone: Phone {
            number: "123 invalid".to_string(),
        },
        card: Some(Card {
            number: "1234567890123456".to_string(),
            cvv: 1
        }),
        preferences: vec![
            Preference {
                name: "abc".to_string(),
                value: true
            },
        ]
    };
    let res = signup.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("firstName"));
    assert_eq!(errs["firstName"].len(), 1);
    assert_eq!(errs["firstName"][0].code, "length");
    assert_eq!(errs["firstName"][0].path, vec!["firstName"]);
    assert!(errs.contains_key("phone.number"));
    assert_eq!(errs["phone.number"].len(), 1);
    assert_eq!(errs["phone.number"][0].code, "phone");
    assert_eq!(errs["phone.number"][0].path, vec!["phone", "number"]);
    assert!(errs.contains_key("card.number"));
    assert_eq!(errs["card.number"].len(), 1);
    assert_eq!(errs["card.number"][0].code, "credit_card");
    assert_eq!(errs["card.number"][0].path, vec!["card", "number"]);
    assert!(errs.contains_key("card.cvv"));
    assert_eq!(errs["card.cvv"].len(), 1);
    assert_eq!(errs["card.cvv"][0].code, "range");
    assert_eq!(errs["card.cvv"][0].path, vec!["card", "cvv"]);
    assert!(errs.contains_key("preferences.0.name"));
    assert_eq!(errs["preferences.0.name"].len(), 1);
    assert_eq!(errs["preferences.0.name"][0].code, "length");
    assert_eq!(errs["preferences.0.name"][0].path, vec!["preferences", "0", "name"]);
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
        #[validate(length(min = "1", max = "10"))]
        address: Option<Option<&'a str>>,
        #[validate(range(min = "1", max = "100"))]
        age: Option<Option<usize>>,
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
        address: Some(Some("gol")),
        age: Some(Some(20)),
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
        address: Option<Option<String>>,
        #[validate(length(min = "1", max = "10"))]
        ids: Option<Vec<usize>>,
        #[validate(length(min = "1", max = "10"))]
        opt_ids: Option<Option<Vec<usize>>>,
        #[validate(range(min = "1", max = "100"))]
        age: Option<Option<usize>>,
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
        address: Some(Some("gol".to_string())),
        ids: Some(vec![1, 2, 3]),
        opt_ids: Some(Some(vec![1, 2, 3])),
        age: Some(Some(20)),
        range: Some(2),
        email: Some("hi@gmail.com".to_string()),
        url: Some("http://google.com".to_string()),
        text: Some("@someone".to_string()),
        re: Some("hi".to_string()),
        custom: Some("hey".to_string()),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn test_works_with_question_mark_operator() {
    fn some_fn() -> Result<(), ValidationErrors> {
        let signup = SignupData {
            mail: "invalid_email".to_string(),
            site: "http://hello.com".to_string(),
            first_name: "Bob".to_string(),
            age: 18,
            phone: Phone {
                number: "+14152370800".to_string()
            },
            card: None,
            preferences: Vec::new(),
        };

        signup.validate()?;
        Ok(())
    }

    assert!(some_fn().is_err());
}

#[test]
fn test_works_with_none_values() {
    #[derive(Debug, Validate)]
    struct PutStruct {
        #[validate(length(min = "1", max = "10"))]
        name: Option<String>,
        #[validate(length(min = "1", max = "10"))]
        address: Option<Option<String>>,
        #[validate(range(min = "1", max = "100"))]
        age: Option<Option<usize>>,
        #[validate(range(min = "1", max = "10"))]
        range: Option<usize>,
    }

    let p = PutStruct {
        name: None,
        address: None,
        age: None,
        range: None,
    };

    let q = PutStruct {
        name: None,
        address: Some(None),
        age: Some(None),
        range: None,
    };

    assert!(p.validate().is_ok());
    assert!(q.validate().is_ok());
}