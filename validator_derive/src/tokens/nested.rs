use quote::quote;
use syn::Type;

pub fn nested_tokens(
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
    field_type: &Type,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    (
        quote! {
            errors.merge_self(#field_name_str, (&#field_name).validate());
        },
        quote! {
            constraints.merge(
                #field_name_str,
                <#field_type as ::validator::Constraints>::constraints(),
                <#field_type as ::validator::Constraints>::is_collection(),
            );
        },
    )
}
