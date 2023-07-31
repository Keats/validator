use quote::quote;
use syn::Ident;

use crate::types::Contains;
use crate::utils::{quote_code, quote_message};

pub fn contains_tokens(
    contains: Contains,
    field_name: &Ident,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let (needle, needle_err) = if let Some(v) = contains.pattern {
        (quote!(#v), quote!(err.add_param(::std::borrow::Cow::from("needle"), &#v);))
    } else {
        (quote!(None), quote!())
    };

    let message = quote_message(contains.message);
    let code = quote_code(contains.code, "contains");

    quote! {
        use ::validator::ValidateContains;
        if !self.#field_name.validate_contains(#needle) {
            #code
            #message
            #needle_err
            err.add_param(::std::borrow::Cow::from("value"), &self.#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
