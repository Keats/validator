use serde::Serialize;
use std::{borrow::Cow, collections::HashMap};
use validator::{
    validate_length, Validate, ValidationError, ValidationErrors, ValidationErrorsKind, Validator,
};

#[derive(Debug, Validate)]
struct Root<'a> {
    #[validate(length(min = 1))]
    value: String,

    #[validate]
    a: &'a A,
}

#[derive(Debug, Validate)]
struct A {
    #[validate(length(min = 1))]
    value: String,

    #[validate]
    b: B,
}

#[derive(Debug, Validate)]
struct B {
    #[validate(length(min = 1))]
    value: String,
}

#[derive(Debug, Validate)]
struct ParentWithOptionalChild {
    #[validate]
    child: Option<Child>,
}

#[derive(Debug, Validate)]
struct ParentWithVectorOfChildren {
    #[validate]
    #[validate(length(min = 1))]
    child: Vec<Child>,
}

#[derive(Debug, Validate, Serialize)]
struct Child {
    #[validate(length(min = 1))]
    value: String,
}

#[test]
fn is_fine_with_nested_validations() {
    let root = Root {
        value: "valid".to_string(),
        a: &A { value: "valid".to_string(), b: B { value: "valid".to_string() } },
    };

    assert!(root.validate().is_ok());
}

#[test]
fn failed_validation_points_to_original_field_names() {
    let root = Root {
        value: String::new(),
        a: &A { value: String::new(), b: B { value: String::new() } },
    };

    let res = root.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.errors();
    assert_eq!(errs.len(), 2);
    assert!(errs.contains_key("value"));
    if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
        assert_eq!(errs.len(), 1);
        assert_eq!(errs[0].code, "length");
    } else {
        panic!("Expected field validation errors");
    }
    assert!(errs.contains_key("a"));
    if let ValidationErrorsKind::Struct(ref errs) = errs["a"] {
        unwrap_map(errs, |errs| {
            assert_eq!(errs.len(), 2);
            assert!(errs.contains_key("value"));
            if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                assert_eq!(errs.len(), 1);
                assert_eq!(errs[0].code, "length");
            } else {
                panic!("Expected field validation errors");
            }
            assert!(errs.contains_key("b"));
            if let ValidationErrorsKind::Struct(ref errs) = errs["b"] {
                unwrap_map(errs, |errs| {
                    assert_eq!(errs.len(), 1);
                    assert!(errs.contains_key("value"));
                    if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                        assert_eq!(errs.len(), 1);
                        assert_eq!(errs[0].code, "length");
                    } else {
                        panic!("Expected field validation errors");
                    }
                });
            } else {
                panic!("Expected struct validation errors");
            }
        });
    } else {
        panic!("Expected struct validation errors");
    }
}

#[test]
fn test_can_validate_option_fields_without_lifetime() {
    let instance = ParentWithOptionalChild { child: Some(Child { value: String::new() }) };

    let res = instance.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.errors();
    assert_eq!(errs.len(), 1);
    assert!(errs.contains_key("child"));
    if let ValidationErrorsKind::Struct(ref errs) = errs["child"] {
        unwrap_map(errs, |errs| {
            assert_eq!(errs.len(), 1);
            assert!(errs.contains_key("value"));
            if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                assert_eq!(errs.len(), 1);
                assert_eq!(errs[0].code, "length");
            } else {
                panic!("Expected field validation errors");
            }
        });
    } else {
        panic!("Expected struct validation errors");
    }
}

#[test]
fn test_can_validate_option_fields_with_lifetime() {
    #[derive(Debug, Validate)]
    struct ParentWithLifetimeAndOptionalChild<'a> {
        #[validate]
        child: Option<&'a Child>,
    }

    let child = Child { value: String::new() };

    let instance = ParentWithLifetimeAndOptionalChild { child: Some(&child) };

    let res = instance.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.errors();
    assert_eq!(errs.len(), 1);
    assert!(errs.contains_key("child"));
    if let ValidationErrorsKind::Struct(ref errs) = errs["child"] {
        unwrap_map(errs, |errs| {
            assert_eq!(errs.len(), 1);
            assert!(errs.contains_key("value"));
            if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                assert_eq!(errs.len(), 1);
                assert_eq!(errs[0].code, "length");
            } else {
                panic!("Expected field validation errors");
            }
        });
    } else {
        panic!("Expected struct validation errors");
    }
}

#[test]
fn test_works_with_none_values() {
    let instance = ParentWithOptionalChild { child: None };

    let res = instance.validate();
    assert!(res.is_ok());
}

