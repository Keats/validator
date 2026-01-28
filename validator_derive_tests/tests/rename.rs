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
