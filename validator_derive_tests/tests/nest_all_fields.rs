use validator::Validate;

#[test]
fn field_without_attribute_ignored() {
    #[derive(Validate)]
    struct Test {
        _a: Nested,
        #[validate(nested)]
        b: NestedValidated,
    }

    struct Nested {
        _val: String,
    }

    #[derive(Validate)]
    struct NestedValidated {
        #[validate(length(min = 5, max = 10))]
        val: String,
    }

    let test = Test {
        _a: Nested { _val: "test".to_string() },
        b: NestedValidated { val: "valid str".to_string() },
    };

    assert!(test.validate().is_ok());
}

#[test]
fn nest_all_fields_attribute_works() {
    #[derive(Validate)]
    #[validate(nest_all_fields)]
    struct Test {
        #[validate(skip)]
        _a: Nested,
        b: NestedValidated,
    }

    struct Nested {
        _val: String,
    }

    #[derive(Validate)]
    struct NestedValidated {
        #[validate(length(min = 5, max = 10))]
        val: String,
    }

    let test = Test {
        _a: Nested { _val: "test".to_string() },
        b: NestedValidated { val: "valid str".to_string() },
    };

    assert!(test.validate().is_ok());
}

#[test]
fn nest_all_fields_attribute_ignores_validated_fields() {
    #[derive(Validate)]
    #[validate(nest_all_fields)]
    struct Test {
        #[validate(skip)]
        _a: Nested,
        b: NestedValidated,
        #[validate(range(min = 10))]
        c: u8,
    }

    struct Nested {
        _val: String,
    }

    #[derive(Validate)]
    struct NestedValidated {
        #[validate(length(min = 5, max = 10))]
        val: String,
    }

    let test = Test {
        _a: Nested { _val: "test".to_string() },
        b: NestedValidated { val: "valid str".to_string() },
        c: 11,
    };

    assert!(test.validate().is_ok());

    let test_invalid = Test {
        _a: Nested { _val: "test".to_string() },
        b: NestedValidated { val: "valid str".to_string() },
        c: 0,
    };

    assert!(test_invalid.validate().is_err());
}
