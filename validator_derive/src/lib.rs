#![recursion_limit = "128"]

#[macro_use]
extern crate if_chain;
#[macro_use]
extern crate lazy_static;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate regex;
#[macro_use]
extern crate syn;
extern crate validator;

use proc_macro::TokenStream;
use quote::ToTokens;
use std::collections::HashMap;
use validator::Validator;


mod lit;
mod validation;
mod asserts;
mod quoting;

use lit::*;
use validation::*;
use asserts::{assert_string_type, assert_type_matches, assert_has_len, assert_has_range};
use quoting::{FieldQuoter, quote_field_validation, quote_schema_validation};


#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive_validation(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_validate(&ast).into()
}


fn impl_validate(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    // Ensure the macro is on a struct with named fields
    let fields = match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            if fields.iter().any(|field| field.ident.is_none()) {
                panic!("struct has unnamed fields");
            }
            fields.iter().cloned().collect()
        },
        _ => panic!("#[derive(Validate)] can only be used with structs"),
    };

    let mut validations = vec![];
    let mut nested_validations = vec![];

    let field_types = find_fields_type(&fields);

    for field in &fields {
        let field_ident = field.ident.clone().unwrap();
        let (name, field_validations) = find_validators_for_field(field, &field_types);
        let field_type = field_types.get(&field_ident.to_string()).cloned().unwrap();
        let field_quoter = FieldQuoter::new(field_ident, name, field_type);

        for validation in &field_validations {
            quote_field_validation(&field_quoter, validation, &mut validations, &mut nested_validations);
        }
    }

    let schema_validation = quote_schema_validation(find_struct_validation(&ast.attrs));

    let ident = &ast.ident;

    // Helper is provided for handling complex generic types correctly and effortlessly
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let impl_ast = quote!(
        impl #impl_generics Validate for #ident #ty_generics #where_clause {
            #[allow(unused_mut)]
            fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
                let mut errors = ::validator::ValidationErrors::new();

                #(#validations)*

                #schema_validation

                let mut result = if errors.is_empty() {
                    ::std::result::Result::Ok(())
                } else {
                    ::std::result::Result::Err(errors)
                };

                #(#nested_validations)*
                result
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
        if attr.path != parse_quote!(validate) {
            continue;
        }

        if_chain! {
            if let Some(syn::Meta::List(syn::MetaList { ref nested, .. })) = attr.interpret_meta();
            if let syn::NestedMeta::Meta(ref item) = nested[0];
            if let &syn::Meta::List(syn::MetaList { ref ident, ref nested, .. }) = item;

            then {
                if ident != "schema" {
                    error("Only `schema` is allowed as validator on a struct")
                }

                let mut function = String::new();
                let mut skip_on_field_errors = true;
                let mut code = None;
                let mut message = None;

                for arg in nested {
                    if_chain! {
                        if let syn::NestedMeta::Meta(ref item) = *arg;
                        if let syn::Meta::NameValue(syn::MetaNameValue { ref ident, ref lit, .. }) = *item;

                        then {
                            match ident.to_string().as_ref() {
                                "function" => {
                                    function = match lit_to_string(lit) {
                                        Some(s) => s,
                                        None => error("invalid argument type for `function` \
                                        : only a string is allowed"),
                                    };
                                },
                                "skip_on_field_errors" => {
                                    skip_on_field_errors = match lit_to_bool(lit) {
                                        Some(s) => s,
                                        None => error("invalid argument type for `skip_on_field_errors` \
                                        : only a bool is allowed"),
                                    };
                                },
                                "code" => {
                                    code = match lit_to_string(lit) {
                                        Some(s) => Some(s),
                                        None => error("invalid argument type for `code` \
                                        : only a string is allowed"),
                                    };
                                },
                                "message" => {
                                    message = match lit_to_string(lit) {
                                        Some(s) => Some(s),
                                        None => error("invalid argument type for `message` \
                                        : only a string is allowed"),
                                    };
                                },
                                _ => error("Unknown argument")
                            }
                        } else {
                            error("Unexpected args")
                        }
                    }
                }

                if function == "" {
                    error("`function` is required");
                }

                return Some(
                    SchemaValidation {
                        function,
                        skip_on_field_errors,
                        code,
                        message,
                    }
                );
            } else {
                error("Unexpected struct validator")
            }
        }
    }

    None
}


