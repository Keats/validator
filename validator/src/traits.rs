use crate::types::{ValidationErrors, ValidationErrorsKind};
use std::borrow::Cow;
use std::collections::btree_map::BTreeMap;
use std::collections::HashMap;

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

macro_rules! impl_validate_list {
    ($container:ty) => {
        impl<T: Validate> Validate for $container {
            fn validate(&self) -> Result<(), ValidationErrors> {
                let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

                for (index, item) in self.iter().enumerate() {
                    if let Err(e) = item.validate() {
                        vec_err.insert(index, Box::new(e));
                    }
                }

                if vec_err.is_empty() {
                    Ok(())
                } else {
                    let err_kind = ValidationErrorsKind::List(vec_err);
                    let errors = ValidationErrors(std::collections::HashMap::from([(
                        Cow::Borrowed("_tmp_validator"),
                        err_kind,
                    )]));
                    Err(errors)
                }
            }
        }
    };
}

impl_validate_list!(std::collections::HashSet<T>);
impl_validate_list!(std::collections::BTreeSet<T>);
impl_validate_list!(std::collections::BinaryHeap<T>);
impl_validate_list!(std::collections::LinkedList<T>);
impl_validate_list!(std::collections::VecDeque<T>);
impl_validate_list!(std::vec::Vec<T>);
impl_validate_list!([T]);

impl<T: Validate, const N: usize> Validate for [T; N] {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

        for (index, item) in self.iter().enumerate() {
            if let Err(e) = item.validate() {
                vec_err.insert(index, Box::new(e));
            }
        }

        if vec_err.is_empty() {
            Ok(())
        } else {
            let err_kind = ValidationErrorsKind::List(vec_err);
            let errors = ValidationErrors(std::collections::HashMap::from([(
                Cow::Borrowed("_tmp_validator"),
                err_kind,
            )]));
            Err(errors)
        }
    }
}

impl<K, V: Validate, S> Validate for &HashMap<K, V, S> {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

        for (index, (_key, value)) in self.iter().enumerate() {
            if let Err(e) = value.validate() {
                vec_err.insert(index, Box::new(e));
            }
        }

        if vec_err.is_empty() {
            Ok(())
        } else {
            let err_kind = ValidationErrorsKind::List(vec_err);
            let errors =
                ValidationErrors(HashMap::from([(Cow::Borrowed("_tmp_validator"), err_kind)]));
            Err(errors)
        }
    }
}

impl<K, V: Validate> Validate for &BTreeMap<K, V> {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

        for (index, (_key, value)) in self.iter().enumerate() {
            if let Err(e) = value.validate() {
                vec_err.insert(index, Box::new(e));
            }
        }

        if vec_err.is_empty() {
            Ok(())
        } else {
            let err_kind = ValidationErrorsKind::List(vec_err);
            let errors =
                ValidationErrors(HashMap::from([(Cow::Borrowed("_tmp_validator"), err_kind)]));
            Err(errors)
        }
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
