use validator::Validate;

#[test]
fn can_validate_length_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = 5, max = 10))]
        val: String,
    }

    let s = TestStruct { val: "hello".to_string() };

    assert!(s.validate().is_ok());
}

#[test]
fn value_out_of_length_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = 5, max = 10))]
        val: String,
    }

    let s = TestStruct { val: String::new() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "length");
    assert_eq!(errs["val"][0].params["value"], "");
    assert_eq!(errs["val"][0].params["min"], 5);
    assert_eq!(errs["val"][0].params["max"], 10);
}

#[test]
fn can_specify_code_for_length() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = 5, max = 10, code = "oops"))]
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
fn can_specify_message_for_length() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = 5, max = 10, message = "oops"))]
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

#[test]
fn can_validate_ref_for_length() {
    use serde_json::Value;

    #[derive(Debug, Validate)]
    struct TestStruct<'a> {
        #[validate(length(min = 5, max = 10))]
        val: &'a Vec<String>,
    }

    let strings = vec![String::new()];
    let s = TestStruct { val: &strings };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "length");
    assert_eq!(errs["val"][0].params["value"], Value::Array(vec![Value::String(String::new())]));
    assert_eq!(errs["val"][0].params["min"], 5);
    assert_eq!(errs["val"][0].params["max"], 10);
}
