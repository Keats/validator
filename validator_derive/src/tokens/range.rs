use quote::quote;

use crate::types::Range;
use crate::utils::{quote_code, quote_message};

pub fn range_tokens(
    range: Range,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let (min, min_err) = if let Some(m) = range.min {
        (quote!(Some(#m)), quote!(err.add_param(::std::borrow::Cow::from("min"), &#m);))
    } else {
        (quote!(None), quote!())
    };

    let (max, max_err) = if let Some(m) = range.max {
        (quote!(Some(#m)), quote!(err.add_param(::std::borrow::Cow::from("max"), &#m);))
    } else {
        (quote!(None), quote!())
    };

    let (ex_min, ex_min_err) = if let Some(m) = range.exclusive_min {
        (quote!(Some(#m)), quote!(err.add_param(::std::borrow::Cow::from("exclusive_min"), &#m);))
    } else {
        (quote!(None), quote!())
    };

    let (ex_max, ex_max_err) = if let Some(m) = range.exclusive_max {
        (quote!(Some(#m)), quote!(err.add_param(::std::borrow::Cow::from("exclusive_max"), &#m);))
    } else {
        (quote!(None), quote!())
    };

    let message = quote_message(range.message);
    let code = quote_code(range.code, "range");

    quote! {
        if !#field_name.validate_range(#min, #max, #ex_min, #ex_max) {
            #code
            #message
            #min_err
            #max_err
            #ex_min_err
            #ex_max_err
            err.add_param(::std::borrow::Cow::from("value"), &#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
