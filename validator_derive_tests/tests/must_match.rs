use validator::Validate;

#[test]
fn can_validate_valid_must_match() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(must_match(other = "val2"))]
        val: String,
        val2: String,
    }

    let s = TestStruct { val: "bob".to_string(), val2: "bob".to_string() };

    assert!(s.validate().is_ok());
}

#[test]
fn not_matching_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(must_match(other = "val2"))]
        val: String,
        val2: String,
    }

    let s = TestStruct { val: "bob".to_string(), val2: "bobby".to_string() };

    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "must_match");
    assert_eq!(errs["val"][0].params["value"], "bob");
    assert_eq!(errs["val"][0].params["other"], "bobby");
}

#[test]
fn can_specify_code_for_must_match() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(must_match(other = "val2", code = "oops"))]
        val: String,
        val2: String,
    }
    let s = TestStruct { val: "bob".to_string(), val2: "bobb".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "oops");
}

#[test]
fn can_specify_message_for_must_match() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(must_match(other = "val2", message = "oops"))]
        val: String,
        val2: String,
    }
    let s = TestStruct { val: "bob".to_string(), val2: "bobb".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].clone().message.unwrap(), "oops");
}

#[test]
fn can_validate_option_must_match_with_required() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(required)]
        a: Option<String>,
        #[validate(required, must_match(other = "a"))]
        b: Option<String>,
    }

    let ok = TestStruct { a: Some("bob".to_string()), b: Some("bob".to_string()) };
    assert!(ok.validate().is_ok());

    let mismatch = TestStruct { a: Some("bobby".to_string()), b: Some("bob".to_string()) };
    let err = mismatch.validate().unwrap_err();
    let errs = err.field_errors();
    assert_eq!(errs["b"].len(), 1);
    assert_eq!(errs["b"][0].code, "must_match");
    assert_eq!(errs["b"][0].params["value"], "bob");
    assert_eq!(errs["b"][0].params["other"], "bobby");

    let other_none = TestStruct { a: None, b: Some("bob".to_string()) };
    let err = other_none.validate().unwrap_err();
    let errs = err.field_errors();
    assert_eq!(errs["a"][0].code, "required");
    assert_eq!(errs["b"][0].code, "must_match");
    assert_eq!(errs["b"][0].params["value"], "bob");
    assert!(errs["b"][0].params["other"].is_null());

    let self_none = TestStruct { a: Some("bob".to_string()), b: None };
    let err = self_none.validate().unwrap_err();
    let errs = err.field_errors();
    assert!(!errs.contains_key("a"));
    assert_eq!(errs["b"].len(), 1);
    assert_eq!(errs["b"][0].code, "required");
}

#[test]
fn can_validate_option_must_match_one_required() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(required, must_match(other = "b"))]
        a: Option<String>,
        b: Option<String>,
    }

    assert!(
        TestStruct { a: Some("bob".to_string()), b: Some("bob".to_string()) }.validate().is_ok()
    );
    assert!(TestStruct { a: Some("bob".to_string()), b: None }.validate().is_err());
    assert!(TestStruct { a: None, b: None }.validate().is_err());
}

#[test]
fn can_validate_option_must_match_without_required() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(must_match(other = "b"))]
        a: Option<String>,
        b: Option<String>,
    }

    assert!(TestStruct { a: None, b: None }.validate().is_ok());
    assert!(TestStruct { a: None, b: Some("bob".to_string()) }.validate().is_ok());
    assert!(
        TestStruct { a: Some("bob".to_string()), b: Some("bob".to_string()) }.validate().is_ok()
    );

    let err = TestStruct { a: Some("bob".to_string()), b: None }.validate().unwrap_err();
    assert_eq!(err.field_errors()["a"][0].code, "must_match");

    let err = TestStruct { a: Some("bob".to_string()), b: Some("bobby".to_string()) }
        .validate()
        .unwrap_err();
    assert_eq!(err.field_errors()["a"][0].code, "must_match");
}

#[test]
fn can_validate_mixed_option_and_non_option_must_match() {
    #[derive(Debug, Validate)]
    struct OptToPlain {
        #[validate(must_match(other = "b"))]
        a: Option<String>,
        b: String,
    }

    assert!(OptToPlain { a: None, b: "bob".to_string() }.validate().is_ok());
    assert!(OptToPlain { a: Some("bob".to_string()), b: "bob".to_string() }.validate().is_ok());
    assert!(OptToPlain { a: Some("bob".to_string()), b: "bobby".to_string() }.validate().is_err());

    #[derive(Debug, Validate)]
    struct PlainToOpt {
        #[validate(must_match(other = "b"))]
        a: String,
        #[validate(skip)]
        b: Option<String>,
    }

    assert!(PlainToOpt { a: "bob".to_string(), b: Some("bob".to_string()) }.validate().is_ok());
    assert!(PlainToOpt { a: "bob".to_string(), b: None }.validate().is_err());
    assert!(PlainToOpt { a: "bob".to_string(), b: Some("bobby".to_string()) }.validate().is_err());
}

#[test]
fn can_validate_number_and_nested_option_must_match() {
    #[derive(Debug, Validate)]
    struct NumberOpt<'a> {
        #[validate(required, must_match(other = "n2"))]
        n1: Option<i32>,
        n2: Option<i32>,
        #[validate(must_match(other = "n4"))]
        n3: Option<i32>,
        n4: i32,
        #[validate(must_match(other = "s2"))]
        s1: Option<Option<&'a str>>,
        s2: Option<Option<&'a str>>,
    }

    let ok = NumberOpt {
        n1: Some(42),
        n2: Some(42),
        n3: Some(7),
        n4: 7,
        s1: Some(Some("hi")),
        s2: Some(Some("hi")),
    };
    assert!(ok.validate().is_ok());

    let bad_num = NumberOpt {
        n1: Some(42),
        n2: Some(99),
        n3: None,
        n4: 7,
        s1: Some(Some("hi")),
        s2: Some(None),
    };
    let err = bad_num.validate().unwrap_err();
    let errs = err.field_errors();
    assert_eq!(errs["n1"][0].code, "must_match");
    assert_eq!(errs["s1"][0].code, "must_match");
}
