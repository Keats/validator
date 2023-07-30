use quote::quote;
use syn::Ident;

use crate::types::Ip;
use crate::utils::{quote_code, quote_message};

pub fn ip_tokens(ip: Ip, field_name: &Ident, field_name_str: &str) -> proc_macro2::TokenStream {
    let message = quote_message(ip.message);
    let code = quote_code(ip.code, "ip");

    let version = if ip.v4 {
        quote!(validate_ipv4())
    } else if ip.v6 {
        quote!(validate_ipv6())
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
