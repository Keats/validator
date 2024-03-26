use quote::quote;
use syn::Attribute;

use crate::ValidateField;

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

pub fn quote_use_stmts(fields: &Vec<ValidateField>) -> proc_macro2::TokenStream {
    let mut length = quote!();
    let mut email = quote!();
    let mut card = quote!();
    let mut url = quote!();
    let mut ip = quote!();
    let mut ncc = quote!();
    let mut range = quote!();
    let mut required = quote!();
    let mut contains = quote!();
    let mut does_not_contain = quote!();
    let mut regex = quote!();

    for f in fields {
        if f.length.is_some() {
            length = quote!(
                use validator::ValidateLength;
            );
        }

        if f.email.is_some() {
            email = quote!(
                use validator::ValidateEmail;
            );
        }

        if f.credit_card.is_some() {
            card = quote!(
                use validator::ValidateCreditCard;
            );
        }

        if f.url.is_some() {
            url = quote!(
                use validator::ValidateUrl;
            );
        }

        if f.ip.is_some() {
            ip = quote!(
                use validator::ValidateIp;
            );
        }

        if f.non_control_character.is_some() {
            ncc = quote!(
                use validator::ValidateNonControlCharacter;
            );
        }

        if f.range.is_some() {
            range = quote!(
                use validator::ValidateRange;
            );
        }

        if f.required.is_some() {
            required = quote!(
                use validator::ValidateRequired;
            );
        }

        if f.contains.is_some() {
            contains = quote!(
                use validator::ValidateContains;
            );
        }

        if f.does_not_contain.is_some() {
            does_not_contain = quote!(
                use validator::ValidateDoesNotContain;
            );
        }

        if f.regex.is_some() {
            regex = quote!(
                use validator::ValidateRegex;
            );
        }
    }

    quote!(
        #length
        #email
        #card
        #url
        #ip
        #ncc
        #range
        #required
        #contains
        #does_not_contain
        #regex
    )
}

pub fn get_attr<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a Attribute> {
    attrs.iter().find(|a| match &a.meta {
        syn::Meta::List(list) => list.tokens.clone().into_iter().any(|t| match t {
            proc_macro2::TokenTree::Ident(i) => i == name,
            _ => false,
        }),
        _ => false,
    })
}
