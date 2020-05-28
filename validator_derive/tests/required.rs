#[macro_use]
extern crate validator_derive;

use serde::Serialize;
use validator::Validate;

#[derive(Serialize)]
struct ObjectRef {
    id: i32,
    name: String,
}

#[derive(Serialize, Validate)]
struct CheckedObjectRef {
    #[validate(range(min = 1))]
    id: i32,
    #[validate(length(min = 1))]
    name: String,
}

#[derive(Validate)]
struct Required {
    #[validate(required)]
    val: Option<ObjectRef>,
}

#[derive(Validate)]
struct RequiredNested {
    #[validate(required_nested)]
    val: Option<CheckedObjectRef>,
}

#[test]
fn can_validate_required() {
    let s = Required { val: Some(ObjectRef { id: 0, name: String::new() }) };

    assert!(s.validate().is_ok());
}

#[test]
fn can_validate_required_nested() {
    let s = RequiredNested {
        val: Some(CheckedObjectRef { id: 1, name: String::from("Reference representation") }),
    };

    assert!(s.validate().is_ok());
}

#[test]
fn none_fails_required() {
    let s = Required { val: None };

    assert!(s.validate().is_err());
}

#[test]
fn none_fails_required_nested() {
    let s = RequiredNested { val: None };

    assert!(s.validate().is_err());
}
