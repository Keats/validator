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

#[derive(Debug, Validate)]
struct ParentWithOptionalChild {
    #[validate]
    child: Option<Child>,
}

#[derive(Debug, Validate)]
struct ParentWithLifetimeAndOptionalChild<'a> {
    #[validate]
    child: Option<&'a Child>,
}

#[derive(Debug, Validate)]
struct ParentWithVectorOfChildren {
    #[validate]
    child: Vec<Child>,
}

#[derive(Debug, Validate)]
struct Child {
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
        value: String::new(),
        a: &A {
            value: String::new(),
            b: B {
                value: String::new(),
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

#[test]
fn test_can_validate_option_fields_without_lifetime() {
    let instance = ParentWithOptionalChild {
        child: Some(Child {
            value: String::new(),
        })
    };

    let res = instance.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert_eq!(errs.len(), 1);
    assert!(errs.contains_key("child.value"));
    assert_eq!(errs["child.value"].len(), 1);
    assert_eq!(errs["child.value"][0].path, vec!["child", "value"]);
    assert_eq!(errs["child.value"][0].code, "length");
}

#[test]
fn test_can_validate_option_fields_with_lifetime() {
    let child = Child {
        value: String::new(),
    };

    let instance = ParentWithLifetimeAndOptionalChild {
        child: Some(&child)
    };

    let res = instance.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert_eq!(errs.len(), 1);
    assert!(errs.contains_key("child.value"));
    assert_eq!(errs["child.value"].len(), 1);
    assert_eq!(errs["child.value"][0].path, vec!["child", "value"]);
    assert_eq!(errs["child.value"][0].code, "length");
}

#[test]
fn test_works_with_none_values() {
    let instance = ParentWithOptionalChild {
        child: None,
    };

    let res = instance.validate();
    assert!(res.is_ok());
}

#[test]
fn test_can_validate_vector_fields() {
    let instance = ParentWithVectorOfChildren {
        child: vec![
            Child {
                value: "valid".to_string(),
            },
            Child {
                value: String::new(),
            },
            Child {
                value: "valid".to_string(),
            },
            Child {
                value: String::new(),
            }
        ],
    };

    let res = instance.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert_eq!(errs.len(), 2);
    assert!(errs.contains_key("child[1].value"));
    assert_eq!(errs["child[1].value"].len(), 1);
    assert_eq!(errs["child[1].value"][0].path, vec!["child[1]", "value"]);
    assert_eq!(errs["child[1].value"][0].code, "length");
    assert!(errs.contains_key("child[3].value"));
    assert_eq!(errs["child[3].value"].len(), 1);
    assert_eq!(errs["child[3].value"][0].path, vec!["child[3]", "value"]);
    assert_eq!(errs["child[3].value"][0].code, "length");
}