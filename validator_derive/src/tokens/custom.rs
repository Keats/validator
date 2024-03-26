use quote::quote;

use crate::types::Custom;
use crate::utils::quote_message;

pub fn custom_tokens(
    custom: Custom,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let fn_call = custom.function.unwrap();

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

    let code = if let Some(c) = custom.code {
        quote!(
            err.code = ::std::borrow::Cow::from(#c);
        )
    } else {
        quote!()
    };

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
    }
}
