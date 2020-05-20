#[macro_use]
extern crate validator_derive;

use serde::Serialize;
use validator::Validate;

#[derive(Debug, Serialize)]
struct ObjectRef {
    id: i32,
    name: String,
}

#[derive(Debug, Validate)]
struct TestStruct {
    #[validate(required)]
    val: Option<ObjectRef>,
}

#[test]
fn can_validate_some() {
    let s = TestStruct { val: Some(ObjectRef { id: 0, name: String::new() }) };

    assert!(s.validate().is_ok());
}

#[test]
fn none_fails_validate() {
    let s = TestStruct { val: None };

    assert!(s.validate().is_err());
}
