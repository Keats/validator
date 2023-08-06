use convert_case::{Case, Casing};
use quote::{format_ident, quote, ToTokens};
use syn::Ident;

use crate::types::Custom;
use crate::utils::{ident_from_expr, ident_from_path, lit_to_str, quote_code, quote_message};

pub fn custom_tokens(
    custom: Custom,
    field_name: &Ident,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let closure = format_ident!("{}_closure", field_name);

    let message = quote_message(custom.message);

    let code = if let Some(c) = custom.code {
        quote!(
            err.code = ::std::borrow::Cow::from(#c);
        )
    } else {
        quote!()
    };

    quote! {
        match #closure(self.#field_name.clone()) {
            ::std::result::Result::Ok(()) => {}
            ::std::result::Result::Err(mut err) => {
                #code
                #message
                err.add_param(::std::borrow::Cow::from("value"), &self.#field_name);
                errors.add(#field_name_str, err);
            }
        }
    }
}
