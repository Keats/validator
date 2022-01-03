use std::ops::AddAssign;

use validator::{Validate, ValidateArgs, ValidationError};

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
    #[validate(schema(function = "valid_reference_with_lifetime", arg = "&'v_a mut CustomStruct"))]
    struct TestStruct {
        value: String,
    }

    let s = TestStruct { value: "Hello World".to_string() };
    let mut cs = CustomStruct { counter: 0 };
    assert!(s.validate_args(&mut cs).is_ok());
    assert_eq!(cs.counter, 1);
}

#[test]
fn validate_schema_fn_tuple_err() {
    fn invalid_custom_tuple(_: &TestStruct, _arg: (i64, i64)) -> Result<(), ValidationError> {
        Err(ValidationError::new("meh"))
    }

    #[allow(dead_code)]
    #[derive(Debug, Validate)]
    #[validate(schema(function = "invalid_custom_tuple", arg = "(i64, i64)"))]
    struct TestStruct {
        value: String,
    }

    let s = TestStruct { value: "Hello World".to_string() };
    assert!(s.validate_args((77, 555)).is_err());
}

#[test]
fn invalidate_schema_fn_complex_arg_err() {
    fn invalid_validation_complex_args<'a, T: AddAssign>(
        _: &TestStruct,
        arg: (&'a mut CustomStruct, &'a mut T, T),
    ) -> Result<(), ValidationError> {
        arg.0.counter += 1;
        *arg.1 += arg.2;
        Err(ValidationError::new("meh"))
    }

    #[allow(dead_code)]
    #[derive(Debug, Validate)]
    #[validate(schema(
        function = "invalid_validation_complex_args",
        arg = "(&'v_a mut CustomStruct, &'v_a mut i32, i32)"
    ))]
    struct TestStruct {
        value: String,
    }

    let s = TestStruct { value: "Hello World".to_string() };
    let mut cs = CustomStruct { counter: 0 };
    let mut value = 10;
    assert!(s.validate_args((&mut cs, &mut value, 5)).is_err());
    assert_eq!(cs.counter, 1);
    assert_eq!(value, 15);
}

#[test]
fn validate_schema_multiple_fn_with_args_ok() {
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
    #[validate(schema(function = "valid_reference_with_lifetime", arg = "&'v_a mut CustomStruct"))]
    #[validate(schema(function = "invalid_custom_tuple", arg = "(i64, i64)"))]
    struct TestStruct {
        value: String,
        other_value: String,
    }

    let s = TestStruct {
        value: "Hello World".to_string(),
        other_value: "I'm different from value".to_string(),
    };

    let mut cs = CustomStruct { counter: 0 };
    assert!(s.validate_args((&mut cs, (123, 456))).is_err());
    assert_eq!(cs.counter, 1);
}
