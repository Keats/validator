#![feature(proc_macro, proc_macro_lib)]
#![recursion_limit = "128"]

#[macro_use] extern crate quote;
extern crate proc_macro;
extern crate syn;
extern crate validator;


use proc_macro::TokenStream;
use quote::ToTokens;
use validator::{Validator};


static LENGTH_TYPES: [&'static str; 2] = ["String", "Vec"];
static RANGE_TYPES: [&'static str; 12] = [
    "usize", "u8", "u16", "u32", "u64", "isize", "i8", "i16", "i32", "i64", "f32", "f64"
];


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

    for field in fields {
        let field_ident = match field.ident {
            Some(ref i) => i,
            None => unreachable!()
        };

        let (name, validators) = find_validators_for_field(field);
        for validator in &validators {
            validations.push(match validator {
                &Validator::Length {min, max, equal} =>  {
                    // Can't interpolate None
                    let min_tokens = option_u64_to_tokens(min);
                    let max_tokens = option_u64_to_tokens(max);
                    let equal_tokens = option_u64_to_tokens(equal);
                    quote!(
                        if !::validator::validate_length(
                            ::validator::Validator::Length {
                                min: #min_tokens,
                                max: #max_tokens,
                                equal: #equal_tokens
                            },
                            &self.#field_ident
                        ) {
                            errors.entry(#name.to_string()).or_insert_with(|| vec![]).push("length".to_string());
                        }
                    )
                },
                &Validator::Range {min, max} => {
                    quote!(
                        if !::validator::validate_range(
                            ::validator::Validator::Range {min: #min, max: #max},
                            self.#field_ident as f64
                        ) {
                            errors.entry(#name.to_string()).or_insert_with(|| vec![]).push("range".to_string());
                        }
                    )
                },
                &Validator::Email => {
                    quote!(
                        if !::validator::validate_email(&self.#field_ident) {
                            errors.entry(#name.to_string()).or_insert_with(|| vec![]).push("email".to_string());
                        }
                    )
                }
                &Validator::Url => {
                    quote!(
                        if !::validator::validate_url(&self.#field_ident) {
                            errors.entry(#name.to_string()).or_insert_with(|| vec![]).push("url".to_string());
                        }
                    )
                },
                &Validator::Custom(ref f) => {
                    let fn_ident = syn::Ident::new(f.clone());
                    quote!(
                        match #fn_ident(&self.#field_ident) {
                            ::std::option::Option::Some(s) => errors.entry(#name.to_string()).or_insert_with(|| vec![]).push(s),
                            ::std::option::Option::None => (),
                        };
                    )
                },
            });
        }
    }

    let ident = &ast.ident;
    let impl_ast = quote!(
        impl Validate for #ident {
            fn validate(&self) -> ::std::result::Result<(), ::validator::Errors> {
                use std::collections::HashMap;
                let mut errors = HashMap::new();

                 #(#validations)*

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

/// Find everything we need to know about a Field.
fn find_validators_for_field(field: &syn::Field) -> (String, Vec<Validator>) {
    let mut field_name = match field.ident {
        Some(ref s) => s.to_string(),
        None => unreachable!(),
    };

    let error = |msg: &str| -> ! {
        panic!("Invalid attribute #[validate] on field `{}`: {}", field.ident.clone().unwrap().to_string(), msg);
    };

    let field_type = match field.ty {
        syn::Ty::Path(_, ref p) => {
            p.segments[0].ident.to_string()

        },
        _ => error(&format!("Type `{:?}` not supported", field.ty))
    };

    let mut validators = vec![];

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
                                    if field_type != "String" {
                                        panic!("`email` validator can only be used on String");
                                    }
                                    validators.push(Validator::Email);
                                },
                                "url" => {
                                    if field_type != "String" {
                                        panic!("`url` validator can only be used on String");
                                    }
                                    validators.push(Validator::Url);
                                },
                                _ => panic!("Unexpected word validator: {}", name)
                            },
                            // custom
                            syn::MetaItem::NameValue(ref name, ref val) => {
                                if name == "custom" {
                                    match lit_to_string(val) {
                                        Some(s) => validators.push(Validator::Custom(s)),
                                        None => error("invalid argument for `custom` validator: only strings are allowed"),
                                    };
                                } else {
                                    panic!("unexpected name value validator: {:?}", name);
                                }
                            },
                            // validators with args: length for example
                            syn::MetaItem::List(ref name, ref meta_items) => {
                                // Some sanity checking first
                                if name == "length" {
                                    if !LENGTH_TYPES.contains(&field_type.as_ref()) {
                                        error(&format!(
                                            "Validator `length` can only be used on types `String` or `Vec` but found `{}`",
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

    if validators.is_empty() {
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

//
//fn quote_length_validator(min: Option<u64>, max: Option<u64>, equal: Option<u64>) {
//
//}

fn lit_to_string(lit: &syn::Lit) -> Option<String> {
    match *lit {
        syn::Lit::Str(ref s, _) => Some(s.to_string()),
        _ => None,
    }
}

fn lit_to_int(lit: &syn::Lit) -> Option<u64> {
    match *lit {
        syn::Lit::Int(ref s, _) => Some(*s),
        _ => None,
    }
}

fn lit_to_float(lit: &syn::Lit) -> Option<f64> {
    match *lit {
        syn::Lit::Float(ref s, _) => Some(s.parse::<f64>().unwrap()),
        syn::Lit::Int(ref s, _) => Some(*s as f64),
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
