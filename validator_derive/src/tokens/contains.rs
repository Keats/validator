use quote::quote;

use crate::types::Contains;
use crate::utils::{quote_code, quote_message};
use crate::CrateName;

pub fn contains_tokens(
    crate_name: &CrateName,
    contains: Contains,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let p = contains.pattern;
    let (needle, needle_err) =
        (quote!(#p), quote!(err.add_param(::std::borrow::Cow::from("needle"), &#p);));

    let message = quote_message(contains.message);
    let code = quote_code(crate_name, contains.code, "contains");

    quote! {
        if !#field_name.validate_contains(#needle) {
            #code
            #message
            #needle_err
            err.add_param(::std::borrow::Cow::from("value"), &#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
