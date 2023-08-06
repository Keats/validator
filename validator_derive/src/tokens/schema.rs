use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use syn::Ident;

use crate::types::Schema;
use crate::utils::quote_message;

pub fn schema_tokens(schema: Schema, field_name: &Ident) -> proc_macro2::TokenStream {
    let closure = format_ident!("{}_schema_closure", field_name.to_string().to_case(Case::Snake));

    let message = quote_message(schema.message);

    let code = if let Some(c) = schema.code {
        quote!(
            err.code = ::std::borrow::Cow::from(#c);
        )
    } else {
        quote!()
    };

    quote! {
        match #closure(&self) {
            ::std::result::Result::Ok(()) => {}
            ::std::result::Result::Err(mut err) => {
                #code
                #message
                errors.add("__all__", err);
            }
        }
    }
}
