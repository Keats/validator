use quote::quote;

use crate::CrateName;

pub fn nested_tokens(
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
    crate_name: &CrateName,
) -> proc_macro2::TokenStream {
    quote! {
        if let std::collections::hash_map::Entry::Vacant(entry) = errors.0.entry(::std::borrow::Cow::Borrowed(#field_name_str)) {
            use #crate_name::Validate;
            errors.merge_self(#field_name_str, (&#field_name).validate());
        }
    }
}
