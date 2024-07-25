use quote::quote;

use crate::types::Regex;
use crate::utils::{quote_code, quote_message, CrateName};

pub fn regex_tokens(
    crate_name: &CrateName,
    regex: Regex,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let path = regex.path;
    let message = quote_message(regex.message);
    let code = quote_code(crate_name, regex.code, "regex");

    quote! {
        if !&#field_name.validate_regex(&#path) {
            #code
            #message
            err.add_param(::std::borrow::Cow::from("value"), &#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
