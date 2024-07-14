use quote::quote;
use syn::Attribute;

use crate::{CrateName, ValidateField};

pub fn quote_message(message: Option<String>) -> proc_macro2::TokenStream {
    if let Some(m) = message {
        quote!(
            err.message = Some(::std::borrow::Cow::from(#m));
        )
    } else {
        quote!()
    }
}

pub fn quote_code(
    crate_name: &CrateName,
    code: Option<String>,
    default: &str,
) -> proc_macro2::TokenStream {
    if let Some(c) = code {
        quote!(
            let mut err = ::#crate_name::ValidationError::new(#c);
        )
    } else {
        quote!(
            let mut err = ::#crate_name::ValidationError::new(#default);
        )
    }
}

pub fn quote_use_stmts(
    crate_name: &CrateName,
    fields: &Vec<ValidateField>,
) -> proc_macro2::TokenStream {
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
                use ::#crate_name::ValidateLength;
            );
        }

        if f.email.is_some() {
            email = quote!(
                use ::#crate_name::ValidateEmail;
            );
        }

        if f.credit_card.is_some() {
            card = quote!(
                use ::#crate_name::ValidateCreditCard;
            );
        }

        if f.url.is_some() {
            url = quote!(
                use ::#crate_name::ValidateUrl;
            );
        }

        if f.ip.is_some() {
            ip = quote!(
                use ::#crate_name::ValidateIp;
            );
        }

        if f.non_control_character.is_some() {
            ncc = quote!(
                use ::#crate_name::ValidateNonControlCharacter;
            );
        }

        if f.range.is_some() {
            range = quote!(
                use ::#crate_name::ValidateRange;
            );
        }

        if f.required.is_some() {
            required = quote!(
                use ::#crate_name::ValidateRequired;
            );
        }

        if f.contains.is_some() {
            contains = quote!(
                use ::#crate_name::ValidateContains;
            );
        }

        if f.does_not_contain.is_some() {
            does_not_contain = quote!(
                use ::#crate_name::ValidateDoesNotContain;
            );
        }

        if f.regex.is_some() {
            regex = quote!(
                use ::#crate_name::ValidateRegex;
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
