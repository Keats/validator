// #![recursion_limit = "128"]
#![allow(unused)]

use std::collections::HashMap;

use darling::ast::Data;
use darling::util::Override;
use darling::{FromDeriveInput, FromField, FromMeta};
use proc_macro2::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Expr, Ident, Lit};

use tokens::cards::credit_card_tokens;
use tokens::contains::contains_tokens;
use tokens::custom::custom_tokens;
use tokens::does_not_contain::does_not_contain_tokens;
use tokens::email::email_tokens;
use tokens::ip::ip_tokens;
use tokens::length::length_tokens;
use tokens::must_match::must_match_tokens;
use tokens::non_control_character::non_control_char_tokens;
use tokens::range::range_tokens;
use tokens::regex::regex_tokens;
use tokens::required::required_tokens;
use tokens::required_nested::required_nested_tokens;
use tokens::url::url_tokens;
use types::*;

mod tokens;
mod types;
mod utils;

// This struct holds all the validation information on a field
// The "ident" and "ty" fields are populated by `darling`
// The others are our attributes for example:
// #[validate(email(message = "asdfg"))]
//            ^^^^^
//
#[derive(Debug, FromField, Clone)]
#[darling(attributes(validate))]
struct ValidateField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    credit_card: Option<Override<Card>>,
    contains: Option<Contains>,
    does_not_contain: Option<DoesNotContain>,
    email: Option<Override<Email>>,
    ip: Option<Override<Ip>>,
    length: Option<Length>,
    must_match: Option<MustMatch>,
    non_control_character: Option<Override<NonControlCharacter>>,
    range: Option<Range>,
    required: Option<Override<Required>>,
    required_nested: Option<Override<Required>>,
    url: Option<Override<Url>>,
    regex: Option<Regex>,
    custom: Option<Custom>,
}

// The field gets converted to tokens in the same format as it was before
impl ToTokens for ValidateField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let field_name = self.ident.clone().unwrap();
        let field_name_str = self.ident.clone().unwrap().to_string();

        // Length validation
        let length = if let Some(length) = self.length.clone() {
            length_tokens(length, &field_name, &field_name_str)
        } else {
            quote!()
        };

        // Email validation
        let email = if let Some(email) = self.email.clone() {
            email_tokens(
                match email {
                    Override::Inherit => Email::default(),
                    Override::Explicit(e) => e,
                },
                &field_name,
                &field_name_str,
            )
        } else {
            quote!()
        };

        // Credit card validation
        let card = if let Some(credit_card) = self.credit_card.clone() {
            credit_card_tokens(
                match credit_card {
                    Override::Inherit => Card::default(),
                    Override::Explicit(c) => c,
                },
                &field_name,
                &field_name_str,
            )
        } else {
            quote!()
        };

        // Url validation
        let url = if let Some(url) = self.url.clone() {
            url_tokens(
                match url {
                    Override::Inherit => Url::default(),
                    Override::Explicit(u) => u,
                },
                &field_name,
                &field_name_str,
            )
        } else {
            quote!()
        };

        // Ip address validation
        let ip = if let Some(ip) = self.ip.clone() {
            ip_tokens(
                match ip {
                    Override::Inherit => Ip::default(),
                    Override::Explicit(i) => i,
                },
                &field_name,
                &field_name_str,
            )
        } else {
            quote!()
        };

        // Non control character validation
        let ncc = if let Some(ncc) = self.non_control_character.clone() {
            non_control_char_tokens(
                match ncc {
                    Override::Inherit => NonControlCharacter::default(),
                    Override::Explicit(n) => n,
                },
                &field_name,
                &field_name_str,
            )
        } else {
            quote!()
        };

        // Range validation
        let range = if let Some(range) = self.range.clone() {
            range_tokens(range, &field_name, &field_name_str)
        } else {
            quote!()
        };

        // Required validation
        let required = if let Some(required) = self.required.clone() {
            required_tokens(
                match required {
                    Override::Inherit => Required::default(),
                    Override::Explicit(r) => r,
                },
                &field_name,
                &field_name_str,
            )
        } else {
            quote!()
        };

        // Required nested validation
        let required_nested = if let Some(required_nested) = self.required_nested.clone() {
            required_nested_tokens(
                match required_nested {
                    Override::Inherit => Required::default(),
                    Override::Explicit(r) => r,
                },
                &field_name,
                &field_name_str,
            )
        } else {
            quote!()
        };

        // Contains validation
        let contains = if let Some(contains) = self.contains.clone() {
            contains_tokens(contains, &field_name, &field_name_str)
        } else {
            quote!()
        };

        // Does not contain validation
        let does_not_contain = if let Some(does_not_contain) = self.does_not_contain.clone() {
            does_not_contain_tokens(does_not_contain, &field_name, &field_name_str)
        } else {
            quote!()
        };

        // Must match validation
        let must_match = if let Some(must_match) = self.must_match.clone() {
            must_match_tokens(must_match, &field_name, &field_name_str)
        } else {
            quote!()
        };

        // Regex validation
        let regex = if let Some(regex) = self.regex.clone() {
            regex_tokens(regex, &field_name, &field_name_str)
        } else {
            quote!()
        };

        // Custom validation
        let custom = if let Some(custom) = self.custom.clone() {
            custom_tokens(custom, &field_name, &field_name_str)
        } else {
            quote!()
        };

        tokens.extend(quote! {
            #length
            #email
            #card
            #url
            #ip
            #ncc
            #range
            #required
            #required_nested
            #contains
            #does_not_contain
            #must_match
            #regex
            #custom
        });
    }
}

