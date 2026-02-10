use serde::Deserialize;
use validator::Validate;

#[test]
fn can_flatten_structs() {
    #[derive(Deserialize, Validate)]
    struct TestStruct {
        #[validate(range(min = -5))]
        field: i16,
        #[validate(nested)]
        inner_regular: InnerStruct,
        #[serde(flatten)]
        #[validate(nested)]
        inner_merged: FlattenedStruct,
    }

    #[derive(Deserialize, Validate)]
    struct InnerStruct {
        #[validate(length(max = 5))]
        test_sample: String,
        #[validate(range(max = 200))]
        something: i64,
    }

    #[derive(Deserialize, Validate)]
    struct FlattenedStruct {
        #[validate(length(max = 5))]
        hello_world: String,
        #[validate(range(max = 200))]
        anything: i64,
    }

    let s = TestStruct {
        field: -10,
        inner_regular: InnerStruct { test_sample: "abcdef".to_owned(), something: 500 },
        inner_merged: FlattenedStruct { hello_world: "abcdef".to_owned(), anything: 500 },
    };

    let errs = s.validate().unwrap_err().0;

    assert!(errs.contains_key("field"));
    assert!(errs.contains_key("inner_regular"));
    assert!(errs.contains_key("hello_world"));
    assert!(errs.contains_key("anything"));

    assert!(!errs.contains_key("inner_merged"));
    assert!(!errs.contains_key("test_sample"));
    assert!(!errs.contains_key("something"));
}
