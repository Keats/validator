use quote::quote;

pub fn nested_tokens(
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    quote! {
        if !errors.0.contains_key(#field_name_str) {
            errors.merge_self(#field_name_str, #field_name.validate_nested(#field_name_str, args));
        }
    }
}
