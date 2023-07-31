use quote::quote;
use syn::Ident;

use crate::types::DoesNotContain;
use crate::utils::{quote_code, quote_message};

pub fn does_not_contain_tokens(
    does_not_contain: DoesNotContain,
    field_name: &Ident,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let (needle, needle_err) = if let Some(v) = does_not_contain.pattern {
        (quote!(#v), quote!(err.add_param(::std::borrow::Cow::from("needle"), &#v);))
    } else {
        (quote!(None), quote!())
    };

    let message = quote_message(does_not_contain.message);
    let code = quote_code(does_not_contain.code, "does_not_contain");

    quote! {
        use ::validator::ValidateDoesNotContain;
        if !self.#field_name.validate_does_not_contain(#needle) {
            #code
            #message
            #needle_err
            err.add_param(::std::borrow::Cow::from("value"), &self.#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
