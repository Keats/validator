use quote::quote;

use crate::types::Card;
use crate::utils::{quote_code, quote_message};

pub fn credit_card_tokens(
    credit_card: Card,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let message = quote_message(credit_card.message);
    let code = quote_code(credit_card.code, "credit_card");

    quote! {
        if !#field_name.validate_credit_card() {
            #code
            #message
            err.add_param(::std::borrow::Cow::from("value"), &#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
