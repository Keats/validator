use validator::Validate;

const MIN_CONST: u64 = 1;
const MAX_CONST: u64 = 10;

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

    assert!(s.validate().is_err());
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

#[test]
fn can_validate_slice_for_length() {
    use serde_json::Value;

    #[derive(Debug, Validate)]
    struct TestStruct<'a> {
        #[validate(length(min = 5, max = 10))]
        val: &'a [String],
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

#[test]
fn can_validate_array_for_length() {
    use serde_json::Value;

    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = 5, max = 10))]
        val: [String; 1],
    }

    let s = TestStruct { val: [String::new()] };
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

#[test]
fn can_validate_custom_impl_for_length() {
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    struct CustomString(String);

    impl validator::ValidateLength<u64> for CustomString {
        fn length(&self) -> Option<u64> {
            Some(self.0.chars().count() as u64)
        }
    }

    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = 5, max = 10))]
        val: CustomString,
    }

    #[derive(Debug, Validate)]
    struct EqualsTestStruct {
        #[validate(length(equal = 11))]
        val: CustomString,
    }

    let too_short = TestStruct { val: CustomString(String::from("oops")) };

    let too_long = TestStruct { val: CustomString(String::from("too long for this")) };

    let ok = TestStruct { val: CustomString(String::from("perfect")) };

    let equals_ok = EqualsTestStruct { val: CustomString(String::from("just enough")) };

    assert!(too_short.validate().is_err());
    assert!(too_long.validate().is_err());
    assert!(ok.validate().is_ok());
    assert!(equals_ok.validate().is_ok());
}
