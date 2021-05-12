use regex::Regex;

use lazy_static::lazy_static;
use proc_macro_error::abort;
use syn::spanned::Spanned;

lazy_static! {
    pub static ref COW_TYPE: Regex = Regex::new(r"Cow<'[a-z]+,str>").unwrap();
}

pub static NUMBER_TYPES: [&str; 40] = [
    "usize",
    "u8",
    "u16",
    "u32",
    "u64",
    "u128",
    "isize",
    "i8",
    "i16",
    "i32",
    "i64",
    "i128",
    "f32",
    "f64",
    "Option<usize>",
    "Option<u8>",
    "Option<u16>",
    "Option<u32>",
    "Option<u64>",
    "Option<u128>",
    "Option<isize>",
    "Option<i8>",
    "Option<i16>",
    "Option<i32>",
    "Option<i64>",
    "Option<i128>",
    "Option<f32>",
    "Option<f64>",
    "Option<Option<usize>>",
    "Option<Option<u8>>",
    "Option<Option<u16>>",
    "Option<Option<u32>>",
    "Option<Option<u64>>",
    "Option<Option<isize>>",
    "Option<Option<i8>>",
    "Option<Option<i16>>",
    "Option<Option<i32>>",
    "Option<Option<i64>>",
    "Option<Option<f32>>",
    "Option<Option<f64>>",
];

pub fn assert_string_type(name: &str, type_name: &str, field_type: &syn::Type) {
    if type_name != "String"
        && type_name != "&str"
        && !COW_TYPE.is_match(type_name)
        && type_name != "Option<String>"
        && type_name != "Option<Option<String>>"
        && !(type_name.starts_with("Option<") && type_name.ends_with("str>"))
        && !(type_name.starts_with("Option<Option<") && type_name.ends_with("str>>"))
    {
        abort!(
            field_type.span(),
            "`{}` validator can only be used on String, &str, Cow<'_,str> or an Option of those",
            name
        );
    }
}

pub fn assert_type_matches(
    field_name: String,
    field_type: &str,
    field_type2: Option<&String>,
    field_attr: &syn::Attribute,
) {
    if let Some(t2) = field_type2 {
        if field_type != t2 {
            abort!(field_attr.span(), "Invalid argument for `must_match` validator of field `{}`: types of field can't match", field_name);
        }
    } else {
        abort!(field_attr.span(), "Invalid argument for `must_match` validator of field `{}`: the other field doesn't exist in struct", field_name);
    }
}

pub fn assert_has_len(field_name: String, type_name: &str, field_type: &syn::Type) {
    if let syn::Type::Reference(ref tref) = field_type {
        let elem = &tref.elem;
        let type_name = format!("{}", quote::quote! { #elem }).replace(' ', "");

        if type_name == "str" {
            return;
        }
        assert_has_len(field_name, &type_name, elem);
        return;
    }

    if type_name != "String"
        && !type_name.starts_with("Vec<")
        && !type_name.starts_with("Option<Vec<")
        && !type_name.starts_with("Option<Option<Vec<")
        && type_name != "Option<String>"
        && type_name != "Option<Option<String>>"
        // a bit ugly
        && !(type_name.starts_with("Option<") && type_name.ends_with("str>"))
        && !(type_name.starts_with("Option<Option<") && type_name.ends_with("str>>"))
        && !COW_TYPE.is_match(type_name)
        && type_name != "&str"
    {
        abort!(field_type.span(),
                "Validator `length` can only be used on types `String`, `&str`, Cow<'_,str> or `Vec` but found `{}` for field `{}`",
                type_name, field_name
            );
    }
}

pub fn assert_has_range(field_name: String, type_name: &str, field_type: &syn::Type) {
    if !NUMBER_TYPES.contains(&type_name) {
        abort!(
            field_type.span(),
            "Validator `range` can only be used on number types but found `{}` for field `{}`",
            type_name,
            field_name
        );
    }
}
