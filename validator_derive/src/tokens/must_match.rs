use quote::quote;
use syn::Ident;

use crate::types::MustMatch;
use crate::utils::{quote_code, quote_message};

pub fn must_match_tokens(
    must_match: MustMatch,
    field_name: &Ident,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let (other, other_err) = if let Some(v) = must_match.other {
        (quote!(self.#v), quote!(err.add_param(::std::borrow::Cow::from("other"), &self.#v);))
    } else {
        (quote!(None), quote!())
    };

    let message = quote_message(must_match.message);
    let code = quote_code(must_match.code, "must_match");

    quote! {
        if !::validator::validate_must_match(&self.#field_name, &#other) {
            #code
            #message
            #other_err
            err.add_param(::std::borrow::Cow::from("value"), &self.#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
