use serde::Serialize;
use validator::Validate;

#[test]
fn can_validate_ipv4() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(ip(v4))]
        val: String,
    }

    let s = TestStruct { val: "192.168.1.1".to_string() };

    assert!(s.validate().is_ok());
}

#[test]
fn can_validate_ipv6() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(ip(v6))]
        val: String,
    }

    let s = TestStruct { val: "2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string() };

    assert!(s.validate().is_ok());
}

#[test]
fn can_validate_ip() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(ip)]
        val: String,
    }

    let s = TestStruct { val: "2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string() };
    assert!(s.validate().is_ok());

    let s = TestStruct { val: "192.168.1.1".to_string() };
    assert!(s.validate().is_ok());
}

#[test]
fn bad_ip_fails_validation() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(ip)]
        val: String,
    }

    let s = TestStruct { val: "123.123.123".to_string() };
    let res = s.validate();
    assert!(res.is_err());

    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "ip");
    assert_eq!(errs["val"][0].params["value"], "123.123.123");
}

#[test]
fn bad_ipv4_fails_validation() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(ip(v4))]
        val: String,
    }

    let s = TestStruct { val: "123.123.123".to_string() };
    let res = s.validate();
    assert!(res.is_err());

    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "ip");
    assert_eq!(errs["val"][0].params["value"], "123.123.123");
}

#[test]
fn can_specify_code_for_ip() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(ip(code = "oops"))]
        val: String,
    }

    let s = TestStruct { val: "123.123.123".to_string() };
    let res = s.validate();
    assert!(res.is_err());

    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "oops");
}

#[test]
fn can_specify_code_for_ipv4() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(ip(v4, code = "oops"))]
        val: String,
    }

    let s = TestStruct { val: "123.123.123".to_string() };
    let res = s.validate();
    assert!(res.is_err());

    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "oops");
}

#[test]
fn can_specify_message_for_ip() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(ip(message = "oops"))]
        val: String,
    }
    let s = TestStruct { val: "123.123.123".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].clone().message.unwrap(), "oops");
}

#[test]
fn can_specify_message_for_ipv6() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(ip(v6, message = "oops"))]
        val: String,
    }
    let s = TestStruct { val: "123.123.123".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].clone().message.unwrap(), "oops");
}

#[test]
fn can_validate_custom_impl_for_ip() {
    #[derive(Serialize)]
    struct CustomIp {
        a: u8,
        b: u8,
        c: u8,
        d: u8,
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(ip)]
        val: CustomIp,
    }

    impl ToString for CustomIp {
        fn to_string(&self) -> String {
            format!("{}.{}.{}.{}", self.a, self.b, self.c, self.d)
        }
    }

    let valid = TestStruct { val: CustomIp { a: 192, b: 168, c: 1, d: 1 } };
    assert!(valid.validate().is_ok());
}
