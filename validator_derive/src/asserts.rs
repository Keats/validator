
pub static NUMBER_TYPES: [&'static str; 36] = [
    "usize", "u8", "u16", "u32", "u64",
    "isize", "i8", "i16", "i32", "i64",
    "f32", "f64",

    "Option<usize>", "Option<u8>", "Option<u16>", "Option<u32>", "Option<u64>",
    "Option<isize>", "Option<i8>", "Option<i16>", "Option<i32>", "Option<i64>",
    "Option<f32>", "Option<f64>",

    "Option<Option<usize>>", "Option<Option<u8>>", "Option<Option<u16>>", "Option<Option<u32>>", "Option<Option<u64>>",
    "Option<Option<isize>>", "Option<Option<i8>>", "Option<Option<i16>>", "Option<Option<i32>>", "Option<Option<i64>>",
    "Option<Option<f32>>", "Option<Option<f64>>",
];


pub fn assert_string_type(name: &str, field_type: &String) {
    if field_type != "String"
        && field_type != "&str"
        && field_type != "Option<String>"
        && field_type != "Option<Option<String>>"
        && !(field_type.starts_with("Option<") && field_type.ends_with("str>"))
        && !(field_type.starts_with("Option<Option<") && field_type.ends_with("str>>")) {
        panic!("`{}` validator can only be used on String, &str or an Option of those", name);
    }
}

pub fn assert_type_matches(field_name: String, field_type: &String, field_type2: Option<&String>) {
    if let Some(t2) = field_type2 {
        if field_type != t2 {
            panic!("Invalid argument for `must_match` validator of field `{}`: types of field can't match", field_name);
        }
    } else {
        panic!("Invalid argument for `must_match` validator of field `{}`: the other field doesn't exist in struct", field_name);
    }
}

pub fn assert_has_len(field_name: String, field_type: &String) {
    if field_type != "String"
        && !field_type.starts_with("Vec<")
        && !field_type.starts_with("Option<Vec<")
        && !field_type.starts_with("Option<Option<Vec<")
        && field_type != "Option<String>"
        && field_type != "Option<Option<String>>"
        // a bit ugly
        && !(field_type.starts_with("Option<") && field_type.ends_with("str>"))
        && !(field_type.starts_with("Option<Option<") && field_type.ends_with("str>>"))
        && field_type != "&str" {
            panic!(
                "Validator `length` can only be used on types `String`, `&str` or `Vec` but found `{}` for field `{}`",
                field_type, field_name
            );
    }
}

pub fn assert_has_range(field_name: String, field_type: &String) {
    if !NUMBER_TYPES.contains(&field_type.as_ref()) {
        panic!(
            "Validator `range` can only be used on number types but found `{}` for field `{}`",
            field_type, field_name
        );
    }
}
