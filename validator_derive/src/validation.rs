use syn;

use validator::Validator;

use lit::*;


#[derive(Debug)]
pub struct SchemaValidation {
    pub function: String,
    pub skip_on_field_errors: bool,
    pub code: Option<String>,
    pub message: Option<String>,
}


#[derive(Debug)]
pub struct FieldValidation {
    pub code: String,
    pub message: Option<String>,
    pub validator: Validator,
}

impl FieldValidation {
    pub fn new(validator: Validator) -> FieldValidation {
        FieldValidation {
            code: validator.code().to_string(),
            validator,
            message: None,
        }
    }
}

pub fn extract_length_validation(field: String, meta_items: &Vec<syn::NestedMetaItem>) -> FieldValidation {
    let mut min = None;
    let mut max = None;
    let mut equal = None;

    let mut code = None;
    let mut message = None;

    let error = |msg: &str| -> ! {
        panic!("Invalid attribute #[validate] on field `{}`: {}", field, msg);
    };

    for meta_item in meta_items {
        if let syn::NestedMetaItem::MetaItem(ref item) = *meta_item {
            if let syn::MetaItem::NameValue(ref name, ref val) = *item {
                match name.to_string().as_ref() {
                    "min" => {
                        min = match lit_to_int(val) {
                            Some(s) => Some(s),
                            None => error("invalid argument type for `min` of `length` validator: only integers are allowed"),
                        };
                    },
                    "max" => {
                        max = match lit_to_int(val) {
                            Some(s) => Some(s),
                            None => error("invalid argument type for `max` of `length` validator: only integers are allowed"),
                        };
                    },
                    "equal" => {
                        equal = match lit_to_int(val) {
                            Some(s) => Some(s),
                            None => error("invalid argument type for `equal` of `length` validator: only integers are allowed"),
                        };
                    },
                    "code" => {
                        code = match lit_to_string(val) {
                            Some(s) => Some(s),
                            None => error("invalid argument type for `code` of `length` validator: only a string is allowed"),
                        };
                    },
                    "message" => {
                        message = match lit_to_string(val) {
                            Some(s) => Some(s),
                            None => error("invalid argument type for `message` of `length` validator: only a string is allowed"),
                        };
                    },
                    _ => error(&format!(
                        "unknown argument `{}` for validator `length` (it only has `min`, `max`, `equal`)",
                        name.to_string()
                    ))
                }
            } else {
                panic!("unexpected item {:?} while parsing `length` validator of field {}", item, field)
            }
        }
    }

    if equal.is_some() && (min.is_some() || max.is_some()) {
        error("both `equal` and `min` or `max` have been set in `length` validator: probably a mistake");
    }
    if min.is_none() && max.is_none() && equal.is_none() {
        error("Validator `length` requires at least 1 argument out of `min`, `max` and `equal`");
    }

    let validator = Validator::Length { min, max, equal };
    FieldValidation {
        message,
        code: code.unwrap_or_else(|| validator.code().to_string()),
        validator,
    }
}

pub fn extract_range_validation(field: String, meta_items: &Vec<syn::NestedMetaItem>) -> FieldValidation {
    let mut min = 0.0;
    let mut max = 0.0;

    let mut code = None;
    let mut message = None;

    let error = |msg: &str| -> ! {
        panic!("Invalid attribute #[validate] on field `{}`: {}", field, msg);
    };

    // whether it has both `min` and `max`
    let mut has_min = false;
    let mut has_max = false;

    for meta_item in meta_items {
        match *meta_item {
            syn::NestedMetaItem::MetaItem(ref item) => match *item {
                syn::MetaItem::NameValue(ref name, ref val) => {
                    match name.to_string().as_ref() {
                        "min" => {
                            min = match lit_to_float(val) {
                                Some(s) => s,
                                None => error("invalid argument type for `min` of `range` validator: only integers are allowed")
                            };
                            has_min = true;
                        },
                        "max" => {
                            max = match lit_to_float(val) {
                                Some(s) => s,
                                None => error("invalid argument type for `max` of `range` validator: only integers are allowed")
                            };
                            has_max = true;
                        },
                        "code" => {
                            code = match lit_to_string(val) {
                                Some(s) => Some(s),
                                None => error("invalid argument type for `code` of `length` validator: only a string is allowed"),
                            };
                        },
                        "message" => {
                            message = match lit_to_string(val) {
                                Some(s) => Some(s),
                                None => error("invalid argument type for `message` of `length` validator: only a string is allowed"),
                            };
                        },
                        _ => error(&format!(
                            "unknown argument `{}` for validator `range` (it only has `min`, `max`)",
                            name.to_string()
                        ))
                    }
                },
                _ => panic!("unexpected item {:?} while parsing `range` validator", item)
            },
            _=> unreachable!()
        }
    }

    if !has_min || !has_max {
        error("Validator `range` requires 2 arguments: `min` and `max`");
    }

    let validator = Validator::Range { min, max };
    FieldValidation {
        message,
        code: code.unwrap_or_else(|| validator.code().to_string()),
        validator,
    }
}

