use serde::Serialize;
use validator::Validate;

#[test]
fn can_validate_utf8_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(non_control_character)]
        val: String,
    }

    let s = TestStruct { val: "하늘".to_string() };

    assert!(s.validate().is_ok());
}

#[test]
fn utf8_with_control_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(non_control_character)]
        val: String,
    }

    let s = TestStruct { val: "\u{009F}하늘".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "non_control_character");
}

#[test]
fn can_specify_code_for_non_control_character() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(non_control_character(code = "oops"))]
        val: String,
    }
    let s = TestStruct { val: "\u{009F}하늘".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "oops");
    assert_eq!(errs["val"][0].params["value"], "\u{9F}하늘");
}

#[test]
fn can_specify_message_for_non_control_character() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(non_control_character(message = "oops"))]
        val: String,
    }
    let s = TestStruct { val: "\u{009F}하늘".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].clone().message.unwrap(), "oops");
}

#[test]
fn can_validate_custom_impl_for_non_control_character() {
    #[derive(Debug, Serialize)]
    struct CustomNonControlCharacter {
        char: char,
    }

    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(non_control_character)]
        val: CustomNonControlCharacter,
    }

    impl validator::ValidateNonControlCharacter for CustomNonControlCharacter {
        fn as_non_control_character_iterator(&self) -> Box<dyn Iterator<Item = char> + '_> {
            Box::new(std::iter::once(self.char))
        }
    }

    let valid = TestStruct { val: CustomNonControlCharacter { char: 'a' } };

    let invalid = TestStruct { val: CustomNonControlCharacter { char: '\u{000c}' } };

    assert!(valid.validate().is_ok());
    assert!(invalid.validate().is_err());
}
