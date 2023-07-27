// #![recursion_limit = "128"]
#![allow(unused)]

use darling::ast::Data;
use darling::util::Override;
use darling::{FromDeriveInput, FromField, FromMeta};
use proc_macro_error::proc_macro_error;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Expr};

use tokens::email::email_tokens;
use tokens::length::length_tokens;
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
#[derive(Debug, FromField)]
#[darling(attributes(validate))]
struct ValidateField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    card: Option<Card>,
    contains: Option<Contains>,
    does_not_contain: Option<DoesNotContain>,
    email: Override<Email>,
    ip: Option<Ip>,
    length: Option<Length>,
    must_match: Option<MustMatch>,
    non_control_character: Option<NonControlCharacter>,
    range: Option<Range>,
    required: Option<Required>,
    url: Option<Url>,
}

// The field gets converted to tokens in the same format as it was before
impl ToTokens for ValidateField {
    // Move all the token generation to seperate functions in seperate modules?
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let field_name = self.ident.clone().unwrap();
        let field_name_str = self.ident.clone().unwrap().to_string();

        let length = length_tokens(self.length.clone(), &field_name, &field_name_str);
        let email = email_tokens(
            match self.email.clone() {
                Override::Inherit => Some(Email::default()),
                Override::Explicit(email) => Some(email),
            },
            &field_name,
            &field_name_str,
        );

        tokens.extend(quote! {
            #length
            #email
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

    let ident = validation_data.ident;
    let (imp, ty, gen) = validation_data.generics.split_for_impl();

    quote! {
        impl #imp ::validator::Validate for #ident #ty #gen {
            fn validate (&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
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
}
