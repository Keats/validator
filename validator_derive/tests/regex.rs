extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate validator_derive;
extern crate validator;

use validator::{Validate, ValidationErrorsKind};
use regex::Regex;

lazy_static! {
    static ref RE2: Regex = Regex::new(r"^[a-z]{2}$").unwrap();
}

#[test]
fn can_validate_valid_regex() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(regex = "RE2")]
        val: String,
    }

    let s = TestStruct {
        val: "aa".to_string(),
    };

    assert!(s.validate().is_ok());
}

#[test]
fn bad_value_for_regex_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(regex = "RE2")]
        val: String,
    }

    let s = TestStruct {
        val: "2".to_string(),
    };
    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("val"));
    if let ValidationErrorsKind::Field(ref err) = errs["val"] {
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].code, "regex");
        assert_eq!(err[0].params["value"], "2");
    } else {
        panic!("Expected field validation errors");
    }
}

#[test]
fn can_specify_code_for_regex() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(regex(path = "RE2", code = "oops"))]
        val: String,
    }
    let s = TestStruct {
        val: "2".to_string(),
    };
    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("val"));
    if let ValidationErrorsKind::Field(ref err) = errs["val"] {
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].code, "oops");
    } else {
        panic!("Expected field validation errors");
    }
}

#[test]
fn can_specify_message_for_regex() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(regex(path = "RE2", message = "oops"))]
        val: String,
    }
    let s = TestStruct {
        val: "2".to_string(),
    };
    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("val"));
    if let ValidationErrorsKind::Field(ref err) = errs["val"] {
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].clone().message.unwrap(), "oops");
    } else {
        panic!("Expected field validation errors");
    }
}
