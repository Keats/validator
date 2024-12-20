use serde::Serialize;
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};
use validator::{
    Validate, ValidateLength, ValidationError, ValidationErrors, ValidationErrorsKind,
};

#[test]
fn is_fine_with_nested_validations() {
    #[derive(Validate)]
    struct Root<'a> {
        #[validate(length(min = 5, max = 10))]
        value: String,
        #[validate(nested)]
        a: &'a A,
    }

    #[derive(Validate)]
    struct A {
        #[validate(length(min = 5, max = 10))]
        value: String,
        #[validate(nested)]
        b: B,
    }

    #[derive(Validate)]
    struct B {
        #[validate(length(min = 5, max = 10))]
        value: String,
    }

    let root = Root {
        value: "valid".to_string(),
        a: &A { value: "valid".to_string(), b: B { value: "valid".to_string() } },
    };

    assert!(root.validate().is_ok());
}

#[test]
fn fails_nested_validation_multiple_members() {
    #[derive(Validate)]
    struct Root<'a> {
        #[validate(length(min = 5, max = 10))]
        value: String,
        #[validate(nested)]
        a: &'a A,
    }

    #[derive(Validate)]
    struct A {
        #[validate(length(min = 5, max = 10))]
        value1: String,
        #[validate(length(min = 5, max = 10))]
        value2: String,
    }

    let root = Root {
        value: "valid".to_string(),
        a: &A { value1: "invalid value".to_string(), value2: "invalid value".to_string() },
    };

    let error_kind = ValidationErrorsKind::Field(vec![{
        let mut error = ValidationError::new("length");
        error.add_param("min".into(), &5);
        error.add_param("max".into(), &10);
        error.add_param("value".into(), &"invalid value");
        error
    }]);
    assert_eq!(
        root.validate(),
        Err(ValidationErrors(HashMap::from_iter([(
            "a",
            ValidationErrorsKind::Struct(Box::new(ValidationErrors(HashMap::from_iter([
                ("value1", error_kind.clone()),
                ("value2", error_kind),
            ]))))
        )])))
    );
}

#[test]
fn fails_nested_validation() {
    #[derive(Validate)]
    struct Root<'a> {
        #[validate(length(min = 5, max = 10))]
        value: String,
        #[validate(nested)]
        a: &'a A,
    }

    #[derive(Validate)]
    struct A {
        #[validate(length(min = 5, max = 10))]
        value: String,
        #[validate(nested)]
        b: B,
    }

    #[derive(Validate)]
    struct B {
        #[validate(length(min = 5, max = 10))]
        value: String,
    }

    let root = Root {
        value: "valid".to_string(),
        a: &A { value: "invalid value".to_string(), b: B { value: "valid".to_string() } },
    };

    assert!(root.validate().is_err());

    let root = Root {
        value: "valid".to_string(),
        a: &A { value: "valid".to_string(), b: B { value: "invalid value".to_string() } },
    };

    assert!(root.validate().is_err());
}

// // #[test]
// // fn failed_validation_points_to_original_field_names() {
// //     let root = Root {
// //         value: String::new(),
// //         _a: &A { value: String::new(), _b: B { value: String::new() } },
// //     };

// //     let res = root.validate();
// //     assert!(res.is_err());
// //     let err = res.unwrap_err();
// //     let errs = err.errors();
// //     assert_eq!(errs.len(), 2);
// //     assert!(errs.contains_key("value"));
// //     if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
// //         assert_eq!(errs.len(), 1);
// //         assert_eq!(errs[0].code, "length");
// //     } else {
// //         panic!("Expected field validation errors");
// //     }
// //     assert!(errs.contains_key("a"));
// //     if let ValidationErrorsKind::Struct(ref errs) = errs["a"] {
// //         unwrap_map(errs, |errs| {
// //             assert_eq!(errs.len(), 2);
// //             assert!(errs.contains_key("value"));
// //             if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
// //                 assert_eq!(errs.len(), 1);
// //                 assert_eq!(errs[0].code, "length");
// //             } else {
// //                 panic!("Expected field validation errors");
// //             }
// //             assert!(errs.contains_key("b"));
// //             if let ValidationErrorsKind::Struct(ref errs) = errs["b"] {
// //                 unwrap_map(errs, |errs| {
// //                     assert_eq!(errs.len(), 1);
// //                     assert!(errs.contains_key("value"));
// //                     if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
// //                         assert_eq!(errs.len(), 1);
// //                         assert_eq!(errs[0].code, "length");
// //                     } else {
// //                         panic!("Expected field validation errors");
// //                     }
// //                 });
// //             } else {
// //                 panic!("Expected struct validation errors");
// //             }
// //         });
// //     } else {
// //         panic!("Expected struct validation errors");
// //     }
// // }

