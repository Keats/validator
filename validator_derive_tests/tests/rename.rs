use serde::Deserialize;
use validator::Validate;

#[test]
fn renames_fields() {
    #[derive(Deserialize, Validate)]
    struct TestStruct {
        #[serde(rename = "fieldNAME123")]
        #[validate(range(min = -5))]
        field_name: i16,
        #[serde(default, skip_serializing, rename = "_SomeTest")]
        #[validate(length(max = 5))]
        some_test: String,
    }

    let s = TestStruct { field_name: -10, some_test: "abcdef".to_owned() };

    let err = s.validate().unwrap_err();
    let errs = err.field_errors();

    assert!(errs.contains_key("fieldNAME123"));
    assert!(errs.contains_key("_SomeTest"));
    assert!(!errs.contains_key("field_name"));
    assert!(!errs.contains_key("some_test"));
}

#[test]
fn renames_fields_as_in_deserialize() {
    #[derive(Deserialize, Validate)]
    struct TestStruct {
        #[serde(rename(serialize = "abc", deserialize = "fieldNAME123"))]
        #[validate(range(min = -5))]
        field_name: i16,
        #[serde(default, skip_serializing, rename(deserialize = "_SomeTest"))]
        #[validate(length(max = 5))]
        some_test: String,
    }

    let s = TestStruct { field_name: -10, some_test: "abcdef".to_owned() };

    let err = s.validate().unwrap_err();
    let errs = err.field_errors();

    assert!(errs.contains_key("fieldNAME123"));
    assert!(errs.contains_key("_SomeTest"));
    assert!(!errs.contains_key("abc"));
    assert!(!errs.contains_key("field_name"));
    assert!(!errs.contains_key("some_test"));
}

#[test]
fn rename_all_camel_case_works() {
    #[derive(Deserialize, Validate)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    struct TestStruct {
        #[validate(range(min = -5))]
        field_name: i16,
        #[validate(length(max = 5))]
        some_test_hello: String,
    }

    let s = TestStruct { field_name: -10, some_test_hello: "abcdef".to_owned() };

    let err = s.validate().unwrap_err();
    let errs = err.field_errors();

    assert!(errs.contains_key("fieldName"));
    assert!(errs.contains_key("someTestHello"));
    assert!(!errs.contains_key("field_name"));
    assert!(!errs.contains_key("some_test_hello"));
}

#[test]
fn rename_all_camel_case_as_in_deserialize() {
    #[derive(Deserialize, Validate)]
    #[serde(deny_unknown_fields, rename_all(serialize = "kebab-case", deserialize = "camelCase"))]
    struct TestStruct {
        #[validate(range(min = -5))]
        field_name: i16,
        #[validate(length(max = 5))]
        some_test_hello: String,
    }

    let s = TestStruct { field_name: -10, some_test_hello: "abcdef".to_owned() };

    let err = s.validate().unwrap_err();
    let errs = err.field_errors();

    assert!(errs.contains_key("fieldName"));
    assert!(errs.contains_key("someTestHello"));
    assert!(!errs.contains_key("field-name"));
    assert!(!errs.contains_key("some-test-hello"));
    assert!(!errs.contains_key("field_name"));
    assert!(!errs.contains_key("some_test_hello"));
}

#[test]
fn rename_all_kebab_uppercase_works() {
    #[derive(Deserialize, Validate)]
    #[serde(rename_all = "SCREAMING-KEBAB-CASE")]
    struct TestStruct {
        #[validate(range(min = -5))]
        field_name: i16,
        #[validate(length(max = 5))]
        some_test_hello: String,
    }

    let s = TestStruct { field_name: -10, some_test_hello: "abcdef".to_owned() };

    let err = s.validate().unwrap_err();
    let errs = err.field_errors();

    assert!(errs.contains_key("FIELD-NAME"));
    assert!(errs.contains_key("SOME-TEST-HELLO"));
    assert!(!errs.contains_key("field_name"));
    assert!(!errs.contains_key("some_test_hello"));
}

#[test]
fn rename_all_pascal_with_custom() {
    #[derive(Deserialize, Validate)]
    #[serde(rename_all = "PascalCase")]
    struct TestStruct {
        #[validate(range(min = -5))]
        field_name: i16,
        #[serde(default, skip_serializing, rename = "_Some-test-123")]
        #[validate(length(max = 5))]
        some_test_hello: String,
    }

    let s = TestStruct { field_name: -10, some_test_hello: "abcdef".to_owned() };

    let err = s.validate().unwrap_err();
    let errs = err.field_errors();

    assert!(errs.contains_key("FieldName"));
    assert!(errs.contains_key("_Some-test-123"));
    assert!(!errs.contains_key("field_name"));
    assert!(!errs.contains_key("some_test_hello"));
}
