#[macro_use]
extern crate validator_derive;
extern crate validator;

use validator::{Validate, ValidationErrorsKind};


#[test]
fn can_validate_valid_must_match() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(must_match = "val2")]
        val: String,
        val2: String,
    }

    let s = TestStruct {
        val: "bob".to_string(),
        val2: "bob".to_string(),
    };

    assert!(s.validate().is_ok());
}

#[test]
fn not_matching_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(must_match = "val2")]
        val: String,
        val2: String,
    }

    let s = TestStruct {
        val: "bob".to_string(),
        val2: "bobby".to_string(),
    };

    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("val"));
    if let ValidationErrorsKind::Field(ref err) = errs["val"] {
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].code, "must_match");
        assert_eq!(err[0].params["value"], "bob");
        assert_eq!(err[0].params["other"], "bobby");
    } else {
        panic!("Expected field validation errors");
    }
}

#[test]
fn can_specify_code_for_must_match() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(must_match(other = "val2", code = "oops"))]
        val: String,
        val2: String,
    }
    let s = TestStruct {
        val: "bob".to_string(),
        val2: "bobb".to_string(),
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
fn can_specify_message_for_must_match() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(must_match(other = "val2", message = "oops"))]
        val: String,
        val2: String,
    }
    let s = TestStruct {
        val: "bob".to_string(),
        val2: "bobb".to_string(),
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
