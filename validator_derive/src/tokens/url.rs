use quote::quote;

use crate::types::Url;
use crate::utils::{quote_code, quote_message, CrateName};

pub fn url_tokens(
    crate_name: &CrateName,
    url: Url,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let message = quote_message(url.message);
    let code = quote_code(crate_name, url.code, "url");

    quote! {
        if !#field_name.validate_url() {
            #code
            #message
            err.add_param(::std::borrow::Cow::from("value"), &#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
