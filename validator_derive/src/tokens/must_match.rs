use quote::quote;

use crate::types::MustMatch;
use crate::utils::quote_message;

pub fn must_match_tokens(
    must_match: MustMatch,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let other = must_match.other;
    let other_str = other.get_ident().unwrap().to_string();

    let message = quote_message(must_match.message);
    let code = must_match.code.as_deref().unwrap_or("must_match");

    (
        quote! {
            if !::validator::validate_must_match(&#field_name, &self.#other) {
                let mut err = ::validator::ValidationError::new(#code);
                #message
                err.add_param(::std::borrow::Cow::from("other"), &self.#other);
                err.add_param(::std::borrow::Cow::from("value"), &#field_name);
                errors.add(#field_name_str, err);
            }
        },
        quote! {
            constraints.add(
                #field_name_str,
                ::validator::ValidationConstraint::MustMatch {
                    code: #code.into(),
                    other: #other_str.into(),
                },
            );
        },
    )
}
