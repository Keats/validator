use validator::{Validate, ValidationErrors};

const MAX_CONST: usize = 10;
const MIN_CONST: usize = 0;

// Loose floating point comparison using EPSILON error bound
macro_rules! assert_float {
    ($e1:expr, $e2:expr) => {
        assert!(($e2 - $e1).abs() < f64::EPSILON);
    };
}

#[test]
fn can_validate_range_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(min = 5, max = 10))]
        val: usize,
    }

    let s = TestStruct { val: 6 };

    assert!(s.validate().is_ok());
}

#[test]
fn can_validate_only_min_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(min = 5))]
        val: usize,
    }

    let s = TestStruct { val: 6 };

    assert!(s.validate().is_ok());
}

#[test]
fn can_validate_only_max_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(max = 50))]
        val: usize,
    }

    let s = TestStruct { val: 6 };

    assert!(s.validate().is_ok());
}

#[test]
fn can_validate_range_value_crate_path_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(min = "MIN_CONST", max = "MAX_CONST"))]
        val: usize,
    }

    let s = TestStruct { val: 6 };

    assert!(s.validate().is_ok());
}

#[test]
fn can_validate_range_value_self_path_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        max: usize,
        min: usize,
        #[validate(range(min = "self.min", max = "self.max"))]
        val: usize,
    }

    let s = TestStruct { max: 8, min: 4, val: 6 };

    assert!(s.validate().is_ok());
}

#[test]
fn value_out_of_range_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(min = 5, max = 10))]
        val: usize,
    }

    let s = TestStruct { val: 11 };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "range");
}

#[test]
fn value_out_of_range_fails_validation_with_self_path() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        max: usize,
        min: usize,
        #[validate(range(min = "self.min", max = "self.max"))]
        val: usize,
    }

    let s = TestStruct { min: 4, max: 5, val: 6 };

    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "range");
}

#[test]
fn value_out_of_range_fails_validation_with_crate_path() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        max: usize,
        min: usize,
        #[validate(range(min = "MIN_CONST", max = "MAX_CONST"))]
        val: usize,
    }

    let s = TestStruct { min: 4, max: 5, val: 6 };

    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    println!("{}", err);
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "range");
}

#[test]
fn can_specify_code_for_range() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(min = 5, max = 10, code = "oops"))]
        val: usize,
    }
    let s = TestStruct { val: 11 };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "oops");
    assert_eq!(errs["val"][0].params["value"], 11);
    assert_float!(errs["val"][0].params["min"].as_f64().unwrap(), 5.0);
    assert_float!(errs["val"][0].params["max"].as_f64().unwrap(), 10.0);
}

#[test]
fn can_specify_message_for_range() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(min = 5, max = 10, message = "oops"))]
        val: usize,
    }
    let s = TestStruct { val: 1 };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].clone().message.unwrap(), "oops");
}

#[test]
fn can_pass_reference_as_validate() {
    // This tests that the blanket Validate implementation on
    // `&T where T:Validate` works properly

    #[derive(Validate)]
    struct TestStruct {
        #[validate(range(min = 100))]
        num_field: u32,
    }

    fn validate<T: Validate>(value: T) -> Result<(), ValidationErrors> {
        value.validate()
    }

    let val = TestStruct { num_field: 10 };
    validate(&val).unwrap_err();
    assert_eq!(val.num_field, 10);
}
