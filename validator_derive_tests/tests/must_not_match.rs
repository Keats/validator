use validator::Validate;

#[test]
fn can_validate_valid_must_not_match() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(must_not_match(other = "val2"))]
        val: String,
        val2: String,
    }

    let s = TestStruct { val: "bob".to_string(), val2: "alice".to_string() };

    assert!(s.validate().is_ok());
}

#[test]
fn matching_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(must_not_match(other = "val2"))]
        val: String,
        val2: String,
    }

    let s = TestStruct { val: "bob".to_string(), val2: "bob".to_string() };

    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "must_not_match");
    assert_eq!(errs["val"][0].params["value"], "bob");
    assert_eq!(errs["val"][0].params["other"], "bob");
}

#[test]
fn can_specify_code_for_must_not_match() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(must_not_match(other = "val2", code = "oops"))]
        val: String,
        val2: String,
    }
    let s = TestStruct { val: "bob".to_string(), val2: "bob".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "oops");
}

#[test]
fn can_specify_message_for_must_not_match() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(must_not_match(other = "val2", message = "oops"))]
        val: String,
        val2: String,
    }
    let s = TestStruct { val: "bob".to_string(), val2: "bob".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].clone().message.unwrap(), "oops");
}
