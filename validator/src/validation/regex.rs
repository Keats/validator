use std::borrow::Cow;
use std::cell::OnceCell;
use std::sync::{Arc, Mutex, OnceLock};

use regex::Regex;

pub trait IntoRegex {
    fn into_regex(&self) -> Cow<Regex>;
}

impl IntoRegex for Regex {
    fn into_regex(&self) -> Cow<Regex> {
        Cow::Borrowed(self)
    }
}

impl IntoRegex for &Regex {
    fn into_regex(&self) -> Cow<Regex> {
        Cow::Borrowed(self)
    }
}

impl IntoRegex for &OnceLock<Regex> {
    fn into_regex(&self) -> Cow<Regex> {
        Cow::Borrowed(self.get().unwrap())
    }
}

impl IntoRegex for &Mutex<OnceCell<Regex>> {
    fn into_regex(&self) -> Cow<Regex> {
        Cow::Owned(self.lock().unwrap().get().unwrap().clone())
    }
}

impl IntoRegex for &Mutex<OnceLock<Regex>> {
    fn into_regex(&self) -> Cow<Regex> {
        Cow::Owned(self.lock().unwrap().get().unwrap().clone())
    }
}

impl IntoRegex for &Arc<Mutex<OnceCell<Regex>>> {
    fn into_regex(&self) -> Cow<Regex> {
        Cow::Owned(self.lock().unwrap().get().unwrap().clone())
    }
}

impl IntoRegex for &Arc<Mutex<OnceLock<Regex>>> {
    fn into_regex(&self) -> Cow<Regex> {
        Cow::Owned(self.lock().unwrap().get().unwrap().clone())
    }
}

pub trait ValidateRegex {
    fn validate_regex(&self, regex: impl IntoRegex) -> bool;
}

impl ValidateRegex for String {
    fn validate_regex(&self, regex: impl IntoRegex) -> bool {
        regex.into_regex().is_match(self)
    }
}

impl ValidateRegex for Option<String> {
    fn validate_regex(&self, regex: impl IntoRegex) -> bool {
        if let Some(h) = self {
            regex.into_regex().is_match(h)
        } else {
            true
        }
    }
}

impl ValidateRegex for Option<Option<String>> {
    fn validate_regex(&self, regex: impl IntoRegex) -> bool {
        if let Some(h) = self {
            if let Some(h) = h {
                regex.into_regex().is_match(h)
            } else {
                true
            }
        } else {
            true
        }
    }
}

impl ValidateRegex for &String {
    fn validate_regex(&self, regex: impl IntoRegex) -> bool {
        regex.into_regex().is_match(self)
    }
}

impl ValidateRegex for Option<&String> {
    fn validate_regex(&self, regex: impl IntoRegex) -> bool {
        if let Some(h) = self {
            regex.into_regex().is_match(h)
        } else {
            true
        }
    }
}

impl ValidateRegex for Option<Option<&String>> {
    fn validate_regex(&self, regex: impl IntoRegex) -> bool {
        if let Some(h) = self {
            if let Some(h) = h {
                regex.into_regex().is_match(h)
            } else {
                true
            }
        } else {
            true
        }
    }
}

impl ValidateRegex for &str {
    fn validate_regex(&self, regex: impl IntoRegex) -> bool {
        regex.into_regex().is_match(self)
    }
}

impl ValidateRegex for Option<&str> {
    fn validate_regex(&self, regex: impl IntoRegex) -> bool {
        if let Some(h) = self {
            regex.into_regex().is_match(h)
        } else {
            true
        }
    }
}

impl ValidateRegex for Option<Option<&str>> {
    fn validate_regex(&self, regex: impl IntoRegex) -> bool {
        if let Some(h) = self {
            if let Some(h) = h {
                regex.into_regex().is_match(h)
            } else {
                true
            }
        } else {
            true
        }
    }
}

impl ValidateRegex for Cow<'_, str> {
    fn validate_regex(&self, regex: impl IntoRegex) -> bool {
        regex.into_regex().is_match(self)
    }
}

impl ValidateRegex for Option<Cow<'_, str>> {
    fn validate_regex(&self, regex: impl IntoRegex) -> bool {
        if let Some(h) = self {
            regex.into_regex().is_match(h)
        } else {
            true
        }
    }
}

impl ValidateRegex for Option<Option<Cow<'_, str>>> {
    fn validate_regex(&self, regex: impl IntoRegex) -> bool {
        if let Some(h) = self {
            if let Some(h) = h {
                regex.into_regex().is_match(h)
            } else {
                true
            }
        } else {
            true
        }
    }
}
