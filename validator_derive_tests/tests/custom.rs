use validator::{Validate, ValidationError};

fn valid_custom_fn(_: &String) -> Result<(), ValidationError> {
    Ok(())
}

fn another_valid_custom_fn(_: &String) -> Result<(), ValidationError> {
    Ok(())
}

fn invalid_custom_fn(_: &String) -> Result<(), ValidationError> {
    Err(ValidationError::new("meh"))
}

#[test]
fn can_validate_custom_fn_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(custom(function = valid_custom_fn))]
        val: String,
    }

    let s = TestStruct { val: "hello".to_string() };

    assert!(s.validate().is_ok());
}

#[test]
fn can_validate_multiple_custom_fn_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(custom(function = valid_custom_fn), custom(function=another_valid_custom_fn))]
        val: String,
    }

    let s = TestStruct { val: "hello".to_string() };

    assert!(s.validate().is_ok());
}

#[test]
fn can_fail_custom_fn_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(custom(function = invalid_custom_fn))]
        val: String,
    }

    let s = TestStruct { val: String::new() };
    let res = s.validate();
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
        #[validate(custom(function = invalid_custom_fn, message = "oops"))]
        val: String,
    }
    let s = TestStruct { val: String::new() };
    let res = s.validate();
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
        #[validate(custom(function = invalid_custom_fn, code = "custom_validation"))]
        val: String,
    }
    let s = TestStruct { val: String::new() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].clone().code, "custom_validation");
}

#[test]
fn can_nest_custom_validations() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(nested)]
        a: A,
    }

    #[derive(Validate)]
    struct A {
        #[validate(custom(function = custom_fn))]
        val: String,
    }

    fn custom_fn(val: &String) -> Result<(), ValidationError> {
        if val == "value" {
            Ok(())
        } else {
            Err(ValidationError::new("Invalid string"))
        }
    }

    let t = TestStruct { a: A { val: "value".to_string() } };
    assert!(t.validate().is_ok());

    let t = TestStruct { a: A { val: "invalid value".to_string() } };
    assert!(t.validate().is_err());
}
