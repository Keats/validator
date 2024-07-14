use quote::quote;

use crate::types::Email;
use crate::utils::{quote_code, quote_message};
use crate::CrateName;

pub fn email_tokens(
    crate_name: &CrateName,
    email: Email,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let message = quote_message(email.message);
    let code = quote_code(crate_name, email.code, "email");

    quote! {
        if !#field_name.validate_email() {
            #code
            #message
            err.add_param(::std::borrow::Cow::from("value"), &#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
