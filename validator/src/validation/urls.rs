use std::{
    borrow::Cow,
    cell::{Ref, RefMut},
    rc::Rc,
    sync::Arc,
};
use url::Url;

/// Validates whether the string given is a url
pub trait ValidateUrl {
    fn validate_url(&self) -> bool {
        if let Some(u) = self.as_url_string() {
            Url::parse(&u).is_ok()
        } else {
            true
        }
    }

    fn as_url_string(&self) -> Option<Cow<str>>;
}

macro_rules! validate_type_that_derefs {
    ($type_:ty) => {
        impl<T> ValidateUrl for $type_
        where
            T: ValidateUrl,
        {
            fn as_url_string(&self) -> Option<Cow<str>> {
                T::as_url_string(self)
            }
        }
    };
}

validate_type_that_derefs!(&T);
validate_type_that_derefs!(Arc<T>);
validate_type_that_derefs!(Box<T>);
validate_type_that_derefs!(Rc<T>);
validate_type_that_derefs!(Ref<'_, T>);
validate_type_that_derefs!(RefMut<'_, T>);

macro_rules! validate_type_of_str {
    ($type_:ty) => {
        impl ValidateUrl for $type_ {
            fn as_url_string(&self) -> Option<Cow<str>> {
                Some(Cow::Borrowed(self))
            }
        }
    };
}

validate_type_of_str!(str);
validate_type_of_str!(&str);
validate_type_of_str!(String);

impl<T> ValidateUrl for Option<T>
where
    T: ValidateUrl,
{
    fn as_url_string(&self) -> Option<Cow<str>> {
        let Some(u) = self else {
            return None;
        };

        T::as_url_string(u)
    }
}

impl ValidateUrl for Cow<'_, str> {
    fn as_url_string(&self) -> Option<Cow<'_, str>> {
        <str as ValidateUrl>::as_url_string(self)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::ValidateUrl;

    #[test]
    fn test_validate_url() {
        let tests = vec![
            ("http", false),
            ("https://google.com", true),
            ("http://localhost:80", true),
            ("ftp://localhost:80", true),
        ];

        for (url, expected) in tests {
            assert_eq!(url.validate_url(), expected);
        }
    }

    #[test]
    fn test_validate_url_cow() {
        let test: Cow<'static, str> = "http://localhost:80".into();
        assert!(test.validate_url());
        let test: Cow<'static, str> = String::from("http://localhost:80").into();
        assert!(test.validate_url());
        let test: Cow<'static, str> = "http".into();
        assert!(!test.validate_url());
        let test: Cow<'static, str> = String::from("http").into();
        assert!(!test.validate_url());
    }
}
