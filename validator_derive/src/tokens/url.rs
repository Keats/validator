use quote::quote;

use crate::types::Url;
use crate::utils::quote_message;

pub fn url_tokens(
    url: Url,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let message = quote_message(url.message);
    let code = url.code.as_deref().unwrap_or("url");

    (
        quote! {
            if !#field_name.validate_url() {
                let mut err = ::validator::ValidationError::new(#code);
                #message
                err.add_param(::std::borrow::Cow::from("value"), &#field_name);
                errors.add(#field_name_str, err);
            }
        },
        quote! {
            constraints.add(
                #field_name_str,
                ::validator::ValidationConstraint::Url {
                    code: #code.into(),
                },
            );
        },
    )
}
