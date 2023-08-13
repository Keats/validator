use validator::{Validate, ValidateArgs, ValidationError};

fn valid_custom_fn(arr: &[u8; 2]) -> Result<(), ValidationError> {
    match arr[0] == 1 {
        true => Ok(()),
        false => Err(ValidationError::new("meh")),
    }
}

#[test]
fn can_validate_valid_email_with_unsupported_array() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(email)]
        val: String,
        #[allow(dead_code)]
        array: [u8; 2],
    }

    let s = TestStruct { val: "bob@bob.com".to_string(), array: [0u8; 2] };

    assert!(s.validate().is_ok());
}

#[test]
fn can_validate_custom_with_unsupported_array() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(email)]
        val: String,
        #[validate(custom)]
        array: [u8; 2],
    }

    let s = TestStruct { val: "bob@bob.com".to_string(), array: [1u8, 1u8] };

    assert!(s.validate(valid_custom_fn).is_ok());
}

#[test]
fn can_fail_custom_with_unsupported_array() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(email)]
        val: String,
        #[validate(custom)]
        array: [u8; 2],
    }

    let s = TestStruct { val: "bob@bob.com".to_string(), array: [0u8, 1u8] };

    let res = s.validate(valid_custom_fn);
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("array"));
    assert_eq!(errs["array"].len(), 1);
    assert_eq!(errs["array"][0].code, "meh");
    assert_eq!(errs["array"][0].params["value"][0], 0);
}
