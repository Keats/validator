use quote::quote;

use crate::types::MustMatch;
use crate::utils::{quote_code, quote_message};
use crate::CrateName;

pub fn must_match_tokens(
    crate_name: &CrateName,
    must_match: MustMatch,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let o = must_match.other;
    let (other, other_err) =
        (quote!(self.#o), quote!(err.add_param(::std::borrow::Cow::from("other"), &self.#o);));

    let message = quote_message(must_match.message);
    let code = quote_code(crate_name, must_match.code, "must_match");

    quote! {
        if !#crate_name::validate_must_match(&#field_name, &#other) {
            #code
            #message
            #other_err
            err.add_param(::std::borrow::Cow::from("value"), &#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
