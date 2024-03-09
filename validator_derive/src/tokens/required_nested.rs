use quote::quote;

use crate::types::Required;
use crate::utils::{quote_code, quote_message};

pub fn required_nested_tokens(
    required: Required,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let message = quote_message(required.message);
    let code = quote_code(required.code, "required");
    // TODO: fix me
    quote! {
        if !#field_name.validate_required() {
            #code
            #message
            err.add_param(::std::borrow::Cow::from("value"), &#field_name);
            errors.add(#field_name_str, err);
        }

        if let Some(ref #field_name) = self.#field_name {
            errors.merge_self(#field_name_str, #field_name.validate());
        }
    }
}
