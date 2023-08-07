use quote::quote;
use syn::Ident;

use crate::types::Email;
use crate::utils::{quote_code, quote_message};

pub fn email_tokens(
    email: Email,
    field_name: &Ident,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let message = quote_message(email.message);
    let code = quote_code(email.code, "email");

    quote! {
        if !self.#field_name.validate_email() {
            #code
            #message
            err.add_param(::std::borrow::Cow::from("value"), &self.#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
