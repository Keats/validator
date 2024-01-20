use validator::Validate;

#[test]
fn can_validate_starts_with_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(starts_with = "he")]
        val: String,
    }

    let s = TestStruct { val: "hello".to_string() };

    assert!(s.validate().is_ok());
}

#[test]
fn value_not_starting_with_needle_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(starts_with = "he")]
        val: String,
    }

    let s = TestStruct { val: String::new() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "starts_with");
    assert_eq!(errs["val"][0].params["value"], "");
    assert_eq!(errs["val"][0].params["needle"], "he");
}

#[test]
fn can_specify_code_for_starts_with() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(starts_with(pattern = "he", code = "oops"))]
        val: String,
    }
    let s = TestStruct { val: String::new() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "oops");
}

#[test]
fn can_specify_message_for_starts_with() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(starts_with(pattern = "he", message = "oops"))]
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
