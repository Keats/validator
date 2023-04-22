use proc_macro2::Span;
use proc_macro_error::abort;
use syn::spanned::Spanned;

use validator_types::{CustomArgument, Validator};

use crate::{asserts::assert_custom_arg_type, lit::*};

#[derive(Debug)]
pub struct SchemaValidation {
    pub function: String,
    pub args: Option<CustomArgument>,
    pub skip_on_field_errors: bool,
    pub code: Option<String>,
    pub message: Option<String>,
}

/// This struct holds the combined validation information for one filed
#[derive(Debug)]
pub struct FieldInformation {
    pub field: syn::Field,
    pub field_type: String,
    pub name: String,
    pub validations: Vec<FieldValidation>,
}

impl FieldInformation {
    pub fn new(
        field: syn::Field,
        field_type: String,
        name: String,
        validations: Vec<FieldValidation>,
    ) -> Self {
        FieldInformation { field, field_type, name, validations }
    }
}

/// This struct holds information about one specific validation with it's code, message and validator.
#[derive(Debug)]
pub struct FieldValidation {
    pub code: String,
    pub message: Option<String>,
    pub validator: Validator,
}

impl FieldValidation {
    pub fn new(validator: Validator) -> FieldValidation {
        FieldValidation { code: validator.code().to_string(), validator, message: None }
    }
}

pub fn extract_length_validation(
    field: String,
    attr: &syn::Attribute,
    meta_items: &[syn::NestedMeta],
) -> FieldValidation {
    let mut min = None;
    let mut max = None;
    let mut equal = None;

    let (message, code) = extract_message_and_code("length", &field, meta_items);

    let error = |span: Span, msg: &str| -> ! {
        abort!(span, "Invalid attribute #[validate] on field `{}`: {}", field, msg);
    };

    for meta_item in meta_items {
        if let syn::NestedMeta::Meta(ref item) = *meta_item {
            if let syn::Meta::NameValue(syn::MetaNameValue { ref path, ref lit, .. }) = *item {
                let ident = path.get_ident().unwrap();
                match ident.to_string().as_ref() {
                    "message" | "code" => continue,
                    "min" => {
                        min = match lit_to_u64_or_path(lit) {
                            Some(s) => Some(s),
                            None => error(lit.span(), "invalid argument type for `min` of `length` validator: only number literals or value paths are allowed"),
                        };
                    }
                    "max" => {
                        max = match lit_to_u64_or_path(lit) {
                            Some(s) => Some(s),
                            None => error(lit.span(), "invalid argument type for `max` of `length` validator: only number literals or value paths are allowed"),
                        };
                    }
                    "equal" => {
                        equal = match lit_to_u64_or_path(lit) {
                            Some(s) => Some(s),
                            None => error(lit.span(), "invalid argument type for `equal` of `length` validator: only number literals or value paths are allowed"),
                        };
                    }
                    v => error(path.span(), &format!(
                        "unknown argument `{}` for validator `length` (it only has `min`, `max`, `equal`)",
                        v
                    ))
                }
            } else {
                error(
                    item.span(),
                    &format!(
                        "unexpected item {:?} while parsing `length` validator of field {}",
                        item, field
                    ),
                )
            }
        }

        if equal.is_some() && (min.is_some() || max.is_some()) {
            error(meta_item.span(), "both `equal` and `min` or `max` have been set in `length` validator: probably a mistake");
        }
    }

    if min.is_none() && max.is_none() && equal.is_none() {
        error(
            attr.span(),
            "Validator `length` requires at least 1 argument out of `min`, `max` and `equal`",
        );
    }

    let validator = Validator::Length { min, max, equal };
    FieldValidation {
        message,
        code: code.unwrap_or_else(|| validator.code().to_string()),
        validator,
    }
}

const RANGE_MIN_KEY: &str = "min";
const RANGE_EXCLUSIVE_MIN_KEY: &str = "exclusive_min";
const RANGE_MAX_KEY: &str = "max";
const RANGE_EXCLUSIVE_MAX_KEY: &str = "exclusive_max";

