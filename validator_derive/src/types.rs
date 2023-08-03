use darling::{Error, FromMeta};
use quote::ToTokens;
use syn::{Expr, FnArg, Lit, TypeParam};

// Structs to hold the validation information and to provide attributes
// The name of a field here corresponds to an attribute like
// #[validate(card(message = "something's wrong", code = "1234"))]
//                 ^^^^^^^                        ^^^^
//
#[derive(Debug, Clone, FromMeta, Default)]
pub struct Card {
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta)]
pub struct Contains {
    pub pattern: Option<String>,
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta)]
pub struct DoesNotContain {
    pub pattern: Option<String>,
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta, Default)]
pub struct Email {
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta, Default)]
pub struct Ip {
    pub v4: Option<bool>,
    pub v6: Option<bool>,
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta)]
pub struct Length {
    pub min: Option<Expr>,
    pub max: Option<Expr>,
    pub equal: Option<Expr>,
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta)]
pub struct MustMatch {
    pub other: Option<Expr>,
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta, Default)]
pub struct NonControlCharacter {
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta)]
pub struct Range {
    pub min: Option<Expr>,
    pub max: Option<Expr>,
    pub exclusive_min: Option<Expr>,
    pub exclusive_max: Option<Expr>,
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta, Default)]
pub struct Required {
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta, Default)]
pub struct Url {
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta)]
pub struct Regex {
    pub path: Expr,
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta)]
pub struct Custom {
    pub function: Expr,
    #[darling(multiple)]
    pub arg: Vec<Arg>,
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Arg(FnArg);

impl Arg {
    pub fn ident(&self) -> syn::Ident {
        match self.0.clone() {
            FnArg::Typed(t) => match *t.pat {
                syn::Pat::Ident(i) => i.ident,
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

impl FromMeta for Arg {
    fn from_string(value: &str) -> darling::Result<Self> {
        if let Ok(fn_arg) = syn::parse_str::<FnArg>(value) {
            darling::Result::Ok(Arg(fn_arg))
        } else {
            darling::Result::Err(darling::Error::unknown_value(value))
        }
    }
}

impl ToTokens for Arg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens)
    }
}
