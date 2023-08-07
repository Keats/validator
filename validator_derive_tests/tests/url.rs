use validator::Validate;

#[test]
fn can_validate_url_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(url)]
        val: String,
    }

    let s = TestStruct { val: "https://google.com".to_string() };

    assert!(s.validate().is_ok());
}

#[test]
fn bad_url_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(url)]
        val: String,
    }

    let s = TestStruct { val: "bob".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();

    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "url");
}

#[test]
fn can_specify_code_for_url() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(url(code = "oops"))]
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
    assert_eq!(errs["val"][0].params["value"], "bob");
}

#[test]
fn can_specify_message_for_url() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(url(message = "oops"))]
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
fn can_validate_custom_impl_for_url() {
    use serde::Serialize;
    use std::borrow::Cow;

    #[derive(Debug, Serialize)]
    struct CustomUrl {
        scheme: String,
        subdomain: String,
        domain: String,
        top_level_domain: String,
    }

    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(url)]
        val: CustomUrl,
    }

    impl validator::ValidateUrl for CustomUrl {
        fn to_url_string(&self) -> Option<Cow<'_, str>> {
            Some(Cow::from(format!(
                "{}://{}.{}.{}",
                self.scheme, self.subdomain, self.domain, self.top_level_domain
            )))
        }
    }

    let valid = TestStruct {
        val: CustomUrl {
            scheme: "http".to_string(),
            subdomain: "www".to_string(),
            domain: "google".to_string(),
            top_level_domain: "com".to_string(),
        },
    };

    let invalid = TestStruct {
        val: CustomUrl {
            scheme: "".to_string(),
            subdomain: "".to_string(),
            domain: "google".to_string(),
            top_level_domain: "".to_string(),
        },
    };

    assert!(valid.validate().is_ok());
    assert!(invalid.validate().is_err());
}
