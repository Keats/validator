#[macro_use]
extern crate validator_derive;
extern crate validator;

use validator::{Validate, ValidationError};


#[test]
fn can_validate_schema_fn_ok() {
    fn valid_schema_fn(_: &TestStruct) -> Result<(), ValidationError> {
    Ok(())
}

    #[derive(Debug, Validate)]
    #[validate(schema(function = "valid_schema_fn"))]
    struct TestStruct {
        val: String,
    }

    let s = TestStruct {
        val: "hello".to_string(),
    };

    assert!(s.validate().is_ok());
}

#[test]
fn can_fail_schema_fn_validation() {
    fn invalid_schema_fn(_: &TestStruct) -> Result<(), ValidationError> {
        Err(ValidationError::new("meh"))
    }

    #[derive(Debug, Validate)]
    #[validate(schema(function = "invalid_schema_fn"))]
    struct TestStruct {
        val: String,
    }

    let s = TestStruct {
        val: String::new(),
    };
    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("__all__"));
    assert_eq!(errs["__all__"].len(), 1);
    assert_eq!(errs["__all__"][0].code, "meh");
}

#[test]
fn can_specify_message_for_schema_fn() {
    fn invalid_schema_fn(_: &TestStruct) -> Result<(), ValidationError> {
        Err(ValidationError::new("meh"))
    }

    #[derive(Debug, Validate)]
    #[validate(schema(function = "invalid_schema_fn", message = "oops"))]
    struct TestStruct {
        val: String,
    }
    let s = TestStruct {
        val: String::new(),
    };
    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("__all__"));
    assert_eq!(errs["__all__"].len(), 1);
    assert_eq!(errs["__all__"][0].clone().message.unwrap(), "oops");
}

#[test]
fn can_choose_to_run_schema_validation_even_after_field_errors() {
    fn invalid_schema_fn(_: &TestStruct) -> Result<(), ValidationError> {
        Err(ValidationError::new("meh"))
    }
    #[derive(Debug, Validate)]
    #[validate(schema(function = "invalid_schema_fn", skip_on_field_errors = "false"))]
    struct TestStruct {
        val: String,
        #[validate(range(min = "1", max = "10"))]
        num: usize,
    }

    let s = TestStruct {
        val: "hello".to_string(),
        num: 0,
    };

    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("__all__"));
    assert_eq!(errs["__all__"].len(), 1);
    assert_eq!(errs["__all__"][0].clone().code, "meh");
    assert!(errs.contains_key("num"));
    assert_eq!(errs["num"].len(), 1);
    assert_eq!(errs["num"][0].clone().code, "range");
}
