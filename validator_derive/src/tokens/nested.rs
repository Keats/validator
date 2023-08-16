use quote::quote;
use syn::Ident;

pub fn nested_tokens(field_name: &Ident, field_name_str: &str) -> proc_macro2::TokenStream {
    quote! {
        if !errors.0.contains_key(#field_name_str) {
            errors.merge_self(#field_name_str, self.#field_name.validate_nested(#field_name_str, args));
        }
    }
}
