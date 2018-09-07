#![allow(deprecated)]

#[macro_use]
extern crate validator_derive;
extern crate validator;

use validator::Validate;

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
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "length");
    assert_eq!(errs["val"][0].params["value"], "");
    assert_eq!(errs["val"][0].params["min"], 5);
    assert_eq!(errs["val"][0].params["max"], 10);
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
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "oops");
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
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].clone().message.unwrap(), "oops");
}
