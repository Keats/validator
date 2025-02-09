use quote::quote;

use crate::types::MustNotMatch;
use crate::utils::{quote_code, quote_message, CrateName};

pub fn must_not_match_tokens(
    crate_name: &CrateName,
    must_not_match: MustNotMatch,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let o = must_not_match.other;
    let (other, other_err) =
        (quote!(self.#o), quote!(err.add_param(::std::borrow::Cow::from("other"), &self.#o);));

    let message = quote_message(must_not_match.message);
    let code = quote_code(crate_name, must_not_match.code, "must_not_match");

    quote! {
        if !#crate_name::validate_must_not_match(&#field_name, &#other) {
            #code
            #message
            #other_err
            err.add_param(::std::borrow::Cow::from("value"), &#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
