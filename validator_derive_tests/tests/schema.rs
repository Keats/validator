use validator::{Validate, ValidateArgs, ValidationError};

#[test]
fn can_validate_schema_fn_ok() {
    fn valid_schema_fn(_: &TestStruct) -> Result<(), ValidationError> {
        Ok(())
    }

    #[allow(dead_code)]
    #[derive(Debug, Validate)]
    #[validate(schema)]
    pub struct TestStruct {
        val: String,
    }

    let s = TestStruct { val: "hello".into() };

    assert!(s.validate(|t| valid_schema_fn(t)).is_ok());
}

mod some_defining_mod {
    use validator::Validate;

    #[derive(Debug, Validate)]
    #[validate(schema)]
    pub struct TestStructValid {
        pub val: String,
    }

    #[derive(Debug, Validate)]
    #[validate(schema)]
    pub struct TestStructInvalid {
        pub val: String,
    }
}

mod some_validation_mod {
    use validator::ValidationError;

    use crate::some_defining_mod::{TestStructInvalid, TestStructValid};

    pub fn valid_schema_fn(_: &TestStructValid) -> Result<(), ValidationError> {
        Ok(())
    }

    pub fn invalid_schema_fn(_: &TestStructInvalid) -> Result<(), ValidationError> {
        Err(ValidationError::new("meh"))
    }
}

#[test]
fn can_validate_fully_qualified_fn_ok() {
    let s = some_defining_mod::TestStructValid { val: "hello".into() };

    assert!(s.validate(some_validation_mod::valid_schema_fn).is_ok());
}

#[test]
fn can_fail_fully_qualified_fn_validation() {
    let s = some_defining_mod::TestStructInvalid { val: "hello".into() };

    let res = s.validate(some_validation_mod::invalid_schema_fn);
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("__all__"));
    assert_eq!(errs["__all__"].len(), 1);
    assert_eq!(errs["__all__"][0].code, "meh");
}

// Do we really need multiple schema validations?
// You can simply call a function within the schema validation
//
// #[test]
// fn can_validate_multiple_schema_fn_ok() {
//     fn valid_schema_fn(_: &TestStruct) -> Result<(), ValidationError> {
//         Ok(())
//     }

//     fn valid_schema_fn2(_: &TestStruct) -> Result<(), ValidationError> {
//         Ok(())
//     }

//     #[allow(dead_code)]
//     #[derive(Debug, Validate)]
//     #[validate(schema)]
//     #[validate(schema)]
//     struct TestStruct {
//         val: String,
//     }

//     let s = TestStruct { val: "hello".into() };

//     assert!(s.validate().is_ok());
// }

#[test]
fn can_fail_schema_fn_validation() {
    fn invalid_schema_fn(_: &TestStruct) -> Result<(), ValidationError> {
        Err(ValidationError::new("meh"))
    }

    #[allow(dead_code)]
    #[derive(Debug, Validate)]
    #[validate(schema)]
    struct TestStruct {
        val: String,
    }

    let s = TestStruct { val: String::new() };
    let res = s.validate(invalid_schema_fn);
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("__all__"));
    assert_eq!(errs["__all__"].len(), 1);
    assert_eq!(errs["__all__"][0].code, "meh");
}

// #[test]
// fn can_fail_multiple_schema_fn_validation() {
//     fn invalid_schema_fn(_: &TestStruct) -> Result<(), ValidationError> {
//         Err(ValidationError::new("meh"))
//     }

//     fn invalid_schema_fn2(_: &TestStruct) -> Result<(), ValidationError> {
//         Err(ValidationError::new("meh2"))
//     }

//     #[allow(dead_code)]
//     #[derive(Debug, Validate)]
//     #[validate(schema(function = "invalid_schema_fn"))]
//     #[validate(schema(function = "invalid_schema_fn2"))]
//     struct TestStruct {
//         val: String,
//     }

//     let s = TestStruct { val: String::new() };
//     let res = s.validate();
//     assert!(res.is_err());
//     let err = res.unwrap_err();
//     let errs = err.field_errors();
//     assert!(errs.contains_key("__all__"));
//     assert_eq!(errs["__all__"].len(), 2);
//     assert_eq!(errs["__all__"][0].code, "meh");
//     assert_eq!(errs["__all__"][1].code, "meh2");
// }

#[test]
fn can_specify_message_for_schema_fn() {
    fn invalid_schema_fn(_: &TestStruct) -> Result<(), ValidationError> {
        Err(ValidationError::new("meh"))
    }

    #[allow(dead_code)]
    #[derive(Debug, Validate)]
    #[validate(schema(message = "oops"))]
    struct TestStruct {
        val: String,
    }
    let s = TestStruct { val: String::new() };
    let res = s.validate(invalid_schema_fn);
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("__all__"));
    assert_eq!(errs["__all__"].len(), 1);
    assert_eq!(errs["__all__"][0].clone().message.unwrap(), "oops");
}

#[test]
fn can_choose_to_run_schema_validation_even_after_field_errors() {
    fn invalid_schema_fn(_: &TestStruct) -> Result<(), ValidationError> {
        Err(ValidationError::new("meh"))
    }
    #[allow(dead_code)]
    #[derive(Debug, Validate)]
    #[validate(schema)]
    struct TestStruct {
        val: String,
        #[validate(range(min = 1, max = 10))]
        num: usize,
    }

    let s = TestStruct { val: "hello".to_string(), num: 0 };

    let res = s.validate(invalid_schema_fn);
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("__all__"));
    assert_eq!(errs["__all__"].len(), 1);
    assert_eq!(errs["__all__"][0].clone().code, "meh");
    assert!(errs.contains_key("num"));
    assert_eq!(errs["num"].len(), 1);
    assert_eq!(errs["num"][0].clone().code, "range");
}
