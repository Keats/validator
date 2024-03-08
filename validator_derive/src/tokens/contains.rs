use quote::quote;

use crate::types::Contains;
use crate::utils::quote_message;

pub fn contains_tokens(
    contains: Contains,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let p = contains.pattern;
    let (needle, needle_err) =
        (quote!(#p), quote!(err.add_param(::std::borrow::Cow::from("needle"), &#p);));

    let message = quote_message(contains.message);
    let code = contains.code.as_deref().unwrap_or("contains");

    (
        quote! {
            if !#field_name.validate_contains(#needle) {
                let mut err = ::validator::ValidationError::new(#code);
                #message
                #needle_err
                err.add_param(::std::borrow::Cow::from("value"), &#field_name);
                errors.add(#field_name_str, err);
            }
        },
        quote! {
            constraints.add(
                #field_name_str,
                ::validator::ValidationConstraint::Contains {
                    code: #code.into(),
                    pattern: #p.into(),
                },
            );
        },
    )
}
