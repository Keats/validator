use quote::quote;

use crate::types::Ip;
use crate::utils::{quote_code, quote_message};

pub fn ip_tokens(
    ip: Ip,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let message = quote_message(ip.message);
    let code = quote_code(ip.code, "ip");

    let version = match (ip.v4, ip.v6) {
        (Some(v4), Some(v6)) => match (v4, v6) {
            (true, false) => quote!(validate_ipv4()),
            (false, true) => quote!(validate_ipv6()),
            _ => quote!(validate_ip()),
        },
        (Some(v4), None) => {
            if v4 {
                quote!(validate_ipv4())
            } else {
                quote!(validate_ip())
            }
        }
        (None, Some(v6)) => {
            if v6 {
                quote!(validate_ipv6())
            } else {
                quote!(validate_ip())
            }
        }
        _ => quote!(validate_ip()),
    };

    quote! {
        if !#field_name.#version {
            #code
            #message
            err.add_param(::std::borrow::Cow::from("value"), &#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