#[test]
fn test_can_validate_option_fields_without_lifetime() {
    #[derive(Validate)]
    struct ParentWithOptionalChild {
        #[validate(nested)]
        child: Option<Child>,
    }

    #[derive(Validate)]
    struct Child {
        #[validate(length(min = 1))]
        value: String,
    }

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
    #[derive(Validate)]
    struct ParentWithLifetimeAndOptionalChild<'a> {
        #[validate(nested)]
        child: Option<&'a Child>,
    }

    #[derive(Validate)]
    struct Child {
        #[validate(length(min = 1))]
        value: String,
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
    #[derive(Validate)]
    struct ParentWithOptionalChild {
        #[validate(nested)]
        child: Option<Child>,
    }

    #[derive(Validate)]
    struct Child {
        #[validate(length(min = 1))]
        value: String,
    }

    let instance = ParentWithOptionalChild { child: None };

    let res = instance.validate();
    assert!(res.is_ok());
}

#[test]
fn test_can_validate_vector_fields() {
    #[derive(Validate)]
    struct ParentWithVectorOfChildren {
        #[validate(length(min = 1))]
        #[validate(nested)]
        child: Vec<Child>,
    }

    #[derive(Validate, Serialize)]
    struct Child {
        #[validate(length(min = 1))]
        value: String,
    }

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

    // A valid struct should not fail
    let instance = ParentWithVectorOfChildren {
        child: vec![Child { value: "valid".to_string() }, Child { value: "valid".to_string() }],
    };
    assert!(instance.validate().is_ok());
}

#[test]
fn test_can_validate_slice_fields() {
    #[derive(Validate)]
    struct ParentWithSliceOfChildren<'a> {
        #[validate(length(min = 1))]
        #[validate(nested)]
        child: &'a [Child],
    }

    #[derive(Validate, Serialize)]
    struct Child {
        #[validate(length(min = 1))]
        value: String,
    }

    let child = vec![
        Child { value: "valid".to_string() },
        Child { value: String::new() },
        Child { value: "valid".to_string() },
        Child { value: String::new() },
    ];
    let instance = ParentWithSliceOfChildren { child: &child };

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

    // A valid struct should not fail
    let child = vec![Child { value: "valid".to_string() }, Child { value: "valid".to_string() }];
    let instance = ParentWithSliceOfChildren { child: &child };
    assert!(instance.validate().is_ok());
}

