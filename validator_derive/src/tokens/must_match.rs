use proc_macro_error3::abort;
use quote::quote;
use syn::spanned::Spanned;

use crate::types::MustMatch;
use crate::utils::{CrateName, quote_code, quote_message};

pub fn must_match_tokens(
    crate_name: &CrateName,
    must_match: MustMatch,
    field_ref: &proc_macro2::TokenStream,
    field_name_str: &str,
    other_number_options: u8,
) -> proc_macro2::TokenStream {
    let o = must_match.other;
    let match_check = match other_number_options {
        0 => quote!(#crate_name::validate_must_match(#field_ref, &self.#o)),
        1 => quote!(#crate_name::validate_must_match(
            ::std::option::Option::Some(#field_ref),
            self.#o.as_ref(),
        )),
        2 => quote!(#crate_name::validate_must_match(
            ::std::option::Option::Some(#field_ref),
            self.#o.as_ref().and_then(::std::option::Option::as_ref),
        )),
        _ => {
            abort!(o.span(), "Validation on values nested in more than 2 Option are not supported")
        }
    };
    let other_err = quote!(err.add_param(::std::borrow::Cow::from("other"), &self.#o););

    let message = quote_message(must_match.message);
    let code = quote_code(crate_name, must_match.code, "must_match");

    quote! {
        if !#match_check {
            #code
            #message
            #other_err
            err.add_param(::std::borrow::Cow::from("value"), #field_ref);
            errors.add(#field_name_str, err);
        }
    }
}
