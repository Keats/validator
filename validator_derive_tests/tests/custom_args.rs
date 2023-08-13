use std::ops::AddAssign;
use std::path::Path;

use validator::{Validate, ValidateArgs, ValidationError};

#[derive(Debug, PartialEq)]
struct TestArg {
    val: String,
}

fn valid_fn(_: &String, _arg: TestArg) -> Result<(), ValidationError> {
    Ok(())
}

fn valid_fn_with_ref(_: &String, _arg: &TestArg) -> Result<(), ValidationError> {
    Ok(())
}

fn valid_fn_with_mut_ref(_: &String, arg: &mut TestArg) -> Result<(), ValidationError> {
    arg.val = "new value".to_string();
    Ok(())
}

#[test]
fn validate_simple_custom_fn() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(custom)]
        value: String,
    }

    let test_struct = TestStruct { value: "Something".to_string() };
    assert!(test_struct.validate(|v| valid_fn(v, TestArg { val: "asd".to_string() })).is_ok());
}

#[test]
fn validate_multiple_custom_fn() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(custom)]
        value: String,
        #[validate(custom)]
        value2: String,
        #[validate(custom)]
        value3: String,
    }

    let test_struct = TestStruct {
        value: "Something".to_string(),
        value2: "asd".to_string(),
        value3: "fgre".to_string(),
    };

    let test_arg1 = TestArg { val: "test".to_string() };
    let test_arg2 = TestArg { val: "test".to_string() };
    let test_arg3 = TestArg { val: "test".to_string() };

    assert!(test_struct
        .validate((
            |s| valid_fn(s, test_arg1),
            |s| valid_fn(s, test_arg2),
            |s| valid_fn(s, test_arg3)
        ))
        .is_ok());
}

#[test]
fn validate_custom_fn_with_ref() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(custom)]
        value: String,
    }

    let val = TestArg { val: "asd".to_string() };
    let test_struct = TestStruct { value: "Something".to_string() };
    assert!(test_struct.validate(|s| valid_fn_with_ref(s, &val)).is_ok());

    // test reference
    assert_eq!(val, TestArg { val: "asd".to_string() });
}

#[test]
fn validate_custom_fn_with_mut_ref() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(custom)]
        value: String,
    }

    let mut val = TestArg { val: "old value".to_string() };
    let test_struct = TestStruct { value: "Something".to_string() };
    assert!(test_struct.validate(|s| valid_fn_with_mut_ref(s, &mut val)).is_ok());

    assert_eq!(val, TestArg { val: "new value".to_string() });
}

#[test]
fn validate_custom_fn_with_complex_args() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(custom)]
        value: String,
    }

    struct Arg<T: AddAssign> {
        counter: T,
    }

    let test_struct = TestStruct { value: "test".to_string() };
    let closure = |_val: &String, mut arg: Arg<u32>| -> Result<(), ValidationError> {
        arg.counter += 1;
        Ok(())
    };

    assert!(test_struct.validate(|s| closure(s, Arg { counter: 0 })).is_ok())
}

#[test]
fn validate_custom_fn_with_multiple_args() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(custom)]
        value: String,
    }

    struct Arg {
        counter: i32,
    }

    let closure =
        |_: &String, mut arg: Arg, _foo: &Path, _str: &str| -> Result<(), ValidationError> {
            arg.counter += 1;
            Ok(())
        };

    let test_struct = TestStruct { value: "something".to_string() };
    assert!(test_struct
        .validate(|s| closure(s, Arg { counter: 5 }, Path::new("file.txt"), "str"))
        .is_ok())
}
