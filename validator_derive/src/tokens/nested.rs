use quote::quote;

pub fn nested_tokens(
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
    flatten: bool,
) -> proc_macro2::TokenStream {
    if flatten {
        quote! {
            errors.merge_self_flatten((&#field_name).validate());
        }
    } else {
        quote! {
            if let std::collections::hash_map::Entry::Vacant(entry) = errors.0.entry(::std::borrow::Cow::Borrowed(#field_name_str)) {
                errors.merge_self(#field_name_str, (&#field_name).validate());
            }
        }
    }
}
