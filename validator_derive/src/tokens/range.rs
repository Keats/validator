use quote::quote;

use crate::types::Range;
use crate::utils::quote_message;

pub fn range_tokens(
    range: Range,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let (min, min_err, min_constraint) = if let Some(m) = range.min {
        (
            quote!(Some(#m)),
            quote!(err.add_param(::std::borrow::Cow::from("min"), &#m);),
            quote! { Some(#m.to_string().into()) },
        )
    } else {
        (quote!(None), quote!(), quote! { None })
    };

    let (max, max_err, max_constraint) = if let Some(m) = range.max {
        (
            quote!(Some(#m)),
            quote!(err.add_param(::std::borrow::Cow::from("max"), &#m);),
            quote! { Some(#m.to_string().into()) },
        )
    } else {
        (quote!(None), quote!(), quote! { None })
    };

    let (ex_min, ex_min_err, ex_min_constraint) = if let Some(m) = range.exclusive_min {
        (
            quote!(Some(#m)),
            quote!(err.add_param(::std::borrow::Cow::from("exclusive_min"), &#m);),
            quote! { Some(#m.to_string().into()) },
        )
    } else {
        (quote!(None), quote!(), quote! { None })
    };

    let (ex_max, ex_max_err, ex_max_constraint) = if let Some(m) = range.exclusive_max {
        (
            quote!(Some(#m)),
            quote!(err.add_param(::std::borrow::Cow::from("exclusive_max"), &#m);),
            quote! { Some(#m.to_string().into()) },
        )
    } else {
        (quote!(None), quote!(), quote! { None })
    };

    let message = quote_message(range.message);
    let code = range.code.as_deref().unwrap_or("range");

    (
        quote! {
            if !#field_name.validate_range(#min, #max, #ex_min, #ex_max) {
                let mut err = ::validator::ValidationError::new(#code);
                #message
                #min_err
                #max_err
                #ex_min_err
                #ex_max_err
                err.add_param(::std::borrow::Cow::from("value"), &#field_name);
                errors.add(#field_name_str, err);
            }
        },
        quote! {
            constraints.add(
                #field_name_str,
                ::validator::ValidationConstraint::Range {
                    code: #code.into(),
                    min: #min_constraint,
                    max: #max_constraint,
                    exclusive_min: #ex_min_constraint,
                    exclusive_max: #ex_max_constraint,
                },
            );
        },
    )
}
