use quote::quote;
use syn::Ident;

pub fn nested_tokens(field_name: &Ident, field_name_str: &str) -> proc_macro2::TokenStream {
    quote! {
        errors.add_non_nested(#field_name_str, self.#field_name.validate_nested(#field_name_str));
    }
}
