use std::borrow::Cow;
use std::cell::OnceCell;
use std::rc::Rc;
use std::sync::{Arc, LazyLock, Mutex, OnceLock};

use regex::Regex;

pub trait AsRegex {
    fn as_regex(&self) -> Cow<Regex>;
}

impl AsRegex for Regex {
    fn as_regex(&self) -> Cow<Regex> {
        Cow::Borrowed(self)
    }
}

impl<T> AsRegex for &T
where
    T: AsRegex,
{
    fn as_regex(&self) -> Cow<Regex> {
        T::as_regex(self)
    }
}

impl AsRegex for &OnceLock<Regex> {
    fn as_regex(&self) -> Cow<Regex> {
        Cow::Borrowed(self.get().unwrap())
    }
}

impl AsRegex for &Mutex<OnceCell<Regex>> {
    fn as_regex(&self) -> Cow<Regex> {
        Cow::Owned(self.lock().unwrap().get().unwrap().clone())
    }
}

impl AsRegex for &Mutex<OnceLock<Regex>> {
    fn as_regex(&self) -> Cow<Regex> {
        Cow::Owned(self.lock().unwrap().get().unwrap().clone())
    }
}

impl AsRegex for &Arc<Mutex<OnceCell<Regex>>> {
    fn as_regex(&self) -> Cow<Regex> {
        Cow::Owned(self.lock().unwrap().get().unwrap().clone())
    }
}

impl AsRegex for &Arc<Mutex<OnceLock<Regex>>> {
    fn as_regex(&self) -> Cow<Regex> {
        Cow::Owned(self.lock().unwrap().get().unwrap().clone())
    }
}

impl AsRegex for LazyLock<Regex> {
    fn as_regex(&self) -> Cow<Regex> {
        Cow::Borrowed(self)
    }
}

pub trait ValidateRegex {
    fn validate_regex(&self, regex: impl AsRegex) -> bool;
}

impl<T> ValidateRegex for &T
where
    T: ValidateRegex,
{
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        T::validate_regex(self, regex)
    }
}

impl<T> ValidateRegex for Option<T>
where
    T: ValidateRegex,
{
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        if let Some(h) = self {
            T::validate_regex(h, regex)
        } else {
            true
        }
    }
}

impl<'cow, T> ValidateRegex for Cow<'cow, T>
where
    T: ToOwned + ?Sized,
    for<'a> &'a T: ValidateRegex,
{
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        self.as_ref().validate_regex(regex)
    }
}

impl ValidateRegex for String {
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        regex.as_regex().is_match(self)
    }
}

impl ValidateRegex for &str {
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        regex.as_regex().is_match(self)
    }
}

impl ValidateRegex for str {
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        regex.as_regex().is_match(self)
    }
}

impl<T: ValidateRegex> ValidateRegex for Box<T> {
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        self.as_ref().validate_regex(regex)
    }
}

impl<T: ValidateRegex> ValidateRegex for Rc<T> {
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        self.as_ref().validate_regex(regex)
    }
}

impl<T: ValidateRegex> ValidateRegex for Arc<T> {
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        self.as_ref().validate_regex(regex)
    }
}
