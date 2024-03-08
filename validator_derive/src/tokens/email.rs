use quote::quote;

use crate::types::Email;
use crate::utils::quote_message;

pub fn email_tokens(
    email: Email,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let message = quote_message(email.message);
    let code = email.code.as_deref().unwrap_or("email");

    (
        quote! {
            if !#field_name.validate_email() {
                let mut err = ::validator::ValidationError::new(#code);
                #message
                err.add_param(::std::borrow::Cow::from("value"), &#field_name);
                errors.add(#field_name_str, err);
            }
        },
        quote! {
            constraints.add(
                #field_name_str,
                ::validator::ValidationConstraint::Email {
                    code: #code.into(),
                },
            );

        },
    )
}