pub fn extract_range_validation(
    field: String,
    attr: &syn::Attribute,
    meta_items: &[syn::NestedMeta],
) -> FieldValidation {
    let mut min = None;
    let mut max = None;
    let mut exclusive_min = None;
    let mut exclusive_max = None;

    let (message, code) = extract_message_and_code("range", &field, meta_items);

    let error = |span: Span, msg: &str| -> ! {
        abort!(span, "Invalid attribute #[validate] on field `{}`: {}", field, msg);
    };

    for meta_item in meta_items {
        match *meta_item {
            syn::NestedMeta::Meta(ref item) => match *item {
                syn::Meta::NameValue(syn::MetaNameValue { ref path, ref lit, .. }) => {
                    let ident = path.get_ident().unwrap();
                    match ident.to_string().as_ref() {
                        "message" | "code" => continue,
                        RANGE_MIN_KEY => {
                            min = match lit_to_f64_or_path(lit) {
                                Some(s) => Some(s),
                                None => error(lit.span(), &lit_to_f64_error_message(RANGE_MIN_KEY))
                            };
                        }
                        RANGE_EXCLUSIVE_MIN_KEY => {
                            exclusive_min = match lit_to_f64_or_path(lit) {
                                Some(s) => Some(s),
                                None => error(lit.span(), &lit_to_f64_error_message(RANGE_EXCLUSIVE_MIN_KEY))
                            };
                        }
                        RANGE_MAX_KEY => {
                            max = match lit_to_f64_or_path(lit) {
                                Some(s) => Some(s),
                                None => error(lit.span(), &lit_to_f64_error_message(RANGE_MAX_KEY))
                            };
                        }
                        RANGE_EXCLUSIVE_MAX_KEY => {
                            exclusive_max = match lit_to_f64_or_path(lit) {
                                Some(s) => Some(s),
                                None => error(lit.span(), &lit_to_f64_error_message(RANGE_EXCLUSIVE_MAX_KEY))
                            };
                        }
                        v => error(path.span(), &format!(
                            "unknown argument `{}` for validator `range` (it only has `min`, `max`)",
                            v
                        )),
                    }
                }
                _ => abort!(
                    item.span(),
                    "unexpected item {:?} while parsing `range` validator",
                    item
                ),
            },
            _ => unreachable!(),
        }
    }

    if [&min, &max, &exclusive_min, &exclusive_max].iter().all(|x| x.is_none()) {
        error(
            attr.span(),
            &format!(
                "Validator `range` requires at least 1 argument out of `{}`, `{}`, `{}` and `{}`",
                RANGE_MIN_KEY, RANGE_MAX_KEY, RANGE_EXCLUSIVE_MIN_KEY, RANGE_EXCLUSIVE_MAX_KEY
            ),
        );
    }

    if min.is_some() && exclusive_min.is_some() || max.is_some() && exclusive_max.is_some() {
        error(
            attr.span(),
            &format!(
                "Validator `range` cannot contain one of its limits (`{}`, `{}`) and its exclusive counterpart",
                RANGE_MIN_KEY, RANGE_MAX_KEY
            )
        )
    }
    let validator = Validator::Range { min, max, exclusive_min, exclusive_max };
    FieldValidation {
        message,
        code: code.unwrap_or_else(|| validator.code().to_string()),
        validator,
    }
}

fn lit_to_f64_error_message(val_name: &str) -> String {
    format!("invalid argument type for `{}` of `range` validator: only number literals or value paths are allowed", val_name)
}

pub fn extract_custom_validation(
    field: String,
    attr: &syn::Attribute,
    meta_items: &[syn::NestedMeta],
) -> FieldValidation {
    let mut function = None;
    let mut argument = None;

    let (message, code) = extract_message_and_code("custom", &field, meta_items);

    let error = |span: Span, msg: &str| -> ! {
        abort!(span, "Invalid attribute #[validate] on field `{}`: {}", field, msg);
    };

    for meta_item in meta_items {
        match *meta_item {
            syn::NestedMeta::Meta(ref item) => match *item {
                syn::Meta::NameValue(syn::MetaNameValue { ref path, ref lit, .. }) => {
                    let ident = path.get_ident().unwrap();
                    match ident.to_string().as_ref() {
                        "message" | "code" => continue,
                        "function" => {
                            function = match lit_to_string(lit) {
                                Some(s) => Some(s),
                                None => error(lit.span(), "invalid argument type for `function` of `custom` validator: expected a string")
                            };
                        }
                        "arg" => {
                            match lit_to_string(lit) {
                                Some(s) => {
                                    match syn::parse_str::<syn::Type>(s.as_str()) {
                                        Ok(arg_type) => {
                                            assert_custom_arg_type(&lit.span(), &arg_type);
                                            argument = Some(CustomArgument::new(lit.span(), arg_type));
                                        }
                                        Err(_) => {
                                            let mut msg = "invalid argument type for `arg` of `custom` validator: The string has to be a single type.".to_string();
                                            msg.push_str("\n(Tip: You can combine multiple types into one tuple.)");

                                            error(lit.span(), msg.as_str());
                                        }
                                    }
                                }
                                None => error(lit.span(), "invalid argument type for `arg` of `custom` validator: expected a string")
                            };
                        }
                        v => error(path.span(), &format!(
                            "unknown argument `{}` for validator `custom` (it only has `function`, `arg`)",
                            v
                        )),
                    }
                }
                _ => abort!(
                    item.span(),
                    "unexpected item {:?} while parsing `custom` validator",
                    item
                ),
            },
            _ => unreachable!(),
        }
    }

    if function.is_none() {
        error(attr.span(), "The validator `custom` requires the `function` parameter.");
    }

    let validator = Validator::Custom { function: function.unwrap(), argument: Box::new(argument) };
    FieldValidation {
        message,
        code: code.unwrap_or_else(|| validator.code().to_string()),
        validator,
    }
}

