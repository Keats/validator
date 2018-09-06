#[macro_use]
extern crate validator_derive;
extern crate validator;

use validator::{Validate, ValidationErrorsKind};

#[test]
fn can_validate_length_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = "5", max = "10"))]
        val: String,
    }

    let s = TestStruct {
        val: "hello".to_string(),
    };

    assert!(s.validate().is_ok());
}

#[test]
fn value_out_of_length_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = "5", max = "10"))]
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
        assert_eq!(err[0].code, "length");
        assert_eq!(err[0].params["value"], "");
        assert_eq!(err[0].params["min"], 5);
        assert_eq!(err[0].params["max"], 10);
    } else {
        panic!("Expected field validation errors");
    }
}

#[test]
fn can_specify_code_for_length() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = "5", max = "10", code = "oops"))]
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
fn can_specify_message_for_length() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = "5", max = "10", message = "oops"))]
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
