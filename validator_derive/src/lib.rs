// #![recursion_limit = "128"]
#![allow(unused)]

use darling::ast::Data;
use darling::{FromDeriveInput, FromField, FromMeta};
use proc_macro_error::proc_macro_error;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Expr};

use types::*;

mod types;

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
    email: Option<Email>,
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
        // Length validation tokens
        if let Some(length) = &self.length {
            let (min, min_err) = if let Some(v) = length.min {
                (quote!(Some(#v)), quote!(err.add_param(::std::borrow::Cow::from("min"), &#v);))
            } else {
                (quote!(None), quote!())
            };
            let (max, max_err) = if let Some(v) = length.max {
                (quote!(Some(#v)), quote!(err.add_param(::std::borrow::Cow::from("max"), &#v);))
            } else {
                (quote!(None), quote!())
            };
            let (equal, equal_err) = if let Some(v) = length.equal {
                (quote!(Some(#v)), quote!(err.add_param(::std::borrow::Cow::from("equal"), &#v);))
            } else {
                (quote!(None), quote!())
            };

            let message = if let Some(m) = &length.message {
                quote!(
                    err.message = Some(::std::borrow::Cow::from(#m));
                )
            } else {
                quote!()
            };

            let field_name = self.ident.clone().unwrap();
            let field_name_str = &self.ident.clone().unwrap().to_string();

            // We don't need to use the `validate_length()` function
            // As the type we're validating _should_ already implement this function
            // If it doesn't, a compile error should show up about "T doesn't impl this trait"
            tokens.extend(quote! {
                use ::validator::ValidateLength;
                if !self.#field_name.validate_length(#min, #max, #equal) {
                    let mut err = ::validator::ValidationError::new("length");
                    #message
                    #min_err
                    #max_err
                    #equal_err
                    err.add_param(::std::borrow::Cow::from("value"), &self.#field_name);
                    errors.add(#field_name_str, err);
                }
            })
        }

        // Email validation tokens
        if let Some(email) = &self.email {
            let message = if let Some(m) = &email.message {
                quote!(
                    err.message = Some(::std::borrow::Cow::from(#m));
                )
            } else {
                quote!()
            };

            let field_name = self.ident.clone().unwrap();
            let field_name_str = &self.ident.clone().unwrap().to_string();

            tokens.extend(quote! {
                use ::validator::ValidateEmail;
                if !self.#field_name.validate_email() {
                    let mut err = ::validator::ValidationError::new("email");
                    #message
                    err.add_param(::std::borrow::Cow::from("value"), &self.#field_name);
                    errors.add(#field_name_str, err);
                }
            })
        }

        // To be expanded with all other validations
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
