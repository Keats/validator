use quote::{quote, ToTokens};

use crate::types::Regex;
use crate::utils::quote_message;

pub fn regex_tokens(
    regex: Regex,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let path = regex.path;
    let path_str = path.to_token_stream().to_string();
    let message = quote_message(regex.message);
    let code = regex.code.as_deref().unwrap_or("regex");

    (
        quote! {
            if !&#field_name.validate_regex(&#path) {
                let mut err = ::validator::ValidationError::new(#code);
                #message
                err.add_param(::std::borrow::Cow::from("value"), &#field_name);
                errors.add(#field_name_str, err);
            }
        },
        quote! {
            constraints.add(
                #field_name_str,
                ::validator::ValidationConstraint::Regex {
                    code: #code.into(),
                    path: #path_str.into(),
                },
            );
        },
    )
}
