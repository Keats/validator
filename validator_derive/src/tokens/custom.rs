use quote::{quote, ToTokens};

use crate::types::Custom;
use crate::utils::quote_message;

pub fn custom_tokens(
    custom: Custom,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let fn_call = custom.function.unwrap();
    let fn_str = fn_call.to_token_stream().to_string();

    let args = if let Some(arg) = custom.use_context {
        if arg {
            quote!(#field_name, args)
        } else {
            quote!(#field_name)
        }
    } else {
        quote!(#field_name)
    };

    let message = quote_message(custom.message);

    let (code, code_constraint) = if let Some(c) = custom.code {
        (
            quote!(
                err.code = ::std::borrow::Cow::from(#c);
            ),
            quote! { Some(#c.into()) },
        )
    } else {
        (quote!(), quote! { None })
    };

    (
        quote! {
            match #fn_call(#args) {
                ::std::result::Result::Ok(()) => {}
                ::std::result::Result::Err(mut err) => {
                    #code
                    #message
                    err.add_param(::std::borrow::Cow::from("value"), &#field_name);
                    errors.add(#field_name_str, err);
                }
            }
        },
        quote! {
            constraints.add(
                #field_name_str,
                ::validator::ValidationConstraint::Custom {
                    code: #code_constraint,
                    function: #fn_str.into(),
                },
            );
        },
    )
}
