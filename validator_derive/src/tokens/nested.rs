use quote::quote;

pub fn nested_tokens(
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    quote! {
        errors.merge_self(#field_name_str, (&#field_name).validate());
    }
}
