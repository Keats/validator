use quote::quote;
use syn::{Expr, Lit};

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

pub fn ident_from_expr(expr: &syn::Expr) -> syn::Ident {
    match expr {
        Expr::Reference(r) => match &r.expr.as_ref() {
            Expr::Path(p) => p.path.segments.first().unwrap().ident.clone(),
            _ => todo!(),
        },
        Expr::Path(p) => p.path.segments.first().unwrap().ident.clone(),
        _ => todo!(),
    }
}

pub fn ident_from_path(path: &syn::Path) -> syn::Ident {
    path.segments.first().unwrap().ident.clone()
}

pub fn lit_to_str(lit: &Lit) -> String {
    match lit {
        Lit::Str(lit_str) => lit_str.value(),
        _ => todo!(),
    }
}
