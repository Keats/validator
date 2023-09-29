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
    #[validate(context = CustomStruct, mutable)]
    #[validate(schema(function = valid_reference_with_lifetime, use_context))]
    struct TestStruct {
        value: String,
    }

    let s = TestStruct { value: "Hello World".to_string() };
    let mut cs = CustomStruct { counter: 0 };
    assert!(s.validate_with_args(&mut cs).is_ok());
    assert_eq!(cs.counter, 1);
}

#[test]
fn validate_schema_fn_err() {
    fn invalid_custom_args(_: &TestStruct, _context: &Args) -> Result<(), ValidationError> {
        Err(ValidationError::new("meh"))
    }

    #[allow(dead_code)]
    #[derive(Debug, Validate)]
    #[validate(context = Args)]
    #[validate(schema(function = invalid_custom_args, use_context))]
    struct TestStruct {
        value: String,
    }

    #[allow(dead_code)]
    struct Args {
        arg1: i64,
        arg2: i64,
    }

    let s = TestStruct { value: "Hello World".to_string() };
    let args = Args { arg1: 1, arg2: 2 };
    assert!(s.validate_with_args(&args).is_err());
}

#[test]
fn invalidate_schema_fn_complex_arg_err() {
    fn invalid_validation_complex_args(
        _: &TestStruct,
        args: &mut TestArgs<'_, i32>,
    ) -> Result<(), ValidationError> {
        args.arg1 += 1;
        *args.arg3 += args.arg2;
        Err(ValidationError::new("meh"))
    }

    #[allow(dead_code)]
    #[derive(Debug, Validate)]
    #[validate(context = "TestArgs<'v_a, i32>", mutable)]
    #[validate(schema(function = invalid_validation_complex_args, use_context))]
    struct TestStruct {
        value: String,
    }

    struct TestArgs<'a, T: AddAssign> {
        arg1: T,
        arg2: &'a T,
        arg3: &'a mut T,
    }

    let s = TestStruct { value: "Hello World".to_string() };
    let mut args = TestArgs { arg1: 1, arg2: &2, arg3: &mut 3 };
    assert!(s.validate_with_args(&mut args).is_err());
}

#[test]
fn validate_schema_multiple_fn_with_args_ok() {
    fn multiple_fn(t: &TestStruct, arg: &mut CustomStruct) -> Result<(), ValidationError> {
        valid_reference_with_lifetime(t, arg)?;
        invalid_custom_tuple(t, arg)
    }

    fn valid_reference_with_lifetime(
        _: &TestStruct,
        arg: &mut CustomStruct,
    ) -> Result<(), ValidationError> {
        arg.counter += 1;
        Ok(())
    }

    fn invalid_custom_tuple(
        _: &TestStruct,
        _arg: &mut CustomStruct,
    ) -> Result<(), ValidationError> {
        Err(ValidationError::new("meh"))
    }

    #[allow(dead_code)]
    #[derive(Debug, Validate)]
    #[validate(context = CustomStruct, mutable)]
    #[validate(schema(function = multiple_fn, use_context))]
    struct TestStruct {
        value: String,
        other_value: String,
    }

    let s = TestStruct {
        value: "Hello World".to_string(),
        other_value: "I'm different from value".to_string(),
    };

    let mut cs = CustomStruct { counter: 0 };
    assert!(s.validate_with_args(&mut cs).is_err());
    assert_eq!(cs.counter, 1);
}
