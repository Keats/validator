use proc_macro2;
use syn;

pub fn lit_to_string(lit: &syn::Lit) -> Option<String> {
    match *lit {
        syn::Lit::Str(ref s) => Some(s.value()),
        _ => None,
    }
}

pub fn lit_to_int(lit: &syn::Lit) -> Option<u64> {
    match *lit {
        syn::Lit::Int(ref s) => Some(s.value()),
        // TODO: remove when attr_literals is stable
        syn::Lit::Str(ref s) => Some(s.value().parse::<u64>().unwrap()),
        _ => None,
    }
}

pub fn lit_to_float(lit: &syn::Lit) -> Option<f64> {
    match *lit {
        syn::Lit::Float(ref s) => Some(s.value()),
        syn::Lit::Int(ref s) => Some(s.value() as f64),
        // TODO: remove when attr_literals is stable
        syn::Lit::Str(ref s) => Some(s.value().parse::<f64>().unwrap()),
        _ => None,
    }
}

pub fn lit_to_bool(lit: &syn::Lit) -> Option<bool> {
    match *lit {
        syn::Lit::Bool(ref s) => Some(s.value),
        // TODO: remove when attr_literals is stable
        syn::Lit::Str(ref s) => if s.value() == "true" {
            Some(true)
        } else {
            Some(false)
        },
        _ => None,
    }
}

pub fn option_u64_to_tokens(opt: Option<u64>) -> proc_macro2::TokenStream {
    match opt {
        Some(ref t) => quote!(::std::option::Option::Some(#t)),
        None => quote!(::std::option::Option::None),
    }
}
