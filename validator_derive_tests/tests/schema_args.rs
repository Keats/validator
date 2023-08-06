use std::ops::AddAssign;

use validator::{Validate, ValidationError};

struct CustomStruct {
    pub counter: i32,
}

#[test]
fn validate_schema_fn_reference_with_lifetime_ok() {
    fn valid_reference_with_lifetime(
        _: &TestStruct,
        arg: &mut CustomStruct,
    ) -> Result<(), ValidationError> {
        arg.counter += 1;
        Ok(())
    }

    #[allow(dead_code)]
    #[derive(Debug, Validate)]
    #[validate(schema)]
    struct TestStruct {
        value: String,
    }

    let s = TestStruct { value: "Hello World".to_string() };
    let mut cs = CustomStruct { counter: 0 };
    assert!(s.validate(|t| valid_reference_with_lifetime(t, &mut cs)).is_ok());
    assert_eq!(cs.counter, 1);
}

#[test]
fn validate_schema_fn_err() {
    fn invalid_custom_tuple(_: &TestStruct, arg1: i64, arg2: i64) -> Result<(), ValidationError> {
        Err(ValidationError::new("meh"))
    }

    #[allow(dead_code)]
    #[derive(Debug, Validate)]
    #[validate(schema)]
    struct TestStruct {
        value: String,
    }

    let s = TestStruct { value: "Hello World".to_string() };
    assert!(s.validate(|t| invalid_custom_tuple(t, 69, 420)).is_err());
}

#[test]
fn invalidate_schema_fn_complex_arg_err() {
    fn invalid_validation_complex_args<'a, T: AddAssign>(
        _: &TestStruct,
        arg1: &'a mut CustomStruct,
        arg2: &'a mut T,
        arg3: T,
    ) -> Result<(), ValidationError> {
        arg1.counter += 1;
        *arg2 += arg3;
        Err(ValidationError::new("meh"))
    }

    #[allow(dead_code)]
    #[derive(Debug, Validate)]
    #[validate(schema)]
    struct TestStruct {
        value: String,
    }

    let s = TestStruct { value: "Hello World".to_string() };
    let mut cs = CustomStruct { counter: 0 };
    let mut value = 10;
    assert!(s.validate(|t| invalid_validation_complex_args(t, &mut cs, &mut value, 5)).is_err());
    assert_eq!(cs.counter, 1);
    assert_eq!(value, 15);
}

#[test]
fn validate_schema_multiple_fn_with_args_ok() {
    fn multiple_fn(
        t: &TestStruct,
        arg1: &mut CustomStruct,
        arg2: i64,
        arg3: i64,
    ) -> Result<(), ValidationError> {
        valid_reference_with_lifetime(t, arg1)?;
        invalid_custom_tuple(t, (arg2, arg3))
    }

    fn valid_reference_with_lifetime(
        _: &TestStruct,
        arg: &mut CustomStruct,
    ) -> Result<(), ValidationError> {
        arg.counter += 1;
        Ok(())
    }

    fn invalid_custom_tuple(_: &TestStruct, _arg: (i64, i64)) -> Result<(), ValidationError> {
        Err(ValidationError::new("meh"))
    }

    #[allow(dead_code)]
    #[derive(Debug, Validate)]
    #[validate(schema)]
    struct TestStruct {
        value: String,
        other_value: String,
    }

    let s = TestStruct {
        value: "Hello World".to_string(),
        other_value: "I'm different from value".to_string(),
    };

    let mut cs = CustomStruct { counter: 0 };
    assert!(s.validate(|t| multiple_fn(t, &mut cs, 123, 456)).is_err());
    assert_eq!(cs.counter, 1);
}
