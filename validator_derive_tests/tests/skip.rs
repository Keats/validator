use validator::Validate;

#[test]
fn can_skip_field() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(skip)]
        #[validate(length(min = 5, max = 10))]
        _skipped: String,
    }

    let t = TestStruct { _skipped: "invalid value".to_string() };
    assert!(t.validate().is_ok())
}

#[test]
fn explicit_skip_field() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(skip = true)]
        #[validate(length(min = 5, max = 10))]
        _skipped: String,
    }

    let t = TestStruct { _skipped: "invalid value".to_string() };
    assert!(t.validate().is_ok())
}

#[test]
fn explicit_dont_skip_field() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(skip = false)]
        #[validate(length(min = 5, max = 10))]
        _skipped: String,
    }

    let t = TestStruct { _skipped: "invalid value".to_string() };
    assert!(t.validate().is_err())
}
