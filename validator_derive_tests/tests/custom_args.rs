use validator::{Validate, ValidateArgs, ValidationError};

fn valid_generic_custom_fn<T>(_: &T, hello: i32) -> Result<(), ValidationError> {
    println!("{}", hello);
    Ok(())
}

fn valid_generic_custom_fn_2<T>(_: &T, hello: (i64, i64)) -> Result<(), ValidationError> {
    println!("{}", hello.0);
    Ok(())
}

fn valid_custom_fn(_: &str) -> Result<(), ValidationError> {
    Ok(())
}

#[allow(dead_code)]
fn invalid_custom_fn(_: &str) -> Result<(), ValidationError> {
    Err(ValidationError::new("meh"))
}

#[derive(Debug, Validate)]
struct TestGenericStruct<'a, T: serde::ser::Serialize> {
    #[validate(custom(function = "valid_generic_custom_fn", arg = "i32"))]
    generic: &'a T,

    //    #[validate(custom(function = "valid_generic_custom_fn", arg = "&mut Database"))]
    //    other: &'a T,
    //
    #[validate(custom(function = "valid_generic_custom_fn_2", arg = "(i64, i64)"))]
    duck: &'a T,

    #[validate(custom = "valid_custom_fn")]
    val: String,
}

// impl<'a, T: serde::ser::Serialize> ValidateArgs<'_> for TestGenericStruct<'a, T> {
//     type Args = ();
//
//     fn validate_args(&self, _args: Self::Args) -> Result<(), validator::ValidationErrors> {
//         Ok(())
//     }
// }

#[test]
fn validate_generic_struct_custom_fn_ok() {
    let generic = 746460_i128;
    let generic_2 = 746460_i128;

    let s = TestGenericStruct { generic: &generic, duck: &generic_2, val: "hello".to_string() };

    assert!(s.validate_args((16, (3, 5))).is_ok());
}
