use quote::quote;

pub fn quote_message(message: Option<String>) -> proc_macro2::TokenStream {
    if let Some(m) = message {
        quote!(
            err.message = Some(::std::borrow::Cow::from(#m));
        )
    } else {
        quote!()
    }
}

pub fn quote_code(code: Option<String>, default: &str) -> proc_macro2::TokenStream {
    if let Some(c) = code {
        quote!(
            let mut err = ::validator::ValidationError::new(#c);
        )
    } else {
        quote!(
            let mut err = ::validator::ValidationError::new(#default);
        )
    }
}
