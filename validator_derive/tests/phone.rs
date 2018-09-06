#[macro_use]
extern crate validator_derive;
extern crate validator;

use validator::{Validate, ValidationErrorsKind};


#[cfg(feature = "phone")]
#[test]
fn can_validate_phone_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(phone)]
        val: String,
    }

    let s = TestStruct {
        val: "+14152370800".to_string(),
    };

    assert!(s.validate().is_ok());
}

#[cfg(feature = "phone")]
#[test]
fn bad_phone_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(phone)]
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
        assert_eq!(err[0].code, "phone");
    } else {
        panic!("Expected field validation errors");
    }
}

#[cfg(feature = "phone")]
#[test]
fn can_specify_code_for_phone() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(phone(code = "oops"))]
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

#[cfg(feature = "phone")]
#[test]
fn can_specify_message_for_phone() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(phone(message = "oops"))]
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
