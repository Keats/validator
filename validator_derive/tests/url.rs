#[macro_use]
extern crate validator_derive;
extern crate validator;

use validator::{Validate, ValidationErrorsKind};


#[test]
fn can_validate_url_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(url)]
        val: String,
    }

    let s = TestStruct {
        val: "https://google.com".to_string(),
    };

    assert!(s.validate().is_ok());
}

#[test]
fn bad_url_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(url)]
        val: String,
    }

    let s = TestStruct {
        val: "bob".to_string(),
    };
    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("val"));
    if let ValidationErrorsKind::Field(ref err) = errs["val"] {
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].code, "url");
    } else {
        panic!("Expected field validation errors");
    }
}

#[test]
fn can_specify_code_for_url() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(url(code = "oops"))]
        val: String,
    }
    let s = TestStruct {
        val: "bob".to_string(),
    };
    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("val"));
    if let ValidationErrorsKind::Field(ref err) = errs["val"] {
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].code, "oops");
        assert_eq!(err[0].params["value"], "bob");
    } else {
        panic!("Expected field validation errors");
    }
}

#[test]
fn can_specify_message_for_url() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(url(message = "oops"))]
        val: String,
    }
    let s = TestStruct {
        val: "bob".to_string(),
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