/// Extract url/email/phone/non_control_character field validation with a code or a message
pub fn extract_argless_validation(
    validator_name: String,
    field: String,
    meta_items: &[syn::NestedMeta],
) -> FieldValidation {
    let (message, code) = extract_message_and_code(&validator_name, &field, meta_items);

    for meta_item in meta_items {
        match *meta_item {
            syn::NestedMeta::Meta(ref item) => match *item {
                syn::Meta::NameValue(syn::MetaNameValue { ref path, .. }) => {
                    let ident = path.get_ident().unwrap();
                    match ident.to_string().as_ref() {
                        "message" | "code" => continue,
                        v => abort!(
                            meta_item.span(),
                            "Unknown argument `{}` for validator `{}` on field `{}`",
                            v,
                            validator_name,
                            field
                        ),
                    }
                }
                _ => abort!(
                    meta_item.span(),
                    "unexpected item {:?} while parsing `range` validator",
                    item
                ),
            },
            _ => unreachable!(),
        }
    }

    let validator = match validator_name.as_ref() {
        "email" => Validator::Email,
        #[cfg(feature = "card")]
        "credit_card" => Validator::CreditCard,
        #[cfg(feature = "phone")]
        "phone" => Validator::Phone,
        #[cfg(feature = "unic")]
        "non_control_character" => Validator::NonControlCharacter,
        "required" => Validator::Required,
        _ => Validator::Url,
    };

    FieldValidation {
        message,
        code: code.unwrap_or_else(|| validator.code().to_string()),
        validator,
    }
}

/// For custom, contains, regex, must_match
pub fn extract_one_arg_validation(
    val_name: &str,
    validator_name: String,
    field: String,
    meta_items: &[syn::NestedMeta],
) -> FieldValidation {
    let mut value = None;
    let (message, code) = extract_message_and_code(&validator_name, &field, meta_items);

    for meta_item in meta_items {
        match *meta_item {
            syn::NestedMeta::Meta(ref item) => match *item {
                syn::Meta::NameValue(syn::MetaNameValue { ref path, ref lit, .. }) => {
                    let ident = path.get_ident().unwrap();
                    match ident.to_string().as_ref() {
                        "message" | "code" => continue,
                        v if v == val_name => {
                            value = match lit_to_string(lit) {
                                Some(s) => Some(s),
                                None => abort!(
                                    item.span(),
                                    "Invalid argument type for `{}` for validator `{}` on field `{}`: only a string is allowed",
                                    val_name, validator_name, field
                                ),
                            };
                        }
                        v => abort!(
                            path.span(),
                            "Unknown argument `{}` for validator `{}` on field `{}`",
                            v,
                            validator_name,
                            field
                        ),
                    }
                }
                _ => abort!(
                    item.span(),
                    "unexpected item {:?} while parsing `range` validator",
                    item
                ),
            },
            _ => unreachable!(),
        }

        if value.is_none() {
            abort!(
                meta_item.span(),
                "Missing argument `{}` for validator `{}` on field `{}`",
                val_name,
                validator_name,
                field
            );
        }
    }

    let validator = match validator_name.as_ref() {
        "custom" => Validator::Custom { function: value.unwrap(), argument: Box::new(None) },
        "contains" => Validator::Contains(value.unwrap()),
        "does_not_contain" => Validator::DoesNotContain(value.unwrap()),
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

fn extract_message_and_code(
    validator_name: &str,
    field: &str,
    meta_items: &[syn::NestedMeta],
) -> (Option<String>, Option<String>) {
    let mut message = None;
    let mut code = None;

    for meta_item in meta_items {
        if let syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue {
            ref path,
            ref lit,
            ..
        })) = *meta_item
        {
            let ident = path.get_ident().unwrap();
            match ident.to_string().as_ref() {
                "code" => {
                    code = match lit_to_string(lit) {
                        Some(s) => Some(s),
                        None => abort!(
                                    meta_item.span(),
                                    "Invalid argument type for `code` for validator `{}` on field `{}`: only a string is allowed",
                                    validator_name, field
                                ),
                    };
                }
                "message" => {
                    message = match lit_to_string(lit) {
                        Some(s) => Some(s),
                        None => abort!(
                                    meta_item.span(),
                                    "Invalid argument type for `message` for validator `{}` on field `{}`: only a string is allowed",
                                    validator_name, field
                                ),
                    };
                }
                _ => continue,
            }
        }
    }

    (message, code)
}
