use quote::quote;

use crate::types::Regex;
use crate::utils::{quote_code, quote_message};

pub fn regex_tokens(
    regex: Regex,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let path = regex.path;
    let message = quote_message(regex.message);
    let code = quote_code(regex.code, "regex");

    quote! {
        if !&#field_name.validate_regex(&#path) {
            #code
            #message
            err.add_param(::std::borrow::Cow::from("value"), &#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
