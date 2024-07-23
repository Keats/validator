use quote::quote;

use crate::types::Card;
use crate::utils::quote_message;

pub fn credit_card_tokens(
    credit_card: Card,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let message = quote_message(credit_card.message);
    let code = credit_card.code.as_deref().unwrap_or("credit_card");

    (
        quote! {
            if !#field_name.validate_credit_card() {
                let mut err = ::validator::ValidationError::new(#code);
                #message
                err.add_param(::std::borrow::Cow::from("value"), &#field_name);
                errors.add(#field_name_str, err);
            }
        },
        quote! {
            constraints.add(
                #field_name_str,
                ::validator::ValidationConstraint::CreditCard {
                    code: #code.into(),
                },
            );
        },
    )
}
