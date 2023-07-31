use quote::quote;
use syn::Ident;

use crate::types::Required;
use crate::utils::{quote_code, quote_message};

pub fn required_nested_tokens(
    required: Required,
    field_name: &Ident,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let message = quote_message(required.message);
    let code = quote_code(required.code, "required");

    quote! {
        use ::validator::ValidateRequired;
        if !self.#field_name.validate_required() {
            #code
            #message
            err.add_param(::std::borrow::Cow::from("value"), &self.#field_name);
            errors.add(#field_name_str, err);
        }

        if let Some(ref #field_name) = self.#field_name {
            errors.merge(#field_name_str, #field_name.validate());
        }
    }
}
