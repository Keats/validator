use validator::{Validate, ValidationError};

fn valid_custom_fn(_: &String) -> Result<(), ValidationError> {
    Ok(())
}

fn invalid_custom_fn(_: &String) -> Result<(), ValidationError> {
    Err(ValidationError::new("meh"))
}

#[test]
fn can_validate_custom_fn_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(custom)]
        val: String,
    }

    let s = TestStruct { val: "hello".to_string() };

    assert!(s.validate(|s| valid_custom_fn(s)).is_ok());
}

#[test]
fn can_fail_custom_fn_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(custom)]
        val: String,
    }

    let s = TestStruct { val: String::new() };
    let res = s.validate(|s| invalid_custom_fn(s));
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "meh");
    assert_eq!(errs["val"][0].params["value"], "");
}

#[test]
fn can_specify_message_for_custom_fn() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(custom(message = "oops"))]
        val: String,
    }
    let s = TestStruct { val: String::new() };
    let res = s.validate(|s| invalid_custom_fn(s));
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].clone().message.unwrap(), "oops");
}

#[test]
fn can_specify_code_for_custom_fn() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(custom(code = "custom_validation"))]
        val: String,
    }
    let s = TestStruct { val: String::new() };
    let res = s.validate(|s| invalid_custom_fn(s));
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].clone().code, "custom_validation");
}
