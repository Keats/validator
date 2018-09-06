#[macro_use]
extern crate validator_derive;
extern crate validator;

use validator::{Validate, ValidationErrorsKind};

#[test]
fn can_validate_range_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(min = "5", max = "10"))]
        val: usize,
    }

    let s = TestStruct {
        val: 6,
    };

    assert!(s.validate().is_ok());
}

#[test]
fn value_out_of_range_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(min = "5", max = "10"))]
        val: usize,
    }

    let s = TestStruct {
        val: 11,
    };
    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("val"));
    if let ValidationErrorsKind::Field(ref err) = errs["val"] {
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].code, "range");
    } else {
        panic!("Expected field validation errors");
    }
}

#[test]
fn can_specify_code_for_range() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(min = "5", max = "10", code = "oops"))]
        val: usize,
    }
    let s = TestStruct {
        val: 11,
    };
    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("val"));
    if let ValidationErrorsKind::Field(ref err) = errs["val"] {
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].code, "oops");
        assert_eq!(err[0].params["value"], 11);
        assert_eq!(err[0].params["min"], 5f64);
        assert_eq!(err[0].params["max"], 10f64);
    } else {
        panic!("Expected field validation errors");
    }
}

#[test]
fn can_specify_message_for_range() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(min = "5", max = "10", message = "oops"))]
        val: usize,
    }
    let s = TestStruct {
        val: 1,
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
