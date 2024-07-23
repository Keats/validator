use quote::quote;
use syn::Ident;

use crate::types::Required;
use crate::utils::quote_message;

pub fn required_tokens(
    required: Required,
    field_name: &Ident,
    field_name_str: &str,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let message = quote_message(required.message);
    let code = required.code.as_deref().unwrap_or("required");

    (
        quote! {
            if !self.#field_name.validate_required() {
                let mut err = ::validator::ValidationError::new(#code);
                #message
                err.add_param(::std::borrow::Cow::from("value"), &self.#field_name);
                errors.add(#field_name_str, err);
            }
        },
        quote! {
            constraints.add(
                #field_name_str,
                ::validator::ValidationConstraint::Required {
                    code: #code.into(),
                },
            );
        },
    )
}
