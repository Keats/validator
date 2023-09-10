use std::borrow::Cow;
use std::cell::OnceCell;
use std::sync::{Arc, Mutex, OnceLock};

use regex::Regex;

pub trait AsRegex {
    fn as_regex(&self) -> Cow<Regex>;
}

impl AsRegex for Regex {
    fn as_regex(&self) -> Cow<Regex> {
        Cow::Borrowed(self)
    }
}

impl AsRegex for &Regex {
    fn as_regex(&self) -> Cow<Regex> {
        Cow::Borrowed(self)
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

pub trait ValidateRegex {
    fn validate_regex(&self, regex: impl AsRegex) -> bool;
}

impl ValidateRegex for String {
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        regex.as_regex().is_match(self)
    }
}

impl ValidateRegex for Option<String> {
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        if let Some(h) = self {
            regex.as_regex().is_match(h)
        } else {
            true
        }
    }
}

impl ValidateRegex for Option<Option<String>> {
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        if let Some(h) = self {
            if let Some(h) = h {
                regex.as_regex().is_match(h)
            } else {
                true
            }
        } else {
            true
        }
    }
}

impl ValidateRegex for &String {
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        regex.as_regex().is_match(self)
    }
}

impl ValidateRegex for Option<&String> {
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        if let Some(h) = self {
            regex.as_regex().is_match(h)
        } else {
            true
        }
    }
}

impl ValidateRegex for Option<Option<&String>> {
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        if let Some(h) = self {
            if let Some(h) = h {
                regex.as_regex().is_match(h)
            } else {
                true
            }
        } else {
            true
        }
    }
}

impl ValidateRegex for &str {
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        regex.as_regex().is_match(self)
    }
}

impl ValidateRegex for Option<&str> {
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        if let Some(h) = self {
            regex.as_regex().is_match(h)
        } else {
            true
        }
    }
}

impl ValidateRegex for Option<Option<&str>> {
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        if let Some(h) = self {
            if let Some(h) = h {
                regex.as_regex().is_match(h)
            } else {
                true
            }
        } else {
            true
        }
    }
}

impl ValidateRegex for Cow<'_, str> {
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        regex.as_regex().is_match(self)
    }
}

impl ValidateRegex for Option<Cow<'_, str>> {
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        if let Some(h) = self {
            regex.as_regex().is_match(h)
        } else {
            true
        }
    }
}

impl ValidateRegex for Option<Option<Cow<'_, str>>> {
    fn validate_regex(&self, regex: impl AsRegex) -> bool {
        if let Some(h) = self {
            if let Some(h) = h {
                regex.as_regex().is_match(h)
            } else {
                true
            }
        } else {
            true
        }
    }
}