/// Extract url/email/phone field validation with a code or a message
pub fn extract_argless_validation(validator_name: String, field: String, meta_items: &Vec<syn::NestedMetaItem>) -> FieldValidation {
    let mut code = None;
    let mut message = None;

    for meta_item in meta_items {
        match *meta_item {
            syn::NestedMetaItem::MetaItem(ref item) => match *item {
                syn::MetaItem::NameValue(ref name, ref val) => {
                    match name.to_string().as_ref() {
                        "code" => {
                            code = match lit_to_string(val) {
                                Some(s) => Some(s),
                                None => panic!(
                                    "Invalid argument type for `code` for validator `{}` on field `{}`: only a string is allowed",
                                    validator_name, field
                                ),
                            };
                        },
                        "message" => {
                            message = match lit_to_string(val) {
                                Some(s) => Some(s),
                                None => panic!(
                                    "Invalid argument type for `message` for validator `{}` on field `{}`: only a string is allowed",
                                    validator_name, field
                                ),
                            };
                        },
                        _ => panic!(
                            "Unknown argument `{}` for validator `{}` on field `{}`",
                            name.to_string(), validator_name, field
                        )
                    }
                },
                _ => panic!("unexpected item {:?} while parsing `range` validator", item)
            },
            _=> unreachable!()
        }
    }

    let validator = match validator_name.as_ref() {
        "email" => Validator::Email,
        "phone" => Validator::Phone,
        _ => Validator::Url
    };

    FieldValidation {
        message,
        code: code.unwrap_or_else(|| validator.code().to_string()),
        validator,
    }
}

/// For custom, contains, regex, must_match
pub fn extract_one_arg_validation(val_name: &str, validator_name: String, field: String, meta_items: &Vec<syn::NestedMetaItem>) -> FieldValidation {
    let mut code = None;
    let mut message = None;
    let mut value = None;

    for meta_item in meta_items {
        match *meta_item {
            syn::NestedMetaItem::MetaItem(ref item) => match *item {
                syn::MetaItem::NameValue(ref name, ref val) => {
                    match name.to_string().as_ref() {
                        v if v == val_name => {
                            value = match lit_to_string(val) {
                                Some(s) => Some(s),
                                None => panic!(
                                    "Invalid argument type for `{}` for validator `{}` on field `{}`: only a string is allowed",
                                    val_name, validator_name, field
                                ),
                            };
                        },
                        "code" => {
                            code = match lit_to_string(val) {
                                Some(s) => Some(s),
                                None => panic!(
                                    "Invalid argument type for `code` for validator `{}` on field `{}`: only a string is allowed",
                                    validator_name, field
                                ),
                            };
                        },
                        "message" => {
                            message = match lit_to_string(val) {
                                Some(s) => Some(s),
                                None => panic!(
                                    "Invalid argument type for `message` for validator `{}` on field `{}`: only a string is allowed",
                                    validator_name, field
                                ),
                            };
                        },
                        _ => panic!(
                            "Unknown argument `{}` for validator `{}` on field `{}`",
                            name.to_string(), validator_name, field
                        )
                    }
                },
                _ => panic!("unexpected item {:?} while parsing `range` validator", item)
            },
            _=> unreachable!()
        }
    }

    if value.is_none() {
        panic!("Missing argument `{}` for validator `{}` on field `{}`", val_name, validator_name, field);
    }

    let validator = match validator_name.as_ref() {
        "custom" => Validator::Custom(value.unwrap()),
        "contains" => Validator::Contains(value.unwrap()),
        "must_match" => Validator::MustMatch(value.unwrap()),
        "regex" => Validator::Regex(value.unwrap()),
        _ => unreachable!(),
    };
    FieldValidation {
        message,
        code: code.unwrap_or_else(|| validator.code().to_string()),
        validator,
    }
}