// The main struct we get from parsing the attributes
// The "supports(struct_named)" should guarantee to only have this
// macro work with structs with named fields I think?
#[derive(FromDeriveInput)]
#[darling(attributes(validate), supports(struct_named))]
struct ValidationData {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<(), ValidateField>,
}

#[proc_macro_derive(Validate, attributes(validate))]
#[proc_macro_error]
pub fn derive_validation(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    // Parse the input to the ValidationData struct defined above
    let validation_data = match ValidationData::from_derive_input(&input) {
        Ok(data) => data,
        Err(e) => return e.write_errors().into(),
    };

    // Get all the fields to quote them below
    let validation_field = validation_data.data.take_struct().unwrap().fields;

    let args = validation_field.iter().fold(vec![], |mut acc, f| {
        if let Some(c) = &f.custom {
            acc.extend(c.arg.clone());
            acc
        } else {
            acc
        }
    });

    let args_ident: Vec<Ident> = args.iter().map(|a| a.ident()).collect();

    let ident = validation_data.ident;
    let (imp, ty, gen) = validation_data.generics.split_for_impl();

    let arg_struct_name = format_ident!("{}{}", ident, "Args");

    if args.is_empty() {
        quote! {
            impl #imp ::validator::Validate for #ident #ty #gen {
                fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
                    let mut errors = ::validator::ValidationErrors::new();

                    #(#validation_field)*

                    if errors.is_empty() {
                        ::std::result::Result::Ok(())
                    } else {
                        ::std::result::Result::Err(errors)
                    }
                }
            }
        }
        .into()
    } else {
        quote!(
            pub struct #arg_struct_name {
                #(pub #args,)*
            }

            impl #imp ::validator::ValidateArgs for #ident #ty #gen {
                type Args = #arg_struct_name;

                fn validate(&self, args: Self::Args) -> Result<(), ::validator::ValidationErrors> {
                    let mut errors = ::validator::ValidationErrors::new();

                    let #arg_struct_name { #(#args_ident: #args_ident, )* } = args;

                    #(#validation_field)*

                    if errors.is_empty() {
                        ::std::result::Result::Ok(())
                    } else {
                        ::std::result::Result::Err(errors)
                    }
                }
            }
        )
        .into()
    }
}