/// Find the types (as string) for each field of the struct
/// Needed for the `must_match` filter
fn find_fields_type(fields: &Vec<syn::Field>) -> HashMap<String, String> {
    let mut types = HashMap::new();

    for field in fields {
        let field_ident = field.ident.clone().unwrap().to_string();
        let field_type = match field.ty {
            syn::Type::Path(syn::TypePath { ref path, .. }) => {
                let mut tokens = proc_macro2::TokenStream::new();
                path.to_tokens(&mut tokens);
                tokens.to_string().replace(' ', "")

            },
            syn::Type::Reference(syn::TypeReference { ref lifetime, ref elem, .. }) => {
                let mut tokens = proc_macro2::TokenStream::new();
                elem.to_tokens(&mut tokens);
                let mut name = tokens.to_string().replace(' ', "");
                if lifetime.is_some() {
                    name.insert(0, '&')
                }
                name
            },
            _ => panic!("Type `{:?}` of field `{}` not supported", field.ty, field_ident)
        };

        //println!("{:?}", field_type);
        types.insert(field_ident, field_type);
    }

    types
}

/// Find everything we need to know about a field: its real name if it's changed from the serialization
/// and the list of validators to run on it
fn find_validators_for_field(field: &syn::Field, field_types: &HashMap<String, String>) -> (String, Vec<FieldValidation>) {
    let rust_ident = field.ident.clone().unwrap().to_string();
    let mut field_ident = field.ident.clone().unwrap().to_string();

    let error = |msg: &str| -> ! {
        panic!("Invalid attribute #[validate] on field `{}`: {}", field.ident.clone().unwrap().to_string(), msg);
    };

    let field_type = field_types.get(&field_ident).unwrap();

    let mut validators = vec![];
    let mut has_validate = false;

    for attr in &field.attrs {
        if attr.path != parse_quote!(validate) && attr.path != parse_quote!(serde) {
            continue;
        }

        if attr.path == parse_quote!(validate) {
            has_validate = true;
        }

        match attr.interpret_meta() {
            Some(syn::Meta::List(syn::MetaList { ref nested, .. })) => {
                let meta_items = nested.iter().collect();
                // original name before serde rename
                if attr.path == parse_quote!(serde) {
                    if let Some(s) = find_original_field_name(&meta_items) {
                        field_ident = s;
                    }
                    continue;
                }

                // only validation from there on
                for meta_item in meta_items {
                    match *meta_item {
                        syn::NestedMeta::Meta(ref item) => match *item {
                            // email, url, phone
                            syn::Meta::Word(ref name) => match name.to_string().as_ref() {
                                "email" => {
                                    assert_string_type("email", field_type);
                                    validators.push(FieldValidation::new(Validator::Email));
                                },
                                "url" => {
                                    assert_string_type("url", field_type);
                                    validators.push(FieldValidation::new(Validator::Url));
                                },
                                #[cfg(feature = "phone")]
                                "phone" => {
                                    assert_string_type("phone", field_type);
                                    validators.push(FieldValidation::new(Validator::Phone));
                                },
                                #[cfg(feature = "card")]
                                "credit_card" => {
                                    assert_string_type("credit_card", field_type);
                                    validators.push(FieldValidation::new(Validator::CreditCard));
                                },
                                _ => panic!("Unexpected validator: {}", name)
                            },
                            // custom, contains, must_match, regex
                            syn::Meta::NameValue(syn::MetaNameValue { ref ident, ref lit, .. }) => {
                                match ident.to_string().as_ref() {
                                    "custom" => {
                                        match lit_to_string(lit) {
                                            Some(s) => validators.push(FieldValidation::new(Validator::Custom(s))),
                                            None => error("invalid argument for `custom` validator: only strings are allowed"),
                                        };
                                    },
                                    "contains" => {
                                        match lit_to_string(lit) {
                                            Some(s) => validators.push(FieldValidation::new(Validator::Contains(s))),
                                            None => error("invalid argument for `contains` validator: only strings are allowed"),
                                        };
                                    },
                                    "regex" => {
                                        match lit_to_string(lit) {
                                            Some(s) => validators.push(FieldValidation::new(Validator::Regex(s))),
                                            None => error("invalid argument for `regex` validator: only strings are allowed"),
                                        };
                                    }
                                    "must_match" => {
                                        match lit_to_string(lit) {
                                            Some(s) => {
                                                assert_type_matches(rust_ident.clone(), field_type, field_types.get(&s));
                                                validators.push(FieldValidation::new(Validator::MustMatch(s)));
                                            },
                                            None => error("invalid argument for `must_match` validator: only strings are allowed"),
                                        };
                                    },
                                    v => panic!("unexpected name value validator: {:?}", v),
                                };
                            },
                            // Validators with several args
                            syn::Meta::List(syn::MetaList { ref ident, ref nested, .. }) => {
                                let meta_items = nested.iter().cloned().collect();
                                match ident.to_string().as_ref() {
                                    "length" => {
                                        assert_has_len(rust_ident.clone(), field_type);
                                        validators.push(extract_length_validation(rust_ident.clone(), &meta_items));
                                    },
                                    "range" => {
                                        assert_has_range(rust_ident.clone(), field_type);
                                        validators.push(extract_range_validation(rust_ident.clone(), &meta_items));
                                    },
                                    "email" | "url" | "phone" | "credit_card" => {
                                        validators.push(extract_argless_validation(ident.to_string(), rust_ident.clone(), &meta_items));
                                    },
                                    "custom" => {
                                        validators.push(extract_one_arg_validation("function", ident.to_string(), rust_ident.clone(), &meta_items));
                                    },
                                    "contains" => {
                                        validators.push(extract_one_arg_validation("pattern", ident.to_string(), rust_ident.clone(), &meta_items));
                                    },
                                    "regex" => {
                                        validators.push(extract_one_arg_validation("path", ident.to_string(), rust_ident.clone(), &meta_items));
                                    },
                                    "must_match" => {
                                        let validation = extract_one_arg_validation("other", ident.to_string(), rust_ident.clone(), &meta_items);
                                        if let Validator::MustMatch(ref t2) = validation.validator {
                                            assert_type_matches(rust_ident.clone(), field_type, field_types.get(t2));
                                        }
                                        validators.push(validation);
                                    },
                                    v => panic!("unexpected list validator: {:?}", v)
                                }
                            },
                        },
                        _ => unreachable!("Found a non Meta while looking for validators")
                    };
                }
            },
            Some(syn::Meta::Word(_)) => {
                validators.push(FieldValidation::new(Validator::Nested))
            },
            _ => unreachable!("Got something other than a list of attributes while checking field `{}`", field_ident),
        }
    }

    if has_validate && validators.is_empty() {
        error("it needs at least one validator");
    }

    (field_ident, validators)
}

/// Serde can be used to rename fields on deserialization but most of the times
/// we want the error on the original field.
///
/// For example a JS frontend might send camelCase fields and Rust converts them to snake_case
/// but we want to send the errors back with the original name
fn find_original_field_name(meta_items: &Vec<&syn::NestedMeta>) -> Option<String> {
    let mut original_name = None;

    for meta_item in meta_items {
        match *meta_item {
            &syn::NestedMeta::Meta(ref item) => match *item {
                syn::Meta::Word(_) => continue,
                syn::Meta::NameValue(syn::MetaNameValue { ref ident, ref lit, .. }) => {
                    if ident == "rename" {
                        original_name = Some(lit_to_string(lit).unwrap());
                    }
                },
                syn::Meta::List(syn::MetaList { ref nested, .. }) => {
                    return find_original_field_name(&nested.iter().collect());
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

