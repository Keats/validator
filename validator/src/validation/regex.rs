use std::borrow::Cow;

use regex::Regex;

pub trait ValidateRegex {
    fn validate_regex(&self, regex: &Regex) -> bool;
}

impl ValidateRegex for String {
    fn validate_regex(&self, regex: &Regex) -> bool {
        regex.is_match(self)
    }
}

impl ValidateRegex for Option<String> {
    fn validate_regex(&self, regex: &Regex) -> bool {
        if let Some(h) = self {
            regex.is_match(h)
        } else {
            true
        }
    }
}

impl ValidateRegex for Option<Option<String>> {
    fn validate_regex(&self, regex: &Regex) -> bool {
        if let Some(h) = self {
            if let Some(h) = h {
                regex.is_match(h)
            } else {
                true
            }
        } else {
            true
        }
    }
}

impl ValidateRegex for &String {
    fn validate_regex(&self, regex: &Regex) -> bool {
        regex.is_match(self)
    }
}

impl ValidateRegex for Option<&String> {
    fn validate_regex(&self, regex: &Regex) -> bool {
        if let Some(h) = self {
            regex.is_match(h)
        } else {
            true
        }
    }
}

impl ValidateRegex for Option<Option<&String>> {
    fn validate_regex(&self, regex: &Regex) -> bool {
        if let Some(h) = self {
            if let Some(h) = h {
                regex.is_match(h)
            } else {
                true
            }
        } else {
            true
        }
    }
}

impl ValidateRegex for &str {
    fn validate_regex(&self, regex: &Regex) -> bool {
        regex.is_match(self)
    }
}

impl ValidateRegex for Option<&str> {
    fn validate_regex(&self, regex: &Regex) -> bool {
        if let Some(h) = self {
            regex.is_match(h)
        } else {
            true
        }
    }
}

impl ValidateRegex for Option<Option<&str>> {
    fn validate_regex(&self, regex: &Regex) -> bool {
        if let Some(h) = self {
            if let Some(h) = h {
                regex.is_match(h)
            } else {
                true
            }
        } else {
            true
        }
    }
}

impl ValidateRegex for Cow<'_, str> {
    fn validate_regex(&self, regex: &Regex) -> bool {
        regex.is_match(self)
    }
}

impl ValidateRegex for Option<Cow<'_, str>> {
    fn validate_regex(&self, regex: &Regex) -> bool {
        if let Some(h) = self {
            regex.is_match(h)
        } else {
            true
        }
    }
}

impl ValidateRegex for Option<Option<Cow<'_, str>>> {
    fn validate_regex(&self, regex: &Regex) -> bool {
        if let Some(h) = self {
            if let Some(h) = h {
                regex.is_match(h)
            } else {
                true
            }
        } else {
            true
        }
    }
}
