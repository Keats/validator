use quote::quote;

use crate::types::NonControlCharacter;
use crate::utils::quote_message;

pub fn non_control_char_tokens(
    non_control_char: NonControlCharacter,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let message = quote_message(non_control_char.message);
    let code = non_control_char.code.as_deref().unwrap_or("non_control_character");

    (
        quote! {
            if !#field_name.validate_non_control_character() {
                let mut err = ::validator::ValidationError::new(#code);
                #message
                err.add_param(::std::borrow::Cow::from("value"), &#field_name);
                errors.add(#field_name_str, err);
            }
        },
        quote! {
            constraints.add(
                #field_name_str,
                ::validator::ValidationConstraint::NonControlCharacter {
                    code: #code.into(),
                },
            );
        },
    )
}
