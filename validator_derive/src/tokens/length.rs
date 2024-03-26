use quote::quote;

use crate::types::Length;
use crate::utils::{quote_code, quote_message};

pub fn length_tokens(
    length: Length,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let (min, min_err) = if let Some(v) = length.min.as_ref() {
        (quote!(Some(#v)), quote!(err.add_param(::std::borrow::Cow::from("min"), &#v);))
    } else {
        (quote!(None), quote!())
    };
    let (max, max_err) = if let Some(v) = length.max {
        (quote!(Some(#v)), quote!(err.add_param(::std::borrow::Cow::from("max"), &#v);))
    } else {
        (quote!(None), quote!())
    };
    let (equal, equal_err) = if let Some(v) = length.equal {
        (quote!(Some(#v)), quote!(err.add_param(::std::borrow::Cow::from("equal"), &#v);))
    } else {
        (quote!(None), quote!())
    };

    let message = quote_message(length.message);
    let code = quote_code(length.code, "length");

    quote! {
        if !#field_name.validate_length(#min, #max, #equal) {
            #code
            #message
            #min_err
            #max_err
            #equal_err
            err.add_param(::std::borrow::Cow::from("value"), &#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
