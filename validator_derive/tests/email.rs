#[macro_use]
extern crate validator_derive;
extern crate validator;

use validator::Validate;


#[test]
fn can_validate_valid_email() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(email)]
        val: String,
    }

    let s = TestStruct {
        val: "bob@bob.com".to_string(),
    };

    assert!(s.validate().is_ok());
}

#[test]
fn bad_email_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(email)]
        val: String,
    }

    let s = TestStruct {
        val: "bob".to_string(),
    };
    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
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
    let s = TestStruct {
        val: "bob".to_string(),
    };
    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
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
    let s = TestStruct {
        val: "bob".to_string(),
    };
    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].clone().message.unwrap(), "oops");
}
