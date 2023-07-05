use lazy_regex::lazy_regex;
use once_cell::sync::Lazy;
use regex::Regex;
use validator::Validate;

static RE2: Lazy<Regex> = lazy_regex!(r"^[a-z]{2}$");

#[test]
fn can_validate_valid_regex() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(regex = "crate::RE2")]
        val: String,
    }

    let s = TestStruct { val: "aa".to_string() };

    assert!(s.validate().is_ok());
}

#[test]
fn bad_value_for_regex_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(regex = "crate::RE2")]
        val: String,
    }

    let s = TestStruct { val: "2".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "regex");
    assert_eq!(errs["val"][0].params["value"], "2");
}

#[test]
fn can_specify_code_for_regex() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(regex(path = "crate::RE2", code = "oops"))]
        val: String,
    }
    let s = TestStruct { val: "2".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "oops");
}

#[test]
fn can_specify_message_for_regex() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(regex(path = "crate::RE2", message = "oops"))]
        val: String,
    }
    let s = TestStruct { val: "2".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].clone().message.unwrap(), "oops");
}
