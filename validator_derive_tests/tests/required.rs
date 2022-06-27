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

#[test]
fn can_specify_code_for_required() {
	#[derive(Debug, Validate)]
	struct TestStruct {
		#[validate(required(code = "oops"))]
		val: Option<String>
	}
	let s = TestStruct { val: None };
	let res = s.validate();
	assert!(res.is_err());
	let err = res.unwrap_err();
	let errs = err.field_errors();
	assert!(errs.contains_key("val"));
	assert_eq!(errs["val"].len(), 1);
	assert_eq!(errs["val"][0].code, "oops");
}

#[test]
fn can_specify_message_for_required() {
	#[derive(Debug, Validate)]
	struct TestStruct {
		#[validate(required(message = "oops"))]
		val: Option<String>
	}
	let s = TestStruct { val: None };
	let res = s.validate();
	assert!(res.is_err());
	let err = res.unwrap_err();
	let errs = err.field_errors();
	assert!(errs.contains_key("val"));
	assert_eq!(errs["val"].len(), 1);
	assert_eq!(errs["val"][0].clone().message.unwrap(), "oops");
}