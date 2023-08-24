use std::ops::AddAssign;

use validator::{Validate, ValidateArgs, ValidationError};

#[derive(Debug, PartialEq, Clone)]
struct TestContext {
    val: String,
}

fn valid_fn(_: &String, _arg: &TestContext) -> Result<(), ValidationError> {
    Ok(())
}

fn valid_fn_with_ref(_: &String, _arg: &TestContext) -> Result<(), ValidationError> {
    Ok(())
}

fn valid_fn_with_mut_ref(_: &String, arg: &mut TestContext) -> Result<(), ValidationError> {
    arg.val = "new value".to_string();
    Ok(())
}

#[test]
fn validate_simple_custom_fn() {
    #[derive(Validate)]
    #[validate(context = TestContext)]
    struct TestStruct {
        #[validate(custom(function = valid_fn, use_context))]
        value: String,
    }

    let test_struct = TestStruct { value: "Something".to_string() };
    let c = TestContext { val: "asd".to_string() };
    assert!(test_struct.validate_with_args(&c).is_ok());
}

#[test]
fn validate_multiple_custom_fn() {
    #[derive(Validate)]
    #[validate(context = TestContext)]
    struct TestStruct {
        #[validate(custom(function = valid_fn, use_context))]
        value: String,
        #[validate(custom(function = valid_fn, use_context))]
        value2: String,
        #[validate(custom(function = valid_fn, use_context))]
        value3: String,
    }

    let test_struct = TestStruct {
        value: "Something".to_string(),
        value2: "asd".to_string(),
        value3: "fgre".to_string(),
    };

    let test_arg = TestContext { val: "test".to_string() };

    assert!(test_struct.validate_with_args(&test_arg).is_ok());
}

#[test]
fn validate_custom_fn_with_ref() {
    #[derive(Validate)]
    #[validate(context = TestContext)]
    struct TestStruct {
        #[validate(custom(function = valid_fn_with_ref, use_context))]
        value: String,
    }

    let val = TestContext { val: "asd".to_string() };
    let test_struct = TestStruct { value: "Something".to_string() };
    assert!(test_struct.validate_with_args(&val).is_ok());

    // test reference
    assert_eq!(val, TestContext { val: "asd".to_string() });
}

#[test]
fn validate_custom_fn_with_mut_ref() {
    #[derive(Validate)]
    #[validate(context = TestContext, mutable)]
    struct TestStruct {
        #[validate(custom(function = valid_fn_with_mut_ref, use_context))]
        value: String,
    }

    let mut val = TestContext { val: "old value".to_string() };
    let test_struct = TestStruct { value: "Something".to_string() };
    assert!(test_struct.validate_with_args(&mut val).is_ok());

    assert_eq!(val, TestContext { val: "new value".to_string() });
}

#[test]
fn validate_custom_fn_with_complex_args() {
    #[derive(Validate)]
    #[validate(context = "Arg<i32>", mutable)]
    struct TestStruct {
        #[validate(custom(function = add_assign, use_context))]
        value: String,
    }

    struct Arg<T: AddAssign> {
        counter: T,
    }

    fn add_assign(_value: &str, arg: &mut Arg<i32>) -> Result<(), ValidationError> {
        arg.counter += 1;
        Ok(())
    }

    let mut arg = Arg { counter: 0 };
    let test_struct = TestStruct { value: "test".to_string() };

    assert!(test_struct.validate_with_args(&mut arg).is_ok());

    assert_eq!(arg.counter, 1)
}

#[test]
fn validate_custom_fn_with_multiple_args() {
    #[derive(Debug, Validate, PartialEq)]
    #[validate(context = Arg, mutable)]
    struct TestStruct {
        #[validate(custom(function = add_assign, use_context))]
        value: String,
    }

    #[derive(Debug, PartialEq)]
    struct Arg {
        counter: i32,
        counter2: u8,
    }

    fn add_assign(_: &String, arg: &mut Arg) -> Result<(), ValidationError> {
        arg.counter += 1;
        arg.counter2 += 2;
        Ok(())
    }

    let test_struct = TestStruct { value: "something".to_string() };
    let mut arg = Arg { counter: 5, counter2: 16 };
    assert!(test_struct.validate_with_args(&mut arg).is_ok());

    assert_eq!(arg, Arg { counter: 6, counter2: 18 });
}

#[test]
fn validate_nested_custom_fn() {
    #[derive(Validate)]
    #[validate(context = Arg)]
    struct TestStruct {
        #[validate(nested)]
        child: Child,
    }

    #[derive(Validate)]
    #[validate(context = Arg)]
    struct Child {
        #[validate(custom(function = add_assign, use_context))]
        value: String,
    }

    struct Arg {
        _counter: i32,
    }

    fn add_assign(_: &String, _arg: &Arg) -> Result<(), ValidationError> {
        Ok(())
    }

    let t = TestStruct { child: Child { value: "test".to_string() } };
    let arg = Arg { _counter: 123 };

    assert!(t.validate_with_args(&arg).is_ok());
}
