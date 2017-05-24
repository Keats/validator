#![recursion_limit = "128"]


#[macro_use] extern crate quote;
extern crate proc_macro;
extern crate syn;
extern crate validator;

use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::ToTokens;
use validator::{Validator};


static RANGE_TYPES: [&'static str; 24] = [
    "usize", "u8", "u16", "u32", "u64",
    "isize", "i8", "i16", "i32", "i64",
    "f32", "f64",

    "Option<usize>", "Option<u8>", "Option<u16>", "Option<u32>", "Option<u64>",
    "Option<isize>", "Option<i8>", "Option<i16>", "Option<i32>", "Option<i64>",
    "Option<f32>", "Option<f64>",
];

#[derive(Debug)]
struct SchemaValidation {
    function: String,
    skip_on_field_errors: bool,
}


#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive_validation(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    // Parse the string representation to an AST
    let ast = syn::parse_macro_input(&source).unwrap();

    let expanded = expand_validation(&ast);
    expanded.parse().unwrap()
}


fn expand_validation(ast: &syn::MacroInput) -> quote::Tokens {
    let fields = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => {
            if fields.iter().any(|field| field.ident.is_none()) {
                panic!("struct has unnamed fields");
            }
            fields
        },
        _ => panic!("#[derive(Validate)] can only be used with structs"),
    };

    let mut validations = vec![];

    let field_types = find_fields_type(&fields);

    for field in fields {
        let field_ident = match field.ident {
            Some(ref i) => i,
            None => unreachable!()
        };

        let (name, validators) = find_validators_for_field(field, &field_types);
        let field_name = field_types.get(&field_ident.to_string()).unwrap();
        // Don't put a & in front a pointer
        let validator_param = if field_name.starts_with("&") {
          quote!(self.#field_ident)
        } else {
          quote!(&self.#field_ident)
        };
        // same but for the ident used in a if let block
        let optional_validator_param = quote!(#field_ident);
        // same but for the ident used in a if let Some variable
        let optional_pattern_matched = if field_name.starts_with("Option<&") {
          quote!(#field_ident)
        } else {
          quote!(ref #field_ident)
        };

        for validator in &validators {
            validations.push(match validator {
                &Validator::Length {min, max, equal} =>  {
                    // Can't interpolate None
                    let min_tokens = option_u64_to_tokens(min);
                    let max_tokens = option_u64_to_tokens(max);
                    let equal_tokens = option_u64_to_tokens(equal);
                    // wrap in if-let if we have an option
                    if field_name.starts_with("Option<") {
                        quote!(
                            if let Some(#optional_pattern_matched) = self.#field_ident {
                                if let Err(err) = ::validator::validate_length(
                                    ::validator::Validator::Length {
                                        min: #min_tokens,
                                        max: #max_tokens,
                                        equal: #equal_tokens
                                    },
                                    #optional_validator_param
                                ) {
                                    errors.add(#name, "length", &err);
                                }
                            }
                        )
                    } else {
                        quote!(
                            if let Err(err) = ::validator::validate_length(
                                ::validator::Validator::Length {
                                    min: #min_tokens,
                                    max: #max_tokens,
                                    equal: #equal_tokens
                                },
                                #validator_param
                            ) {
                                errors.add(#name, "length", &err);
                            }
                        )
                    }
                },
                &Validator::Range {min, max} => {
                    // wrap in if-let if we have an option
                    if field_name.starts_with("Option<") {
                        quote!(
                            if let Some(#field_ident) = self.#field_ident {
                                if let Err(err) = ::validator::validate_range(
                                    ::validator::Validator::Range {min: #min, max: #max},
                                    #field_ident as f64
                                ) {
                                    errors.add(#name, "range", &err);
                                }
                            }
                        )
                    } else {
                        quote!(
                            if let Err(err) = ::validator::validate_range(
                                ::validator::Validator::Range {min: #min, max: #max},
                                self.#field_ident as f64
                            ) {
                                errors.add(#name, "range", &err);
                            }
                        )
                    }
                },
                &Validator::Email => {
                    // wrap in if-let if we have an option
                    if field_name.starts_with("Option<") {
                        quote!(
                            if let Some(#optional_pattern_matched) = self.#field_ident {
                                if let Err(err) = ::validator::validate_email(#optional_validator_param) {
                                    errors.add(#name, "email", err);
                                }
                            }
                        )
                    } else {
                        quote!(
                            if let Err(err) = ::validator::validate_email(#validator_param) {
                                errors.add(#name, "email", err);
                            }
                        )
                    }
                }
                &Validator::Url => {
                    // wrap in if-let if we have an option
                    if field_name.starts_with("Option<") {
                        quote!(
                            if let Some(#optional_pattern_matched) = self.#field_ident {
                                if let Err(err) = ::validator::validate_url(#optional_validator_param) {
                                    errors.add(#name, "url", err);
                                }
                            }
                        )
                    } else {
                        quote!(
                            if let Err(err) = ::validator::validate_url(#validator_param) {
                                errors.add(#name, "url", err);
                            }
                        )
                    }
                },
                &Validator::MustMatch(ref f) => {
                    let other_ident = syn::Ident::new(f.clone());
                    quote!(
                        if let Err(err) = ::validator::validate_must_match(
                            &self.#field_ident, stringify!(#field_ident),
                            &self.#other_ident, stringify!(#other_ident)
                        ) {
                            errors.add(#name, "no_match", &err);
                        }
                    )
                },
                &Validator::Custom(ref f) => {
                    let fn_ident = syn::Ident::new(f.clone());
                    // wrap in if-let if we have an option
                    if field_name.starts_with("Option<") {
                        quote!(
                            if let Some(#optional_pattern_matched) = self.#field_ident {
                                match #fn_ident(#optional_validator_param) {
                                    ::std::option::Option::Some(s) => {
                                        errors.add(#name, "custom", &s);
                                    },
                                    ::std::option::Option::None => (),
                                };
                            }
                        )
                    } else {
                        quote!(
                            match #fn_ident(#validator_param) {
                                ::std::option::Option::Some(s) => {
                                    errors.add(#name, "custom", &s);
                                },
                                ::std::option::Option::None => (),
                            };
                        )
                    }
                },
                &Validator::Contains(ref n) => {
                    // wrap in if-let if we have an option
                    if field_name.starts_with("Option<") {
                        quote!(
                            if let Some(#optional_pattern_matched) = self.#field_ident {
                                if let Err(err) = ::validator::validate_contains(#optional_validator_param, &#n) {
                                    errors.add(#name, "contains", &err);
                                }
                            }
                        )
                    } else {
                        quote!(
                            if let Err(err) = ::validator::validate_contains(#validator_param, &#n) {
                                errors.add(#name, "contains", &err);
                            }
                        )
                    }
                },
                &Validator::Regex(ref re) => {
                    let re_ident = syn::Ident::new(re.clone());
                    // wrap in if-let if we have an option
                    if field_name.starts_with("Option<") {
                        quote!(
                            if let Some(#optional_pattern_matched) = self.#field_ident {
                                if !#re_ident.is_match(#optional_validator_param) {
                                    errors.add(#name, "regex", "errorMessage");
                                }
                            }
                        )
                    } else {
                        quote!(
                            if !#re_ident.is_match(#validator_param) {
                                errors.add(#name, "regex", "errorMessage");
                            }
                        )
                    }
                },
            });
        }
    }

    let struct_validation = find_struct_validation(&ast.attrs);
    let struct_validation_tokens = match struct_validation {
        Some(s) => {
            let fn_ident = syn::Ident::new(s.function);
            if s.skip_on_field_errors {
                quote!(
                    if errors.is_empty() {
                        match #fn_ident(self) {
                            ::std::option::Option::Some((key, val)) => {
                                errors.add(&key, &val, "errorMessage");
                            },
                            ::std::option::Option::None => (),
                        }
                    }
                )
            } else {
                quote!(
                    match #fn_ident(self) {
                        ::std::option::Option::Some((key, val)) => {
                            errors.add(&key, &val, "errorMessage");
                        },
                        ::std::option::Option::None => (),
                    }
                )
            }
        },
        None => quote!()
    };

    let ident = &ast.ident;

    // Helper is provided for handling complex generic types correctly and effortlessly
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let impl_ast = quote!(
        impl #impl_generics Validate for #ident #ty_generics #where_clause {
            fn validate(&self) -> ::std::result::Result<(), ::validator::Errors> {
                let mut errors = ::validator::Errors::new();

                #(#validations)*

                #struct_validation_tokens

                if errors.is_empty() {
                    ::std::result::Result::Ok(())
                } else {
                    ::std::result::Result::Err(errors)
                }
            }
        }
    );
    // println!("{}", impl_ast.to_string());
    impl_ast
}


/// Find if a struct has some schema validation and returns the info if so
fn find_struct_validation(struct_attrs: &Vec<syn::Attribute>) -> Option<SchemaValidation> {
    let error = |msg: &str| -> ! {
        panic!("Invalid schema level validation: {}", msg);
    };

    for attr in struct_attrs {
        if attr.value.name() != "validate" {
            continue;
        }
        match attr.value {
            syn::MetaItem::List(_, ref meta_items) => {
                match meta_items[0] {
                    syn::NestedMetaItem::MetaItem(ref item) => match item {
                        &syn::MetaItem::List(ref ident2, ref args) => {
                            if ident2 != "schema" {
                                error("Only `schema` is allowed as validator on a struct")
                            }

                            let mut function = "".to_string();
                            let mut skip_on_field_errors = true;
                            for arg in args {
                                match *arg {
                                    syn::NestedMetaItem::MetaItem(ref item) => match *item {
                                        syn::MetaItem::NameValue(ref name, ref val) => {
                                            match name.to_string().as_ref() {
                                                "function" => {
                                                    function = match lit_to_string(val) {
                                                        Some(s) => s,
                                                        None => error("invalid argument type for `function` \
                                                        : only a string is allowed"),
                                                    };
                                                },
                                                "skip_on_field_errors" => {
                                                    skip_on_field_errors = match lit_to_bool(val) {
                                                        Some(s) => s,
                                                        None => error("invalid argument type for `skip_on_field_errors` \
                                                        : only a bool is allowed"),
                                                    };
                                                },
                                                _ => error("Unknown argument")
                                            }

                                        },
                                        _ => error("Unexpected args")
                                    },
                                    _ => error("Unexpected args")
                                }
                            }

                            if function == "" {
                                error("`function` is required");
                            }

                            return Some(SchemaValidation {
                                function: function,
                                skip_on_field_errors: skip_on_field_errors
                            });
                        },
                        _ => error("Unexpected struct validator")
                    },
                    _ => error("Unexpected struct validator")
                }
            },
            _ => error("Unexpected struct validator")
        }
    }

    None
}


// Find all the types (as string) for each field of the struct
// Needed for the `must_match` filter
fn find_fields_type(fields: &Vec<syn::Field>) -> HashMap<String, String> {
    let mut types = HashMap::new();

    for field in fields {
        let field_name = match field.ident {
            Some(ref s) => s.to_string(),
            None => unreachable!(),
        };
        let field_type = match field.ty {
            syn::Ty::Path(_, ref p) => {
                let mut tokens = quote::Tokens::new();
                p.to_tokens(&mut tokens);
                tokens.to_string().replace(' ', "")

            },
            syn::Ty::Rptr(ref l, ref p) => {
                let mut tokens = quote::Tokens::new();
                p.ty.to_tokens(&mut tokens);
                let mut name = tokens.to_string().replace(' ', "");
                if l.is_some() {
                    name.insert(0, '&')
                }
                name
            },
            _ => panic!("Type `{:?}` of field `{}` not supported", field.ty, field_name)
        };
        //println!("{:?}", field_type);
        types.insert(field_name, field_type);
    }

    types
}

/// Find everything we need to know about a Field.
fn find_validators_for_field(field: &syn::Field, field_types: &HashMap<String, String>) -> (String, Vec<Validator>) {
    let mut field_name = match field.ident {
        Some(ref s) => s.to_string(),
        None => unreachable!(),
    };

    let error = |msg: &str| -> ! {
        panic!("Invalid attribute #[validate] on field `{}`: {}", field.ident.clone().unwrap().to_string(), msg);
    };
    let field_type = field_types.get(&field_name).unwrap();

    let mut validators = vec![];
    let mut has_validate = false;

    let find_struct_validator = |name: String, meta_items: &Vec<syn::NestedMetaItem>| -> Validator {
        match name.as_ref() {
            "length" => {
                let mut min = None;
                let mut max = None;
                let mut equal = None;

                for meta_item in meta_items {
                    match *meta_item {
                        syn::NestedMetaItem::MetaItem(ref item) => match *item {
                            syn::MetaItem::NameValue(ref name, ref val) => {
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
                                    _ => error(&format!(
                                        "unknown argument `{}` for validator `length` (it only has `min`, `max`, `equal`)",
                                        name.to_string()
                                    ))
                                }
                            },
                            _ => panic!("unexpected item {:?} while parsing `length` validator", item)
                        },
                        _=> unreachable!()
                    }
                }
                if equal.is_some() && (min.is_some() || max.is_some()) {
                    error("both `equal` and `min` or `max` have been set in `length` validator: probably a mistake");
                }
                Validator::Length { min: min, max: max, equal: equal }
            },
            "range" => {
                let mut min = 0.0;
                let mut max = 0.0;
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
                                    },
                                    "max" => {
                                        max = match lit_to_float(val) {
                                            Some(s) => s,
                                            None => error("invalid argument type for `max` of `range` validator: only integers are allowed")
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

                Validator::Range { min: min, max: max}
            }
            _ => panic!("unexpected list validator: {:?}", name)
        }
    };

    for attr in &field.attrs {
        if attr.name() != "validate" && attr.name() != "serde" {
            continue;
        }

        if attr.name() == "validate" {
            has_validate = true;
        }

        match attr.value {
            syn::MetaItem::List(_, ref meta_items) => {
                if attr.name() == "serde" {
                    match find_original_field_name(meta_items) {
                        Some(s) => { field_name = s },
                        None => ()
                    };
                    continue;
                }

                // only validation from there on
                for meta_item in meta_items {
                    match *meta_item {
                        syn::NestedMetaItem::MetaItem(ref item) => match *item {
                            // email, url
                            syn::MetaItem::Word(ref name) => match name.to_string().as_ref() {
                                "email" => {
                                    if field_type != "String"
                                        && field_type != "&str"
                                        && field_type != "Option<String>"
                                        && !(field_type.starts_with("Option<") && field_type.ends_with("str>")) {
                                        panic!("`email` validator can only be used on String or &str");
                                    }
                                    validators.push(Validator::Email);
                                },
                                "url" => {
                                    if field_type != "String"
                                        && field_type != "&str"
                                        && field_type != "Option<String>"
                                        && !(field_type.starts_with("Option<") && field_type.ends_with("str>")) {
                                        panic!("`url` validator can only be used on String or &str");
                                    }
                                    validators.push(Validator::Url);
                                },
                                _ => panic!("Unexpected word validator: {}", name)
                            },
                            // custom, contains, must_match
                            syn::MetaItem::NameValue(ref name, ref val) => {
                                match name.to_string().as_ref() {
                                    "custom" => {
                                        match lit_to_string(val) {
                                            Some(s) => validators.push(Validator::Custom(s)),
                                            None => error("invalid argument for `custom` validator: only strings are allowed"),
                                        };
                                    },
                                    "contains" => {
                                        match lit_to_string(val) {
                                            Some(s) => validators.push(Validator::Contains(s)),
                                            None => error("invalid argument for `contains` validator: only strings are allowed"),
                                        };
                                    },
                                    "regex" => {
                                        match lit_to_string(val) {
                                            Some(s) => validators.push(Validator::Regex(s)),
                                            None => error("invalid argument for `regex` validator: only strings are allowed"),
                                        };
                                    }
                                    "must_match" => {
                                        match lit_to_string(val) {
                                            Some(s) => {
                                                if let Some(t2) = field_types.get(&s) {
                                                    if field_type == t2 {
                                                        validators.push(Validator::MustMatch(s));
                                                    } else {
                                                        error("invalid argument for `must_match` validator: types of field can't match");
                                                    }
                                                } else {
                                                    error("invalid argument for `must_match` validator: field doesn't exist in struct");
                                                }
                                            },
                                            None => error("invalid argument for `must_match` validator: only strings are allowed"),
                                        };
                                    },
                                    _ => panic!("unexpected name value validator: {:?}", name),
                                };
                            },
                            // validators with args: length for example
                            syn::MetaItem::List(ref name, ref meta_items) => {
                                // Some sanity checking first
                                if name == "length" {
                                    if field_type != "String"
                                        && !field_type.starts_with("Vec<")
                                        && !field_type.starts_with("Option<Vec<")
                                        && field_type != "Option<String>"
                                        // a bit ugly
                                        && !(field_type.starts_with("Option<") && field_type.ends_with("str>"))
                                        && field_type != "&str" {
                                        error(&format!(
                                            "Validator `length` can only be used on types `String`, `&str` or `Vec` but found `{}`",
                                            field_type
                                        ));
                                    }

                                    if meta_items.len() == 0 {
                                        error("Validator `length` requires at least 1 argument out of `min`, `max` and `equal`");
                                    }
                                }

                                if name == "range" {
                                    if !RANGE_TYPES.contains(&field_type.as_ref()) {
                                        error(&format!(
                                            "Validator `range` can only be used on number types but found `{}`",
                                            field_type
                                        ));
                                    }

                                    if meta_items.len() != 2 {
                                        error("Validator `range` requires 2 arguments: `min` and `max`");
                                    }
                                }

                                validators.push(find_struct_validator(name.to_string(), meta_items));
                            },
                        },
                        _ => unreachable!("Found a non MetaItem while looking for validators")
                    };
                }
            },
            _ => unreachable!("Got something other than a list of attributes while checking field `{}`", field_name),
        }
    }

    if has_validate && validators.is_empty() {
        error("it needs at least one validator");
    }

    (field_name, validators)
}

/// Serde can be used to rename fields on deserialization but most of the times
/// we want the error on the original field.
///
/// For example a JS frontend might send camelCase fields and Rust converts them to snake_case
/// but we want to send the errors back to the frontend with the original name
fn find_original_field_name(meta_items: &Vec<syn::NestedMetaItem>) -> Option<String> {
    let mut original_name = None;

    for meta_item in meta_items {
        match *meta_item {
            syn::NestedMetaItem::MetaItem(ref item) => match *item {
                syn::MetaItem::Word(_) => continue,
                syn::MetaItem::NameValue(ref name, ref val) => {
                    if name == "rename" {
                        original_name = Some(lit_to_string(val).unwrap());
                    }

                },
                // length
                syn::MetaItem::List(_, ref meta_items) => {
                    return find_original_field_name(meta_items);
                }
            },
            _ => unreachable!()
        };

        if original_name.is_some() {
            return original_name;
        }
    }

    original_name
}


fn lit_to_string(lit: &syn::Lit) -> Option<String> {
    match *lit {
        syn::Lit::Str(ref s, _) => Some(s.to_string()),
        _ => None,
    }
}

fn lit_to_int(lit: &syn::Lit) -> Option<u64> {
    match *lit {
        syn::Lit::Int(ref s, _) => Some(*s),
        // TODO: remove when attr_literals is stable
        syn::Lit::Str(ref s, _) => Some(s.parse::<u64>().unwrap()),
        _ => None,
    }
}

fn lit_to_float(lit: &syn::Lit) -> Option<f64> {
    match *lit {
        syn::Lit::Float(ref s, _) => Some(s.parse::<f64>().unwrap()),
        syn::Lit::Int(ref s, _) => Some(*s as f64),
        // TODO: remove when attr_literals is stable
        syn::Lit::Str(ref s, _) => Some(s.parse::<f64>().unwrap()),
        _ => None,
    }
}

fn lit_to_bool(lit: &syn::Lit) -> Option<bool> {
    match *lit {
        syn::Lit::Bool(ref s) => Some(*s),
        // TODO: remove when attr_literals is stable
        syn::Lit::Str(ref s, _) => if s == "true" { Some(true) } else { Some(false) },
        _ => None,
    }
}

fn option_u64_to_tokens(opt: Option<u64>) -> quote::Tokens {
    let mut tokens = quote::Tokens::new();
    tokens.append("::");
    tokens.append("std");
    tokens.append("::");
    tokens.append("option");
    tokens.append("::");
    tokens.append("Option");
    tokens.append("::");
    match opt {
        Some(ref t) => {
            tokens.append("Some");
            tokens.append("(");
            t.to_tokens(&mut tokens);
            tokens.append(")");
        }
        None => {
            tokens.append("None");
        }
    }
    tokens
}
