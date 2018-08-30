#[macro_use]
extern crate validator_derive;
extern crate validator;

use validator::Validate;

#[derive(Debug, Validate)]
struct Root<'a> {
    #[validate(length(min = "1"))]
    value: String,

    #[validate]
    a: &'a A,
}

#[derive(Debug, Validate)]
struct A {
    #[validate(length(min = "1"))]
    value: String,

    #[validate]
    b: B,
}

#[derive(Debug, Validate)]
struct B {
    #[validate(length(min = "1"))]
    value: String,
}

#[test]
fn is_fine_with_nested_validations() {
    let root = Root {
        value: "valid".to_string(),
        a: &A {
            value: "valid".to_string(),
            b: B {
                value: "valid".to_string(),
            }
        }
    };

    assert!(root.validate().is_ok());
}

#[test]
fn failed_validation_points_to_original_field_names() {
    let root = Root {
        value: "".to_string(),
        a: &A {
            value: "".to_string(),
            b: B {
                value: "".to_string(),
            }
        }
    };

    let res = root.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert_eq!(errs.len(), 3);
    assert!(errs.contains_key("value"));
    assert_eq!(errs["value"].len(), 1);
    assert_eq!(errs["value"][0].path, vec!["value"]);
    assert_eq!(errs["value"][0].code, "length");
    assert!(errs.contains_key("a.value"));
    assert_eq!(errs["a.value"].len(), 1);
    assert_eq!(errs["a.value"][0].path, vec!["a", "value"]);
    assert_eq!(errs["a.value"][0].code, "length");
    assert!(errs.contains_key("a.b.value"));
    assert_eq!(errs["a.b.value"].len(), 1);
    assert_eq!(errs["a.b.value"][0].path, vec!["a", "b", "value"]);
    assert_eq!(errs["a.b.value"][0].code, "length");
}