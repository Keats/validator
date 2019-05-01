#[macro_use]
extern crate validator_derive;
extern crate validator;

use validator::Validate;

#[test]
fn can_validate_range_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(min = 5, max = 10))]
        val: usize,
    }

    let s = TestStruct { val: 6 };

    assert!(s.validate().is_ok());
}

#[test]
fn value_out_of_range_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(min = 5, max = 10))]
        val: usize,
    }

    let s = TestStruct { val: 11 };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "range");
}

#[test]
fn can_specify_code_for_range() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(min = 5, max = 10, code = "oops"))]
        val: usize,
    }
    let s = TestStruct { val: 11 };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "oops");
    assert_eq!(errs["val"][0].params["value"], 11);
    assert_eq!(errs["val"][0].params["min"], 5f64);
    assert_eq!(errs["val"][0].params["max"], 10f64);
}

#[test]
fn can_specify_message_for_range() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(min = 5, max = 10, message = "oops"))]
        val: usize,
    }
    let s = TestStruct { val: 1 };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].clone().message.unwrap(), "oops");
}
