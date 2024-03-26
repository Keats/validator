use quote::quote;

use crate::types::MustMatch;
use crate::utils::{quote_code, quote_message};

pub fn must_match_tokens(
    must_match: MustMatch,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let o = must_match.other;
    let (other, other_err) =
        (quote!(self.#o), quote!(err.add_param(::std::borrow::Cow::from("other"), &self.#o);));

    let message = quote_message(must_match.message);
    let code = quote_code(must_match.code, "must_match");

    quote! {
        if !::validator::validate_must_match(&#field_name, &#other) {
            #code
            #message
            #other_err
            err.add_param(::std::borrow::Cow::from("value"), &#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