#[test]
fn test_can_validate_array_fields() {
    #[derive(Validate)]
    struct ParentWithArrayOfChildren {
        #[validate(length(min = 1))]
        #[validate(nested)]
        child: [Child; 4],
    }

    #[derive(Validate, Serialize)]
    struct Child {
        #[validate(length(min = 1))]
        value: String,
    }

    let instance = ParentWithArrayOfChildren {
        child: [
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

    // A valid struct should not fail
    let instance = ParentWithArrayOfChildren {
        child: [
            Child { value: "valid".to_string() },
            Child { value: "valid".to_string() },
            Child { value: "valid".to_string() },
            Child { value: "valid".to_string() },
        ],
    };
    assert!(instance.validate().is_ok());
}

#[test]
fn test_can_validate_option_vector_fields() {
    #[derive(Validate)]
    struct ParentWithOptionVectorOfChildren {
        #[validate(length(min = 1))]
        #[validate(nested)]
        child: Option<Vec<Child>>,
    }

    #[derive(Validate, Serialize)]
    struct Child {
        #[validate(length(min = 1))]
        value: String,
    }

    let instance = ParentWithOptionVectorOfChildren {
        child: Some(vec![
            Child { value: "valid".to_string() },
            Child { value: String::new() },
            Child { value: "valid".to_string() },
            Child { value: String::new() },
        ]),
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

    // A valid struct should not fail
    let instance = ParentWithOptionVectorOfChildren {
        child: Some(vec![
            Child { value: "valid".to_string() },
            Child { value: "valid".to_string() },
        ]),
    };
    assert!(instance.validate().is_ok());
}

#[test]
fn test_can_validate_map_fields() {
    #[derive(Validate)]
    struct ParentWithMapOfChildren {
        #[validate(length(min = 1))]
        #[validate(nested)]
        child: HashMap<i8, Child>,
    }

    #[derive(Validate, Serialize, Clone)]
    struct Child {
        #[validate(length(min = 1))]
        value: String,
    }

    let instance = ParentWithMapOfChildren {
        child: [(0, Child { value: String::new() })].iter().cloned().collect(),
    };

    let res = instance.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.errors();
    assert_eq!(errs.len(), 1);
    assert!(errs.contains_key("child"));
    if let ValidationErrorsKind::List(ref errs) = errs["child"] {
        assert!(errs.contains_key(&0));
        unwrap_map(&errs[&0], |errs| {
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

    // A valid struct should not fail
    let instance = ParentWithMapOfChildren {
        child: [(0, Child { value: "value".to_string() })].iter().cloned().collect(),
    };
    assert!(instance.validate().is_ok());
}

#[test]
fn test_can_validate_ref_map_fields() {
    #[derive(Validate)]
    struct ParentWithRefMapOfChildren<'a> {
        #[validate(length(min = 1))]
        #[validate(nested)]
        child: &'a HashMap<i8, Child>,
    }

    #[derive(Validate, Serialize, Clone)]
    struct Child {
        #[validate(length(min = 1))]
        value: String,
    }

    let child = [(0, Child { value: String::new() })].iter().cloned().collect();
    let instance = ParentWithRefMapOfChildren { child: &child };

    let res = instance.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.errors();
    assert_eq!(errs.len(), 1);
    assert!(errs.contains_key("child"));
    if let ValidationErrorsKind::List(ref errs) = errs["child"] {
        assert!(errs.contains_key(&0));
        unwrap_map(&errs[&0], |errs| {
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

    // A valid struct should not fail
    let child = [(0, Child { value: "value".to_string() })].iter().cloned().collect();
    let instance = ParentWithRefMapOfChildren { child: &child };
    assert!(instance.validate().is_ok());
}

#[test]
fn test_can_validate_option_map_fields() {
    #[derive(Validate)]
    struct ParentWithOptionMapOfChildren {
        #[validate(length(min = 1))]
        #[validate(nested)]
        child: Option<HashMap<i8, Child>>,
    }

    #[derive(Validate, Serialize, Clone)]
    struct Child {
        #[validate(length(min = 1))]
        value: String,
    }

    let instance = ParentWithOptionMapOfChildren {
        child: Some([(0, Child { value: String::new() })].iter().cloned().collect()),
    };

    let res = instance.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.errors();
    assert_eq!(errs.len(), 1);
    assert!(errs.contains_key("child"));
    if let ValidationErrorsKind::List(ref errs) = errs["child"] {
        assert!(errs.contains_key(&0));
        unwrap_map(&errs[&0], |errs| {
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

    // A valid struct should not fail
    let instance = ParentWithOptionMapOfChildren {
        child: Some([(0, Child { value: "value".to_string() })].iter().cloned().collect()),
    };
    assert!(instance.validate().is_ok());
}

#[test]
fn test_can_validate_set_fields() {
    #[derive(Validate)]
    struct ParentWithSetOfChildren {
        #[validate(length(min = 1))]
        #[validate(nested)]
        child: HashSet<Child>,
    }

    #[derive(Validate, Serialize, Clone, PartialEq, Eq, Hash)]
    struct Child {
        #[validate(length(min = 1))]
        value: String,
    }

    let instance = ParentWithSetOfChildren {
        child: [Child { value: String::new() }].iter().cloned().collect(),
    };

    let res = instance.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.errors();
    assert_eq!(errs.len(), 1);
    assert!(errs.contains_key("child"));
    if let ValidationErrorsKind::List(ref errs) = errs["child"] {
        assert!(errs.contains_key(&0));
        unwrap_map(&errs[&0], |errs| {
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

    // A valid struct should not fail
    let instance = ParentWithSetOfChildren {
        child: [Child { value: "value".to_string() }].iter().cloned().collect(),
    };
    assert!(instance.validate().is_ok());
}

#[test]
fn test_can_validate_ref_set_fields() {
    #[derive(Validate)]
    struct ParentWithRefSetOfChildren<'a> {
        #[validate(length(min = 1), nested)]
        child: &'a HashSet<Child>,
    }

    #[derive(Validate, Serialize, Clone, PartialEq, Eq, Hash)]
    struct Child {
        #[validate(length(min = 1))]
        value: String,
    }

    let child = [Child { value: String::new() }].iter().cloned().collect();
    let instance = ParentWithRefSetOfChildren { child: &child };

    let res = instance.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.errors();
    assert_eq!(errs.len(), 1);
    assert!(errs.contains_key("child"));
    if let ValidationErrorsKind::List(ref errs) = errs["child"] {
        assert!(errs.contains_key(&0));
        unwrap_map(&errs[&0], |errs| {
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

    // A valid struct should not fail
    let child = [Child { value: "value".to_string() }].iter().cloned().collect();
    let instance = ParentWithRefSetOfChildren { child: &child };
    assert!(instance.validate().is_ok());
}

#[test]
fn test_can_validate_option_set_fields() {
    #[derive(Validate)]
    struct ParentWithOptionSetOfChildren {
        #[validate(length(min = 1), nested)]
        child: Option<HashSet<Child>>,
    }

    #[derive(Validate, Serialize, Clone, PartialEq, Eq, Hash)]
    struct Child {
        #[validate(length(min = 1))]
        value: String,
    }

    let instance = ParentWithOptionSetOfChildren {
        child: Some([Child { value: String::new() }].iter().cloned().collect()),
    };

    let res = instance.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.errors();
    assert_eq!(errs.len(), 1);
    assert!(errs.contains_key("child"));
    if let ValidationErrorsKind::List(ref errs) = errs["child"] {
        assert!(errs.contains_key(&0));
        unwrap_map(&errs[&0], |errs| {
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

    // A valid struct should not fail
    let instance = ParentWithOptionSetOfChildren {
        child: Some([Child { value: "value".to_string() }].iter().cloned().collect()),
    };
    assert!(instance.validate().is_ok());
}

#[test]
fn test_field_validations_take_priority_over_nested_validations() {
    #[derive(Validate)]
    struct ParentWithVectorOfChildren {
        #[validate(length(min = 1))]
        child: Vec<Child>,
    }

    #[derive(Validate, Serialize, Clone, PartialEq, Eq, Hash)]
    struct Child {
        #[validate(length(min = 1))]
        value: String,
    }

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
#[should_panic(
    expected = "Attempt to add field validation to a non-Field ValidationErrorsKind instance"
)]
#[allow(unused)]
fn test_field_validations_evaluated_after_nested_validations_fails() {
    #[derive(Debug)]
    struct ParentWithStructValidationsFirst {
        child: Vec<Child>,
    }

    #[derive(Debug, Validate, Serialize)]
    struct Child {
        #[validate(length(min = 1))]
        value: String,
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
            if !self.child.validate_length(Some(2u64), None, None) {
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

#[allow(dead_code)]
fn unwrap_map<F>(errors: &ValidationErrors, f: F)
where
    F: FnOnce(HashMap<&'static str, ValidationErrorsKind>),
{
    let errors = errors.clone();
    f(errors.errors().clone());
}
