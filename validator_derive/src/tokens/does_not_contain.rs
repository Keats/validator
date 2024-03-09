use quote::quote;

use crate::types::DoesNotContain;
use crate::utils::{quote_code, quote_message};

pub fn does_not_contain_tokens(
    does_not_contain: DoesNotContain,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let p = does_not_contain.pattern;

    let (needle, needle_err) =
        (quote!(#p), quote!(err.add_param(::std::borrow::Cow::from("needle"), &#p);));

    let message = quote_message(does_not_contain.message);
    let code = quote_code(does_not_contain.code, "does_not_contain");

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
