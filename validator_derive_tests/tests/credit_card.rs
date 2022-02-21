use validator::Validate;

#[test]
fn can_validate_valid_card_number() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(credit_card)]
        val: String,
    }

    let s = TestStruct { val: "5236313877109142".to_string() };

    assert!(s.validate().is_ok());
}

#[test]
fn bad_credit_card_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(credit_card)]
        val: String,
    }

    let s = TestStruct { val: "bob".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "credit_card");
    assert_eq!(errs["val"][0].params["value"], "bob");
}

#[test]
fn can_specify_code_for_credit_card() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(credit_card(code = "oops"))]
        val: String,
    }
    let s = TestStruct { val: "bob".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "oops");
}

#[test]
fn can_specify_message_for_credit_card() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(credit_card(message = "oops"))]
        val: String,
    }
    let s = TestStruct { val: "bob".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].clone().message.unwrap(), "oops");
}

#[test]
fn can_specify_sensitive_for_credit_card() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(credit_card(sensitive = true))]
        val: String,
    }
    let s = TestStruct { val: "bob".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert!(!errs["val"][0].params.contains_key("value"));
}
