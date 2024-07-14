use quote::quote;

use crate::types::DoesNotContain;
use crate::utils::{quote_code, quote_message};
use crate::CrateName;

pub fn does_not_contain_tokens(
    crate_name: &CrateName,
    does_not_contain: DoesNotContain,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let p = does_not_contain.pattern;

    let (needle, needle_err) =
        (quote!(#p), quote!(err.add_param(::std::borrow::Cow::from("needle"), &#p);));

    let message = quote_message(does_not_contain.message);
    let code = quote_code(crate_name, does_not_contain.code, "does_not_contain");

    quote! {
        if !#field_name.validate_does_not_contain(#needle) {
            #code
            #message
            #needle_err
            err.add_param(::std::borrow::Cow::from("value"), &#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
