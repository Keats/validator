use validator::Validate;

#[test]
fn can_validate_valid_email_with_unsupported_array() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(email)]
        val: String,
        #[allow(dead_code)]
        array: [u8; 32],
    }

    let s = TestStruct { val: "bob@bob.com".to_string(), array: [0u8; 32] };

    assert!(s.validate().is_ok());
}
