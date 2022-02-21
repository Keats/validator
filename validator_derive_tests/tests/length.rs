use validator::Validate;

const MIN_CONST: u64 = 1;
const MAX_CONST: u64 = 10;

const MAX_CONST_I32: i32 = 2;
const NEGATIVE_CONST_I32: i32 = -10;

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
fn validate_length_with_ref_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = "MIN_CONST", max = "MAX_CONST"))]
        val: String,
    }

    let s = TestStruct { val: "hello".to_string() };

    assert!(s.validate().is_ok());
}

#[test]
fn validate_length_with_ref_fails() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = "MIN_CONST", max = "MAX_CONST"))]
        val: String,
    }

    let s = TestStruct { val: "".to_string() };

    assert_eq!(s.validate().is_ok(), false);
}

#[test]
fn validate_length_with_ref_i32_fails() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(max = "MAX_CONST_I32"))]
        val: String,
    }

    let s = TestStruct { val: "TO_LONG_YAY".to_string() };

    assert_eq!(s.validate().is_ok(), false);
}

#[test]
fn validate_length_with_ref_negative_i32_fails() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(max = "NEGATIVE_CONST_I32"))]
        val: String,
    }

    let s = TestStruct { val: "TO_LONG_YAY".to_string() };

    assert_eq!(s.validate().is_ok(), true);
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
fn can_specify_sensitive_for_length() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = 5, max = 10, sensitive = true))]
        val: String,
    }
    let s = TestStruct { val: String::new() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert!(!errs["val"][0].params.contains_key("value"));
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

#[cfg(feature = "indexmap")]
#[test]
fn can_validate_set_ref_for_length() {
    use indexmap::{indexset, IndexSet};
    use serde_json::Value;

    #[derive(Debug, Validate)]
    struct TestStruct<'a> {
        #[validate(length(min = 5, max = 10))]
        val: &'a IndexSet<String>,
    }

    let strings = indexset! {String::new()};
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
