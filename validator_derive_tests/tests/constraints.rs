use validator::{
    Constraints, LengthConstraint, Validate, ValidationConstraint, ValidationConstraintsKind,
};

#[derive(Debug, Validate)]
struct A {
    #[validate(length(max = 10, code = "a_length"))]
    value: String,

    #[validate(nested)]
    b: B,
}

#[derive(Debug, Validate)]
struct B {
    #[validate(length(min = 1, code = "b_length"))]
    value: String,
}

#[test]
fn a_constraints_correct() {
    let a_constraints = <A as Constraints>::constraints();

    let mut keys = a_constraints.0.keys().collect::<Vec<_>>();
    keys.sort();
    assert_eq!(keys, &[&"b", &"value"]);

    {
        let value_constraints = &a_constraints.0["value"];
        assert_eq!(value_constraints.len(), 1);

        let struct_constraints = match &value_constraints[0] {
            ValidationConstraintsKind::Field(constraints) => constraints,
            _ => panic!("Expected a constraint kind of Field, found {:?}", &value_constraints[0]),
        };

        assert_eq!(struct_constraints.len(), 1);

        assert_eq!(
            struct_constraints[0],
            ValidationConstraint::Length {
                length: LengthConstraint::Range { min: None, max: Some(10) },
                code: "a_length".into(),
            }
        );
    }

    {
        let b_constraints = &a_constraints.0["b"];
        assert_eq!(b_constraints.len(), 1);

        let struct_constraints = match &b_constraints[0] {
            ValidationConstraintsKind::Struct(constraints) => constraints.as_ref(),
            _ => panic!("Expected a constraint kind of Field, found {:?}", &b_constraints[0]),
        };

        assert_eq!(struct_constraints, &<B as Constraints>::constraints());
    }
}

#[test]
fn b_constraints_correct() {
    let b_constraints = <B as Constraints>::constraints();
    assert_eq!(b_constraints.0.keys().collect::<Vec<_>>(), &[&"value"]);

    let constraint_kinds = &b_constraints.0["value"];
    assert_eq!(constraint_kinds.len(), 1);

    let struct_constraints = match &constraint_kinds[0] {
        ValidationConstraintsKind::Field(constraints) => constraints,
        _ => panic!("Expected a constraint kind of Field, found {:?}", &constraint_kinds[0]),
    };

    assert_eq!(struct_constraints.len(), 1);

    assert_eq!(
        struct_constraints[0],
        ValidationConstraint::Length {
            length: LengthConstraint::Range { min: Some(1), max: None },
            code: "b_length".into(),
        }
    );
}
