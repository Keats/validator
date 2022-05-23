use validator::Validate;

#[test]
fn can_validate_does_not_contain_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(does_not_contain = "asdf")]
        val: String,
    }

    let s = TestStruct { val: "hello".to_string() };

    assert!(s.validate().is_ok());
}

#[test]
fn value_containing_needle_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(does_not_contain = "he")]
        val: String,
    }

    let s = TestStruct { val: "hello".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "does_not_contain");
    assert_eq!(errs["val"][0].params["value"], "hello");
    assert_eq!(errs["val"][0].params["needle"], "he");
}

#[test]
fn can_specify_code_for_does_not_contain() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(does_not_contain(pattern = "he", code = "oops"))]
        val: String,
    }
    let s = TestStruct { val: "hello".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "oops");
}

#[test]
fn can_specify_message_for_does_not_contain() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(does_not_contain(pattern = "he", message = "oops"))]
        val: String,
    }
    let s = TestStruct { val: "hello".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].clone().message.unwrap(), "oops");
}
