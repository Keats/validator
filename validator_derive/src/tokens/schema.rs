use quote::quote;

use crate::types::Schema;
use crate::utils::quote_message;

pub fn schema_tokens(schema: Schema) -> proc_macro2::TokenStream {
    let fn_call = schema.function;
    let args = if let Some(args) = schema.use_context {
        if args {
            quote!(&self, args)
        } else {
            quote!(&self)
        }
    } else {
        quote!(&self)
    };

    let message = quote_message(schema.message);

    let code = if let Some(c) = schema.code {
        quote!(
            err.code = ::std::borrow::Cow::from(#c);
        )
    } else {
        quote!()
    };

    quote! {
        match #fn_call(#args) {
            ::std::result::Result::Ok(()) => {}
            ::std::result::Result::Err(mut err) => {
                #code
                #message
                errors.add("__all__", err);
            }
        }
    }
}
