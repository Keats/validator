use std::{borrow::Cow, collections::HashMap};

use validator::{Validate, ValidationError, ValidationErrors, ValidationErrorsKind};

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
fn can_fail_multiple_custom_fn_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(custom(function = invalid_custom_fn), custom(function=valid_custom_fn))]
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

#[test]
fn custom_fn_on_optional_types_work() {
    fn number_type_custom_fn(val: i16) -> Result<(), ValidationError> {
        if val == 0 {
            Ok(())
        } else {
            Err(ValidationError::new("custom"))
        }
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(custom(function = number_type_custom_fn))]
        plain: i16,
        #[validate(custom(function = number_type_custom_fn))]
        option: Option<i16>,
        #[validate(custom(function = number_type_custom_fn))]
        option_option: Option<Option<i16>>,
    }

    let t = TestStruct { plain: 0, option: Some(0), option_option: Some(Some(0)) };
    assert!(t.validate().is_ok());

    let t = TestStruct { plain: 1, option: Some(1), option_option: Some(Some(1)) };
    let mut error = ValidationError::new("custom");
    error.add_param("value".into(), &1);
    let error_kind = ValidationErrorsKind::Field(vec![{ error }]);
    assert_eq!(
        t.validate(),
        Err(ValidationErrors(HashMap::from_iter([
            (Cow::Borrowed("plain"), error_kind.clone()),
            (Cow::Borrowed("option"), error_kind.clone()),
            (Cow::Borrowed("option_option"), error_kind),
        ])))
    );
}
