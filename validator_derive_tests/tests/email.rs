use serde::Serialize;
use validator::Validate;

#[test]
fn can_validate_valid_email() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(email)]
        val: String,
    }

    let s = TestStruct { val: "bob@bob.com".to_string() };

    assert!(s.validate().is_ok());
}

#[test]
fn bad_email_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(email)]
        val: String,
    }

    let s = TestStruct { val: "bob".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "email");
    assert_eq!(errs["val"][0].params["value"], "bob");
}

#[test]
fn can_specify_code_for_email() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(email(code = "oops"))]
        val: String,
    }
    let s = TestStruct { val: "bob".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "oops");
}

#[test]
fn can_specify_message_for_email() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(email(message = "oops"))]
        val: String,
    }
    let s = TestStruct { val: "bob".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].clone().message.unwrap(), "oops");
}

#[test]
fn can_validate_custom_impl_for_email() {
    use std::borrow::Cow;

    #[derive(Debug, Serialize)]
    struct CustomEmail {
        user_part: String,
        domain_part: String,
    }

    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(email)]
        val: CustomEmail,
    }

    impl validator::ValidateEmail for CustomEmail {
        fn to_email_string(&self) -> Cow<'_, str> {
            Cow::from(format!("{}@{}", self.user_part, self.domain_part))
        }
    }

    let valid = TestStruct {
        val: CustomEmail { user_part: "username".to_string(), domain_part: "gmail.com".to_owned() },
    };

    let invalid = TestStruct {
        val: CustomEmail { user_part: "abc".to_string(), domain_part: "".to_owned() },
    };

    assert!(valid.validate().is_ok());
    assert!(invalid.validate().is_err());
}
