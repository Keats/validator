use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::BuildHasher;

pub trait ValidateContains {
    fn validate_contains(&self, needle: &str) -> bool;
}

impl ValidateContains for String {
    fn validate_contains(&self, needle: &str) -> bool {
        self.contains(needle)
    }
}

impl ValidateContains for Option<String> {
    fn validate_contains(&self, needle: &str) -> bool {
        if let Some(v) = self {
            v.contains(needle)
        } else {
            true
        }
    }
}

impl ValidateContains for Option<Option<String>> {
    fn validate_contains(&self, needle: &str) -> bool {
        if let Some(v) = self {
            if let Some(v) = v {
                v.contains(needle)
            } else {
                true
            }
        } else {
            true
        }
    }
}

impl ValidateContains for &String {
    fn validate_contains(&self, needle: &str) -> bool {
        self.contains(needle)
    }
}

impl ValidateContains for Option<&String> {
    fn validate_contains(&self, needle: &str) -> bool {
        if let Some(v) = self {
            v.contains(needle)
        } else {
            true
        }
    }
}

impl ValidateContains for Option<Option<&String>> {
    fn validate_contains(&self, needle: &str) -> bool {
        if let Some(v) = self {
            if let Some(v) = v {
                v.contains(needle)
            } else {
                true
            }
        } else {
            true
        }
    }
}

impl<'a> ValidateContains for &'a str {
    fn validate_contains(&self, needle: &str) -> bool {
        self.contains(needle)
    }
}

impl<'a> ValidateContains for Option<&'a str> {
    fn validate_contains(&self, needle: &str) -> bool {
        if let Some(v) = self {
            v.contains(needle)
        } else {
            true
        }
    }
}

impl<'a> ValidateContains for Option<Option<&'a str>> {
    fn validate_contains(&self, needle: &str) -> bool {
        if let Some(v) = self {
            if let Some(v) = v {
                v.contains(needle)
            } else {
                true
            }
        } else {
            true
        }
    }
}

impl<'a> ValidateContains for Cow<'a, str> {
    fn validate_contains(&self, needle: &str) -> bool {
        self.contains(needle)
    }
}

impl<'a> ValidateContains for Option<Cow<'a, str>> {
    fn validate_contains(&self, needle: &str) -> bool {
        if let Some(v) = self {
            v.contains(needle)
        } else {
            true
        }
    }
}

impl<'a> ValidateContains for Option<Option<Cow<'a, str>>> {
    fn validate_contains(&self, needle: &str) -> bool {
        if let Some(v) = self {
            if let Some(v) = v {
                v.contains(needle)
            } else {
                true
            }
        } else {
            true
        }
    }
}

impl<S, H: BuildHasher> ValidateContains for HashMap<String, S, H> {
    fn validate_contains(&self, needle: &str) -> bool {
        self.contains_key(needle)
    }
}

impl<S, H: BuildHasher> ValidateContains for Option<HashMap<String, S, H>> {
    fn validate_contains(&self, needle: &str) -> bool {
        if let Some(v) = self {
            v.contains_key(needle)
        } else {
            true
        }
    }
}

impl<S, H: BuildHasher> ValidateContains for Option<Option<HashMap<String, S, H>>> {
    fn validate_contains(&self, needle: &str) -> bool {
        if let Some(v) = self {
            if let Some(v) = v {
                v.contains_key(needle)
            } else {
                true
            }
        } else {
            true
        }
    }
}

impl<'a, S, H: BuildHasher> ValidateContains for &'a HashMap<String, S, H> {
    fn validate_contains(&self, needle: &str) -> bool {
        self.contains_key(needle)
    }
}

impl<'a, S, H: BuildHasher> ValidateContains for Option<&'a HashMap<String, S, H>> {
    fn validate_contains(&self, needle: &str) -> bool {
        if let Some(v) = self {
            v.contains_key(needle)
        } else {
            true
        }
    }
}

impl<'a, S, H: BuildHasher> ValidateContains for Option<Option<&'a HashMap<String, S, H>>> {
    fn validate_contains(&self, needle: &str) -> bool {
        if let Some(v) = self {
            if let Some(v) = v {
                v.contains_key(needle)
            } else {
                true
            }
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_validate_contains_string() {
        assert!("hey".validate_contains("e"));
    }

    #[test]
    fn test_validate_contains_string_can_fail() {
        assert!("hey".validate_contains("o"));
    }

    #[test]
    fn test_validate_contains_hashmap_key() {
        let mut map = HashMap::new();
        map.insert("hey".to_string(), 1);
        assert!(map.validate_contains("hey"));
    }

    #[test]
    fn test_validate_contains_hashmap_key_can_fail() {
        let mut map = HashMap::new();
        map.insert("hey".to_string(), 1);
        assert!(!map.validate_contains("bob"));
    }

    #[test]
    fn test_validate_contains_cow() {
        let test: Cow<'static, str> = "hey".into();
        assert!(test.validate_contains("e"));
        let test: Cow<'static, str> = String::from("hey").into();
        assert!(test.validate_contains("e"));
    }

    #[test]
    fn test_validate_contains_cow_can_fail() {
        let test: Cow<'static, str> = "hey".into();
        assert!(!test.validate_contains("o"));
        let test: Cow<'static, str> = String::from("hey").into();
        assert!(!test.validate_contains("o"));
    }
}
