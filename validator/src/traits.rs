use crate::types::ValidationErrors;

/// This is the original trait that was implemented by deriving `Validate`. It will still be
/// implemented for struct validations that don't take custom arguments. The call is being
/// forwarded to the `ValidateArgs<'v_a>` trait.
pub trait Validate {
    fn validate(&self) -> Result<(), ValidationErrors>;
}

impl<T: Validate> Validate for &T {
    fn validate(&self) -> Result<(), ValidationErrors> {
        T::validate(self)
    }
}

/// This trait will be implemented by deriving `Validate`. This implementation can take one
/// argument and pass this on to custom validators. The default `Args` type will be `()` if
/// there is no custom validation with defined arguments.
///
/// The `Args` type can use the lifetime `'v_a` to pass references onto the validator.
pub trait ValidateArgs<'v_a> {
    type Args;
    fn validate_with_args(&self, args: Self::Args) -> Result<(), ValidationErrors>;
}

impl<'v_a, T, U> ValidateArgs<'v_a> for Option<T>
where
    T: ValidateArgs<'v_a, Args = U>,
{
    type Args = U;

    fn validate_with_args(&self, args: Self::Args) -> Result<(), ValidationErrors> {
        if let Some(nested) = self {
            T::validate_with_args(nested, args)
        } else {
            Ok(())
        }
    }
}
