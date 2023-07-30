use quote::quote;
use syn::Ident;

use crate::types::Ip;
use crate::utils::{quote_code, quote_message};

pub fn ip_tokens(ip: Ip, field_name: &Ident, field_name_str: &str) -> proc_macro2::TokenStream {
    let message = quote_message(ip.message);
    let code = quote_code(ip.code, "ip");

    let version = if let (Some(v4), Some(v6)) = (ip.v4, ip.v6) {
        if v4 && v6 {
            quote!(validate_ip())
        } else if v4 {
            quote!(validate_ipv4())
        } else if v6 {
            quote!(validate_ipv6())
        } else {
            quote!(validate_ip())
        }
    } else if let (Some(v4), None) = (ip.v4, ip.v6) {
        if v4 {
            quote!(validate_ipv4())
        } else {
            quote!(validate_ip())
        }
    } else if let (None, Some(v6)) = (ip.v4, ip.v6) {
        if v6 {
            quote!(validate_ipv6())
        } else {
            quote!(validate_ip())
        }
    } else {
        quote!(validate_ip())
    };

    quote! {
        use ::validator::ValidateIp;
        if !self.#field_name.#version {
            #code
            #message
            err.add_param(::std::borrow::Cow::from("value"), &self.#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
