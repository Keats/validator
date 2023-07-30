use validator::Validate;

#[test]
fn can_validate_ipv4() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(ip)]
        val: String,
    }

    let s = TestStruct { val: "192.168.1.1".to_string() };

    assert!(s.validate().is_ok());
}
