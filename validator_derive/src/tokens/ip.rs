use quote::quote;

use crate::types::Ip;
use crate::utils::quote_message;

pub fn ip_tokens(
    ip: Ip,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let message = quote_message(ip.message);
    let code = ip.code.as_deref().unwrap_or("ip");

    let v4 = ip.v4.unwrap_or_default();
    let v6 = ip.v6.unwrap_or_default();

    let version = match (v4, v6) {
        (true, false) => quote!(validate_ipv4()),
        (false, true) => quote!(validate_ipv6()),
        _ => quote!(validate_ip()),
    };

    (
        quote! {
            if !#field_name.#version {
                let mut err = ::validator::ValidationError::new(#code);
                #message
                err.add_param(::std::borrow::Cow::from("value"), &#field_name);
                errors.add(#field_name_str, err);
            }
        },
        quote! {
            constraints.add(
                #field_name_str,
                ::validator::ValidationConstraint::Ip {
                    code: #code.into(),
                    v4: #v4,
                    v6: #v6,
                },
            );
        },
    )
}
