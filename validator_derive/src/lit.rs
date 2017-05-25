use quote::{self, ToTokens};
use syn;


pub fn lit_to_string(lit: &syn::Lit) -> Option<String> {
    match *lit {
        syn::Lit::Str(ref s, _) => Some(s.to_string()),
        _ => None,
    }
}

pub fn lit_to_int(lit: &syn::Lit) -> Option<u64> {
    match *lit {
        syn::Lit::Int(ref s, _) => Some(*s),
        // TODO: remove when attr_literals is stable
        syn::Lit::Str(ref s, _) => Some(s.parse::<u64>().unwrap()),
        _ => None,
    }
}

pub fn lit_to_float(lit: &syn::Lit) -> Option<f64> {
    match *lit {
        syn::Lit::Float(ref s, _) => Some(s.parse::<f64>().unwrap()),
        syn::Lit::Int(ref s, _) => Some(*s as f64),
        // TODO: remove when attr_literals is stable
        syn::Lit::Str(ref s, _) => Some(s.parse::<f64>().unwrap()),
        _ => None,
    }
}

pub fn lit_to_bool(lit: &syn::Lit) -> Option<bool> {
    match *lit {
        syn::Lit::Bool(ref s) => Some(*s),
        // TODO: remove when attr_literals is stable
        syn::Lit::Str(ref s, _) => if s == "true" { Some(true) } else { Some(false) },
        _ => None,
    }
}

pub fn option_u64_to_tokens(opt: Option<u64>) -> quote::Tokens {
    let mut tokens = quote::Tokens::new();
    tokens.append("::");
    tokens.append("std");
    tokens.append("::");
    tokens.append("option");
    tokens.append("::");
    tokens.append("Option");
    tokens.append("::");
    match opt {
        Some(ref t) => {
            tokens.append("Some");
            tokens.append("(");
            t.to_tokens(&mut tokens);
            tokens.append(")");
        }
        None => {
            tokens.append("None");
        }
    }
    tokens
}
