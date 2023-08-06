use quote::{format_ident, quote};
use syn::Ident;

use crate::types::Schema;
use crate::utils::quote_message;

pub fn schema_tokens(
    schema: Schema,
    field_name: &Ident,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let closure = format_ident!("{}_closure", field_name);

    let message = quote_message(schema.message);

    let code = if let Some(c) = schema.code {
        quote!(
            err.code = ::std::borrow::Cow::from(#c);
        )
    } else {
        quote!()
    };

    quote! {
        match #closure(self.#field_name.clone()) {
            ::std::result::Result::Ok(()) => {}
            ::std::result::Result::Err(mut err) => {
                #code
                #message
                err.add_param(::std::borrow::Cow::from("value"), &self.#field_name);
                errors.add(#field_name_str, err);
            }
        }
    }
}
