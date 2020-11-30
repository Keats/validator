use std::ops::AddAssign;

use validator::{Validate, ValidateArgs, ValidationError};

struct CustomStruct {
    pub counter: i32,
}

fn valid_generic_custom_i32<T>(_: &T, _arg: i32) -> Result<(), ValidationError> {
    Ok(())
}

fn invalid_custom_tuple(_: &str, _arg: (i64, i64)) -> Result<(), ValidationError> {
    Err(ValidationError::new("meh"))
}

fn valid_reference_with_lifetime<'a>(
    _: &str,
    arg: &'a mut CustomStruct,
) -> Result<(), ValidationError> {
    arg.counter += 1;
    Ok(())
}

fn invalid_validation_complex_args<'a, T: AddAssign>(
    _: &str,
    arg: (&'a mut CustomStruct, &'a mut T, T),
) -> Result<(), ValidationError> {
    arg.0.counter += 1;
    *arg.1 += arg.2;
    Err(ValidationError::new("meh"))
}

#[test]
fn validate_custom_fn_tuple() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(custom(
            function = "valid_reference_with_lifetime",
            arg = "&'v_a mut CustomStruct"
        ))]
        value: String,
    }

    let s = TestStruct { value: "Hello World".to_string() };

    let mut cs = CustomStruct { counter: 0 };
    assert!(s.validate_args(&mut cs).is_ok());
    assert!(cs.counter == 1);
}

#[test]
fn validate_custom_fn_tuple_err() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(custom(function = "invalid_custom_tuple", arg = "(i64, i64)"))]
        value: String,
    }

    let s = TestStruct { value: "Hello World".to_string() };

    assert!(s.validate_args((77, 555)).is_err());
}

#[test]
fn validate_custom_struct_generic_and_lifetime_fn_i32_ok() {
    #[derive(Debug, Validate)]
    struct TestGenericStruct<'a, T: serde::ser::Serialize> {
        #[validate(custom(function = "valid_generic_custom_i32", arg = "i32"))]
        generic: &'a T,
    }

    let int_128 = 746460_i128;
    let s = TestGenericStruct { generic: &int_128 };

    assert!(s.validate_args(16).is_ok());
}

#[test]
fn invalidate_custom_fn_complex_arg_err() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(custom(
            function = "invalid_validation_complex_args",
            arg = "(&'v_a mut CustomStruct, &'v_a mut i32, i32)"
        ))]
        value: String,
    }

    let s = TestStruct { value: "Hello World".to_string() };

    let mut cs = CustomStruct { counter: 0 };
    let mut value = 10;
    assert!(s.validate_args((&mut cs, &mut value, 5)).is_err());
    assert!(cs.counter == 1);
    assert!(value == 15);
}

#[test]
fn validate_custom_multiple_fn_with_args_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(custom(
            function = "valid_reference_with_lifetime",
            arg = "&'v_a mut CustomStruct"
        ))]
        value: String,

        #[validate(custom(function = "invalid_custom_tuple", arg = "(i64, i64)"))]
        other_value: String,
    }

    let s = TestStruct {
        value: "Hello World".to_string(),
        other_value: "I'm different from value".to_string(),
    };

    let mut cs = CustomStruct { counter: 0 };
    assert!(s.validate_args((&mut cs, (123, 456))).is_err());
    assert!(cs.counter == 1);
}