#[test]
fn test_can_validate_vector_fields() {
    let instance = ParentWithVectorOfChildren {
        child: vec![
            Child { value: "valid".to_string() },
            Child { value: String::new() },
            Child { value: "valid".to_string() },
            Child { value: String::new() },
        ],
    };

    let res = instance.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.errors();
    assert_eq!(errs.len(), 1);
    assert!(errs.contains_key("child"));
    if let ValidationErrorsKind::List(ref errs) = errs["child"] {
        assert!(errs.contains_key(&1));
        unwrap_map(&errs[&1], |errs| {
            assert_eq!(errs.len(), 1);
            assert!(errs.contains_key("value"));
            if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                assert_eq!(errs.len(), 1);
                assert_eq!(errs[0].code, "length");
            } else {
                panic!("Expected field validation errors");
            }
        });
        assert!(errs.contains_key(&3));
        unwrap_map(&errs[&3], |errs| {
            assert_eq!(errs.len(), 1);
            assert!(errs.contains_key("value"));
            if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                assert_eq!(errs.len(), 1);
                assert_eq!(errs[0].code, "length");
            } else {
                panic!("Expected field validation errors");
            }
        });
    } else {
        panic!("Expected list validation errors");
    }
}

#[test]
fn test_field_validations_take_priority_over_nested_validations() {
    let instance = ParentWithVectorOfChildren { child: Vec::new() };

    let res = instance.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.errors();
    assert_eq!(errs.len(), 1);
    assert!(errs.contains_key("child"));
    if let ValidationErrorsKind::Field(ref errs) = errs["child"] {
        assert_eq!(errs.len(), 1);
        assert_eq!(errs[0].code, "length");
    } else {
        panic!("Expected field validation errors");
    }
}

#[test]
#[should_panic(expected = "Attempt to replace non-empty ValidationErrors entry")]
#[allow(unused)]
fn test_field_validation_errors_replaced_with_nested_validations_fails() {
    #[derive(Debug)]
    struct ParentWithOverridingStructValidations {
        child: Vec<Child>,
    }

    impl Validate for ParentWithOverridingStructValidations {
        // Evaluating structs after fields validations have discovered errors should fail because
        // field validations are expected to take priority over nested struct validations
        #[allow(unused_mut)]
        fn validate(&self) -> Result<(), ValidationErrors> {
            // First validate the length of the vector:
            let mut errors = ValidationErrors::new();
            if !validate_length(
                Validator::Length { min: Some(2u64), max: None, equal: None },
                &self.child,
            ) {
                let mut err = ValidationError::new("length");
                err.add_param(Cow::from("min"), &2u64);
                err.add_param(Cow::from("value"), &&self.child);
                errors.add("child", err);
            }

            // Then validate the nested vector of structs without checking for existing field errors:
            let mut result = if errors.is_empty() { Ok(()) } else { Err(errors) };
            {
                let results: Vec<_> = self
                    .child
                    .iter()
                    .map(|child| {
                        let mut result = Ok(());
                        result = ValidationErrors::merge(result, "child", child.validate());
                        result
                    })
                    .collect();
                result = ValidationErrors::merge_all(result, "child", results);
            }
            result
        }
    }

    let instance =
        ParentWithOverridingStructValidations { child: vec![Child { value: String::new() }] };
    instance.validate();
}

#[test]
#[should_panic(
    expected = "Attempt to add field validation to a non-Field ValidationErrorsKind instance"
)]
#[allow(unused)]
fn test_field_validations_evaluated_after_nested_validations_fails() {
    #[derive(Debug)]
    struct ParentWithStructValidationsFirst {
        child: Vec<Child>,
    }

    impl Validate for ParentWithStructValidationsFirst {
        // Evaluating fields after their nested structs should fail because field
        // validations are expected to take priority over nested struct validations
        #[allow(unused_mut)]
        fn validate(&self) -> Result<(), ValidationErrors> {
            // First validate the nested vector of structs:
            let mut result = Ok(());
            if !ValidationErrors::has_error(&result, "child") {
                let results: Vec<_> = self
                    .child
                    .iter()
                    .map(|child| {
                        let mut result = Ok(());
                        result = ValidationErrors::merge(result, "child", child.validate());
                        result
                    })
                    .collect();
                result = ValidationErrors::merge_all(result, "child", results);
            }

            // Then validate the length of the vector itself:
            if !validate_length(
                Validator::Length { min: Some(2u64), max: None, equal: None },
                &self.child,
            ) {
                let mut err = ValidationError::new("length");
                err.add_param(Cow::from("min"), &2u64);
                err.add_param(Cow::from("value"), &&self.child);
                result = result.and_then(|_| Err(ValidationErrors::new())).map_err(|mut errors| {
                    errors.add("child", err);
                    errors
                });
            }
            result
        }
    }

    let instance = ParentWithStructValidationsFirst { child: vec![Child { value: String::new() }] };
    let res = instance.validate();
}

fn unwrap_map<F>(errors: &ValidationErrors, f: F)
where
    F: FnOnce(HashMap<&'static str, ValidationErrorsKind>),
{
    let errors = errors.clone();
    f(errors.errors().clone());
}
