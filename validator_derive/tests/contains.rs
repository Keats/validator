#[macro_use]
extern crate validator_derive;
extern crate validator;

use validator::{Validate, ValidationErrorsKind};

#[test]
fn can_validate_contains_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(contains = "he")]
        val: String,
    }

    let s = TestStruct {
        val: "hello".to_string(),
    };

    assert!(s.validate().is_ok());
}

#[test]
fn value_not_containing_needle_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(contains = "he")]
        val: String,
    }

    let s = TestStruct {
        val: String::new(),
    };
    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("val"));
    if let ValidationErrorsKind::Field(ref err) = errs["val"] {
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].code, "contains");
        assert_eq!(err[0].params["value"], "");
        assert_eq!(err[0].params["needle"], "he");
    } else {
        panic!("Expected field validation errors");
    }
}

#[test]
fn can_specify_code_for_contains() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(contains(pattern = "he", code = "oops"))]
        val: String,
    }
    let s = TestStruct {
        val: String::new(),
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
fn can_specify_message_for_contains() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(contains(pattern = "he", message = "oops"))]
        val: String,
    }
    let s = TestStruct {
        val: String::new(),
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
