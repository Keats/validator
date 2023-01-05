use if_chain::if_chain;
use proc_macro2::{self, Span};
use quote::quote;

use validator_types::{Validator, ValueOrPath};

use crate::asserts::{COW_TYPE, NUMBER_TYPES};
use crate::lit::{option_to_tokens, value_or_path_to_tokens};
use crate::validation::{FieldValidation, SchemaValidation};

/// Pass around all the information needed for creating a validation
#[derive(Debug)]
pub struct FieldQuoter {
    field: syn::Field,
    /// The field name
    name: String,
    /// The field type
    _type: String,
}

impl FieldQuoter {
    pub fn new(field: syn::Field, name: String, _type: String) -> FieldQuoter {
        FieldQuoter { field, name, _type }
    }

    /// Don't put a & in front a pointer since we are going to pass
    /// a reference to the validator
    /// Also just use the ident without if it's optional and will go through
    /// a if let first
    pub fn quote_validator_param(&self) -> proc_macro2::TokenStream {
        let ident = self.field.ident.as_ref().unwrap();

        if self._type.starts_with("Option<") {
            quote!(#ident)
        } else if COW_TYPE.is_match(self._type.as_ref()) {
            quote!(self.#ident.as_ref())
        } else if self._type.starts_with('&') || NUMBER_TYPES.contains(&self._type.as_ref()) {
            quote!(self.#ident)
        } else {
            quote!(&self.#ident)
        }
    }

    pub fn quote_validator_field(&self) -> proc_macro2::TokenStream {
        let ident = self.field.ident.as_ref().unwrap();

        if self._type.starts_with("Option<") || is_list(&self._type) || is_map(&self._type) {
            quote!(#ident)
        } else if COW_TYPE.is_match(self._type.as_ref()) {
            quote!(self.#ident.as_ref())
        } else {
            quote!(self.#ident)
        }
    }

    pub fn get_optional_validator_param(&self) -> proc_macro2::TokenStream {
        let ident = self.field.ident.as_ref().unwrap();
        if self._type.starts_with("Option<&")
            || self._type.starts_with("Option<Option<&")
            || NUMBER_TYPES.contains(&self._type.as_ref())
        {
            quote!(#ident)
        } else {
            quote!(ref #ident)
        }
    }

    /// Wrap the quoted output of a validation with a if let Some if
    /// the field type is an option
    pub fn wrap_if_option(&self, tokens: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let field_ident = self.field.ident.as_ref().unwrap();
        let optional_pattern_matched = self.get_optional_validator_param();
        if self._type.starts_with("Option<Option<") {
            return quote!(
                if let Some(Some(#optional_pattern_matched)) = self.#field_ident {
                    #tokens
                }
            );
        } else if self._type.starts_with("Option<") {
            return quote!(
                if let Some(#optional_pattern_matched) = self.#field_ident {
                    #tokens
                }
            );
        }

        tokens
    }

    /// Wrap the quoted output of a validation with a for loop if
    /// the field type is a vector
    pub fn wrap_if_collection(&self, tokens: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let field_ident = self.field.ident.as_ref().unwrap();
        let field_name = &self.name;

        // When we're using an option, we'll have the field unwrapped, so we should not access it
        // through `self`.
        let prefix = (!self._type.starts_with("Option<")).then(|| quote! { self. });

        // When iterating over a list, the iterator has Item=T, while a map yields Item=(K, V), and
        // we're only interested in V.
        let args = if is_list(&self._type) {
            quote! { #field_ident }
        } else if is_map(&self._type) {
            quote! { (_, #field_ident) }
        } else {
            return tokens;
        };

        quote! {
            if !::validator::ValidationErrors::has_error(&result, #field_name) {
                let results: Vec<_> = #prefix #field_ident.iter().map(|#args| {
                    let mut result = ::std::result::Result::Ok(());
                    #tokens
                    result
                }).collect();
                result = ::validator::ValidationErrors::merge_all(result, #field_name, results);
            }
        }
    }
}

fn is_map(_type: &str) -> bool {
    if let Some(stripped) = _type.strip_prefix("Option<") {
        is_map(stripped)
    } else if let Some(stripped) = _type.strip_prefix("&") {
        is_map(stripped)
    } else {
        _type.starts_with("HashMap<")
            || _type.starts_with("FxHashMap<")
            || _type.starts_with("FnvHashMap<")
            || _type.starts_with("BTreeMap<")
            || _type.starts_with("IndexMap<")
    }
}

fn is_list(_type: &str) -> bool {
    if let Some(stripped) = _type.strip_prefix("&") {
        is_list(stripped)
    } else if let Some(stripped) = _type.strip_prefix("Option<") {
        is_list(stripped)
    } else {
        _type.starts_with("Vec<")
            || _type.starts_with("HashSet<")
            || _type.starts_with("BTreeSet<")
            || _type.starts_with("IndexSet<")
            || _type.starts_with("[")
    }
}

/// Quote an actual end-user error creation automatically
fn quote_error(validation: &FieldValidation) -> proc_macro2::TokenStream {
    let code = &validation.code;
    let add_message_quoted = if let Some(ref m) = validation.message {
        quote!(err.message = Some(::std::borrow::Cow::from(#m));)
    } else {
        quote!()
    };

    quote!(
        let mut err = ::validator::ValidationError::new(#code);
        #add_message_quoted
    )
}

pub fn quote_length_validation(
    field_quoter: &FieldQuoter,
    validation: &FieldValidation,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let field_name = &field_quoter.name;
    let validator_param = field_quoter.quote_validator_param();

    if let Validator::Length { min, max, equal } = &validation.validator {
        let min_err_param_quoted = if let Some(v) = min {
            let v = value_or_path_to_tokens(v);
            quote!(err.add_param(::std::borrow::Cow::from("min"), &#v);)
        } else {
            quote!()
        };
        let max_err_param_quoted = if let Some(v) = max {
            let v = value_or_path_to_tokens(v);
            quote!(err.add_param(::std::borrow::Cow::from("max"), &#v);)
        } else {
            quote!()
        };
        let equal_err_param_quoted = if let Some(v) = equal {
            let v = value_or_path_to_tokens(v);
            quote!(err.add_param(::std::borrow::Cow::from("equal"), &#v);)
        } else {
            quote!()
        };

        let min_tokens = option_to_tokens(
            &min.clone().as_ref().map(value_or_path_to_tokens).map(|x| quote!(#x as u64)),
        );
        let max_tokens = option_to_tokens(
            &max.clone().as_ref().map(value_or_path_to_tokens).map(|x| quote!(#x as u64)),
        );
        let (option_equal, equal_tokens) = {
            let option_token =
                equal.clone().as_ref().map(value_or_path_to_tokens).map(|x| quote!(#x as u64));
            let tokens = option_to_tokens(&option_token);
            (option_token, tokens)
        };

        let quoted_error = quote_error(validation);
        let quoted = quote!(
            if !::validator::validate_length(
                #validator_param,
                #min_tokens,
                #max_tokens,
                #equal_tokens
            ) {
                #quoted_error
                #min_err_param_quoted
                #max_err_param_quoted
                #equal_err_param_quoted
                err.add_param(::std::borrow::Cow::from("value"), &#validator_param);
                errors.add(#field_name, err);
            }
        );

        let code = &validation.code;
        let constraint_quoted = match option_equal {
            Some(equal) => {
                quote!(
                    constraints.add(
                        #field_name,
                        ::validator::ValidationConstraint::Length {
                            length: ::validator::LengthConstraint::Equal(#equal),
                            code: #code,
                        },
                    );
                )
            }
            None => {
                quote!(
                    constraints.add(
                        #field_name,
                        ::validator::ValidationConstraint::Length {
                            length: ::validator::LengthConstraint::Range {
                                min: #min_tokens,
                                max: #max_tokens,
                            },
                            code: #code,
                        },
                    );
                )
            }
        };

        return (field_quoter.wrap_if_option(quoted), constraint_quoted);
    }

    unreachable!()
}

pub fn quote_range_validation(
    field_quoter: &FieldQuoter,
    validation: &FieldValidation,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let field_name = &field_quoter.name;
    let quoted_ident = field_quoter.quote_validator_param();

    if let Validator::Range { ref min, ref max, ref exclusive_min, ref exclusive_max } =
        validation.validator
    {
        let min_err_param_quoted = err_param_quoted(min, "min");
        let max_err_param_quoted = err_param_quoted(max, "max");
        let exclusive_min_err_param_quoted = err_param_quoted(exclusive_min, "exclusive_min");
        let exclusive_max_err_param_quoted = err_param_quoted(exclusive_max, "exclusive_max");

        // Can't interpolate None
        let min_tokens = generate_tokens(min);
        let max_tokens = generate_tokens(max);
        let exclusive_min_tokens = generate_tokens(exclusive_min);
        let exclusive_max_tokens = generate_tokens(exclusive_max);

        let quoted_error = quote_error(validation);
        let quoted = quote!(
            if !::validator::validate_range(
                #quoted_ident as f64,
                #min_tokens,
                #max_tokens,
                #exclusive_min_tokens,
                #exclusive_max_tokens,
            ) {
                #quoted_error
                #min_err_param_quoted
                #max_err_param_quoted
                #exclusive_min_err_param_quoted
                #exclusive_max_err_param_quoted
                err.add_param(::std::borrow::Cow::from("value"), &#quoted_ident);
                errors.add(#field_name, err);
            }
        );

        let code = &validation.code;
        let constraint_quoted = quote!(
            constraints.add(
                #field_name,
                ::validator::ValidationConstraint::Range {
                    min: #min_tokens,
                    max: #max_tokens,
                    code: #code,
                },
            );
        );

        return (field_quoter.wrap_if_option(quoted), constraint_quoted);
    }

    unreachable!()
}

fn err_param_quoted<T>(option: &Option<ValueOrPath<T>>, name: &str) -> proc_macro2::TokenStream
where
    T: std::fmt::Debug + std::clone::Clone + std::cmp::PartialEq + quote::ToTokens,
{
    if let Some(v) = option {
        let v = value_or_path_to_tokens(v);
        quote!(err.add_param(::std::borrow::Cow::from(#name), &#v);)
    } else {
        quote!()
    }
}

fn generate_tokens<T>(value: &Option<ValueOrPath<T>>) -> proc_macro2::TokenStream
where
    T: std::fmt::Debug + std::clone::Clone + std::cmp::PartialEq + quote::ToTokens,
{
    let tokens = value.clone().map(|x| value_or_path_to_tokens(&x)).map(|x| quote!(#x as f64));
    option_to_tokens(&tokens)
}

#[cfg(feature = "card")]
pub fn quote_credit_card_validation(
    field_quoter: &FieldQuoter,
    validation: &FieldValidation,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let field_name = &field_quoter.name;
    let validator_param = field_quoter.quote_validator_param();

    let quoted_error = quote_error(validation);
    let quoted = quote!(
        if !::validator::validate_credit_card(#validator_param) {
            #quoted_error
            err.add_param(::std::borrow::Cow::from("value"), &#validator_param);
            errors.add(#field_name, err);
        }
    );

    let code = &validation.code;
    let constraint_quoted = quote!(
        constraints.add(
            #field_name,
            ::validator::ValidationConstraint::CreditCard {
                code: #code,
            },
        );
    );

    (field_quoter.wrap_if_option(quoted), constraint_quoted)
}

#[cfg(feature = "unic")]
pub fn quote_non_control_character_validation(
    field_quoter: &FieldQuoter,
    validation: &FieldValidation,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let field_name = &field_quoter.name;
    let validator_param = field_quoter.quote_validator_param();

    let quoted_error = quote_error(validation);
    let quoted = quote!(
        if !::validator::validate_non_control_character(#validator_param) {
            #quoted_error
            err.add_param(::std::borrow::Cow::from("value"), &#validator_param);
            errors.add(#field_name, err);
        }
    );

    let code = &validation.code;
    let constraint_quoted = quote!(
        constraints.add(
            #field_name,
            ::validator::ValidationConstraint::NonControlCharacter {
                code: #code,
            },
        );
    );

    (field_quoter.wrap_if_option(quoted), constraint_quoted)
}

pub fn quote_url_validation(
    field_quoter: &FieldQuoter,
    validation: &FieldValidation,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let field_name = &field_quoter.name;
    let validator_param = field_quoter.quote_validator_param();

    let quoted_error = quote_error(validation);
    let quoted = quote!(
        if !::validator::validate_url(#validator_param) {
            #quoted_error
            err.add_param(::std::borrow::Cow::from("value"), &#validator_param);
            errors.add(#field_name, err);
        }
    );

    let code = &validation.code;
    let constraint_quoted = quote!(
        constraints.add(
            #field_name,
            ::validator::ValidationConstraint::Url {
                code: #code,
            },
        );
    );

    (field_quoter.wrap_if_option(quoted), constraint_quoted)
}

pub fn quote_email_validation(
    field_quoter: &FieldQuoter,
    validation: &FieldValidation,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let field_name = &field_quoter.name;
    let validator_param = field_quoter.quote_validator_param();

    let quoted_error = quote_error(validation);
    let quoted = quote!(
        if !::validator::validate_email(#validator_param) {
            #quoted_error
            err.add_param(::std::borrow::Cow::from("value"), &#validator_param);
            errors.add(#field_name, err);
        }
    );

    let code = &validation.code;
    let constraint_quoted = quote!(
        constraints.add(
            #field_name,
            ::validator::ValidationConstraint::Email {
                code: #code,
            },
        );
    );

    (field_quoter.wrap_if_option(quoted), constraint_quoted)
}

pub fn quote_must_match_validation(
    field_quoter: &FieldQuoter,
    validation: &FieldValidation,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let ident = field_quoter.field.ident.as_ref().unwrap();
    let field_name = &field_quoter.name;

    if let Validator::MustMatch(ref other) = validation.validator {
        let other_ident = syn::Ident::new(other, Span::call_site());
        let quoted_error = quote_error(validation);
        let quoted = quote!(
            if !::validator::validate_must_match(&self.#ident, &self.#other_ident) {
                #quoted_error
                err.add_param(::std::borrow::Cow::from("value"), &self.#ident);
                err.add_param(::std::borrow::Cow::from("other"), &self.#other_ident);
                errors.add(#field_name, err);
            }
        );

        let other_ident_string = other_ident.to_string();
        let code = &validation.code;
        let constraint_quoted = quote!(
            constraints.add(
                #field_name,
                ::validator::ValidationConstraint::MustMatch {
                    other_field: #other_ident_string,
                    code: #code,
                },
            );
        );

        return (field_quoter.wrap_if_option(quoted), constraint_quoted);
    }

    unreachable!();
}

pub fn quote_custom_validation(
    field_quoter: &FieldQuoter,
    validation: &FieldValidation,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let field_name = &field_quoter.name;
    let validator_param = field_quoter.quote_validator_param();

    if let Validator::Custom { function, argument, .. } = &validation.validator {
        let fn_ident: syn::Path = syn::parse_str(function).unwrap();

        let access = if_chain! {
            if let Some(argument) = &**argument;
            if let Some(access) = &argument.arg_access;
            then {
                quote!(, #access)
            } else {
                quote!()
            }
        };

        let add_message_quoted = if let Some(ref m) = validation.message {
            quote!(err.message = Some(::std::borrow::Cow::from(#m));)
        } else {
            quote!()
        };

        let quoted = quote!(
            match #fn_ident(#validator_param #access) {
                ::std::result::Result::Ok(()) => (),
                ::std::result::Result::Err(mut err) => {
                    #add_message_quoted
                    err.add_param(::std::borrow::Cow::from("value"), &#validator_param);
                    errors.add(#field_name, err);
                },
            };
        );

        let code = &validation.code;
        let constraint_quoted = quote!(
            constraints.add(
                #field_name,
                ::validator::ValidationConstraint::Custom {
                    function: #function,
                    code: #code,
                },
            );
        );

        return (field_quoter.wrap_if_option(quoted), constraint_quoted);
    }

    unreachable!();
}

pub fn quote_contains_validation(
    field_quoter: &FieldQuoter,
    validation: &FieldValidation,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let field_name = &field_quoter.name;
    let validator_param = field_quoter.quote_validator_param();

    if let Validator::Contains(ref needle) = validation.validator {
        let quoted_error = quote_error(validation);
        let quoted = quote!(
            if !::validator::validate_contains(#validator_param, &#needle) {
                #quoted_error
                err.add_param(::std::borrow::Cow::from("value"), &#validator_param);
                err.add_param(::std::borrow::Cow::from("needle"), &#needle);
                errors.add(#field_name, err);
            }
        );

        let code = &validation.code;
        let constraint_quoted = quote!(
            constraints.add(
                #field_name,
                ::validator::ValidationConstraint::Contains {
                    needle: #needle,
                    code: #code,
                },
            );
        );

        return (field_quoter.wrap_if_option(quoted), constraint_quoted);
    }

    unreachable!();
}

pub fn quote_regex_validation(
    field_quoter: &FieldQuoter,
    validation: &FieldValidation,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let field_name = &field_quoter.name;
    let validator_param = field_quoter.quote_validator_param();

    if let Validator::Regex(ref re) = validation.validator {
        let re_ident: syn::Path = syn::parse_str(re).unwrap();
        let quoted_error = quote_error(validation);
        let quoted = quote!(
            if !#re_ident.is_match(#validator_param) {
                #quoted_error
                err.add_param(::std::borrow::Cow::from("value"), &#validator_param);
                errors.add(#field_name, err);
            }
        );

        let code = &validation.code;
        let constraint_quoted = quote!(
            constraints.add(
                #field_name,
                ::validator::ValidationConstraint::Regex {
                    name: #re,
                    code: #code,
                },
            );
        );

        return (field_quoter.wrap_if_option(quoted), constraint_quoted);
    }

    unreachable!();
}

pub fn quote_nested_validation(
    field_quoter: &FieldQuoter,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let field_name = &field_quoter.name;
    let validator_field = field_quoter.quote_validator_field();
    let quoted = quote!(result = ::validator::ValidationErrors::merge(result, #field_name, #validator_field.validate()););

    let ty = &field_quoter.field.ty;
    let constraints_quoted = quote!(
        ::validator::ValidationConstraints::merge(
            &mut constraints, #field_name, <#ty as ::validator::Constraints>::constraints(),
        );
    );

    (field_quoter.wrap_if_option(field_quoter.wrap_if_collection(quoted)), constraints_quoted)
}

pub fn quote_validator(
    field_quoter: &FieldQuoter,
    validation: &FieldValidation,
    validations: &mut Vec<proc_macro2::TokenStream>,
    nested_validations: &mut Vec<proc_macro2::TokenStream>,
    constraints: &mut Vec<proc_macro2::TokenStream>,
    nested_constraints: &mut Vec<proc_macro2::TokenStream>,
) {
    match validation.validator {
        Validator::Length { .. } => {
            let (validation, constraint) = quote_length_validation(field_quoter, validation);
            validations.push(validation);
            constraints.push(constraint);
        }
        Validator::Range { .. } => {
            let (validation, constraint) = quote_range_validation(field_quoter, validation);
            validations.push(validation);
            constraints.push(constraint);
        }
        Validator::Email => {
            let (validation, constraint) = quote_email_validation(field_quoter, validation);
            validations.push(validation);
            constraints.push(constraint);
        }
        Validator::Url => {
            let (validation, constraint) = quote_url_validation(field_quoter, validation);
            validations.push(validation);
            constraints.push(constraint);
        }
        Validator::MustMatch(_) => {
            let (validation, constraint) = quote_must_match_validation(field_quoter, validation);
            validations.push(validation);
            constraints.push(constraint);
        }
        Validator::Custom { .. } => {
            let (validation, constraint) = quote_custom_validation(field_quoter, validation);
            validations.push(validation);
            constraints.push(constraint);
        }
        Validator::Contains(_) => {
            let (validation, constraint) = quote_contains_validation(field_quoter, validation);
            validations.push(validation);
            constraints.push(constraint);
        }
        Validator::Regex(_) => {
            let (validation, constraint) = quote_regex_validation(field_quoter, validation);
            validations.push(validation);
            constraints.push(constraint);
        }
        #[cfg(feature = "card")]
        Validator::CreditCard => {
            let (validation, constraint) = quote_credit_card_validation(field_quoter, validation);
            validations.push(validation);
            constraints.push(constraint);
        }
        Validator::Nested => {
            let (validation, constraint) = quote_nested_validation(field_quoter);
            nested_validations.push(validation);
            nested_constraints.push(constraint);
        }
        #[cfg(feature = "unic")]
        Validator::NonControlCharacter => {
            let (validation, constraint) =
                quote_non_control_character_validation(field_quoter, validation);
            validations.push(validation);
            constraints.push(constraint);
        }
        Validator::Required | Validator::RequiredNested => {
            let (validation, constraint) = quote_required_validation(field_quoter, validation);
            validations.push(validation);
            constraints.push(constraint);
        }
        Validator::DoesNotContain(_) => {
            let (validation, constraint) =
                quote_does_not_contain_validation(field_quoter, validation);
            validations.push(validation);
            constraints.push(constraint);
        }
    }
}

pub fn quote_schema_validation(v: &SchemaValidation) -> proc_macro2::TokenStream {
    let fn_ident: syn::Path = syn::parse_str(&v.function).unwrap();

    let arg_quoted = if let Some(ref args) = v.args {
        let arg_type = &args.arg_access;
        quote!(self, #arg_type)
    } else {
        quote!(self)
    };

    let add_message_quoted = if let Some(ref m) = v.message {
        quote!(err.message = Some(::std::borrow::Cow::from(#m));)
    } else {
        quote!()
    };

    let mut_err_token = if v.message.is_some() { quote!(mut) } else { quote!() };

    let quoted = quote!(
        match #fn_ident(#arg_quoted) {
            ::std::result::Result::Ok(()) => (),
            ::std::result::Result::Err(#mut_err_token err) => {
                #add_message_quoted
                errors.add("__all__", err);
            },
        };
    );

    if !v.skip_on_field_errors {
        return quoted;
    }

    quote!(
        #quoted
    )
}

pub fn quote_schema_validations(validation: &[SchemaValidation]) -> Vec<proc_macro2::TokenStream> {
    validation.iter().map(quote_schema_validation).collect()
}

pub fn quote_required_validation(
    field_quoter: &FieldQuoter,
    validation: &FieldValidation,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let field_name = &field_quoter.name;
    let ident = &field_quoter.field.ident.as_ref().unwrap();
    let validator_param = quote!(&self.#ident);

    let quoted_error = quote_error(validation);
    let quoted = quote!(
        if !::validator::validate_required(#validator_param) {
            #quoted_error
            err.add_param(::std::borrow::Cow::from("value"), &#validator_param);
            errors.add(#field_name, err);
        }
    );

    let code = &validation.code;
    let constraint_quoted = quote!(
        constraints.add(
            #field_name,
            ::validator::ValidationConstraint::Required {
                code: #code,
            },
        );
    );

    (quoted, constraint_quoted)
}

pub fn quote_does_not_contain_validation(
    field_quoter: &FieldQuoter,
    validation: &FieldValidation,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let field_name = &field_quoter.name;
    let validator_param = field_quoter.quote_validator_param();

    if let Validator::DoesNotContain(ref needle) = validation.validator {
        let quoted_error = quote_error(validation);
        let quoted = quote!(
            if !::validator::validate_does_not_contain(#validator_param, &#needle) {
                #quoted_error
                err.add_param(::std::borrow::Cow::from("value"), &#validator_param);
                err.add_param(::std::borrow::Cow::from("needle"), &#needle);
                errors.add(#field_name, err);
            }
        );

        let code = &validation.code;
        let constraint_quoted = quote!(
            constraints.add(
                #field_name,
                ::validator::ValidationConstraint::DoesNotContain {
                    needle: #needle,
                    code: #code,
                },
            );
        );

        return (field_quoter.wrap_if_option(quoted), constraint_quoted);
    }

    unreachable!();
}
