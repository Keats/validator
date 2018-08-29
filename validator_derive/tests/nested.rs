#[macro_use]
extern crate validator_derive;
extern crate validator;

use validator::Validate;

#[derive(Debug, Validate)]
struct Root {
    #[validate]
    nested: Nested,
}

#[derive(Debug, Validate)]
struct Nested {
    #[validate(length(min = "1"))]
    value: String,
}

#[test]
fn is_fine_with_nested_validations() {
    let root = Root {
        nested: Nested {
            value: "Something".to_string(),
        }
    };

    assert!(root.validate().is_ok());
}

#[test]
fn failed_validation_points_to_original_field_names() {
    let root = Root {
        nested: Nested {
            value: "".to_string(),
        }
    };

    let res = root.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("value"));
    assert_eq!(errs["value"].len(), 1);
    assert_eq!(errs["value"][0].code, "length");
}