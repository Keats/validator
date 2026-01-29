// Adapted from serde's code:
// https://github.com/serde-rs/serde/blob/d1790205/serde_derive/src/internals/case.rs

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum RenameRule {
    #[default]
    None,
    LowerCase,
    UpperCase,
    PascalCase,
    CamelCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    ScreamingKebabCase,
}

impl RenameRule {
    pub fn parse(s: syn::LitStr) -> Option<Self> {
        match &*s.value() {
            "lowercase" => Some(Self::LowerCase),
            "UPPERCASE" => Some(Self::UpperCase),
            "PascalCase" => Some(Self::PascalCase),
            "camelCase" => Some(Self::CamelCase),
            "snake_case" => Some(Self::SnakeCase),
            "SCREAMING_SNAKE_CASE" => Some(Self::ScreamingSnakeCase),
            "kebab-case" => Some(Self::KebabCase),
            "SCREAMING-KEBAB-CASE" => Some(Self::ScreamingKebabCase),
            _ => None,
        }
    }

    pub fn apply(&self, orig: &str) -> String {
        match self {
            Self::None | Self::LowerCase | Self::SnakeCase => orig.to_owned(),
            Self::ScreamingSnakeCase => orig.to_ascii_uppercase(),
            Self::UpperCase => orig.to_ascii_lowercase(),
            Self::PascalCase => pascal_case(orig, true),
            Self::CamelCase => pascal_case(orig, false),
            Self::KebabCase => orig.replace("_", "-"),
            Self::ScreamingKebabCase => orig.to_ascii_uppercase().replace("_", "-"),
        }
    }

    pub fn is_some(&self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Converts a snake_case field name into PascalCase or camelCase.
/// `cap_start` specifies if the first character is capitalized (as in Pascal).
fn pascal_case(orig: &str, cap_start: bool) -> String {
    let mut pascal = String::new();
    let mut capitalize = cap_start;
    for ch in orig.chars() {
        if ch == '_' {
            capitalize = true;
        } else if capitalize {
            pascal.push(ch.to_ascii_uppercase());
            capitalize = false;
        } else {
            pascal.push(ch);
        }
    }
    pascal
}
