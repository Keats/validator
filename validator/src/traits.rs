use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[cfg(feature = "indexmap")]
use indexmap::{IndexMap, IndexSet};

use crate::{
    types::{ValidationErrors, ValidationErrorsKind},
    ValidationConstraints,
};

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
                        "_tmp_validator",
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
            let errors =
                ValidationErrors(std::collections::HashMap::from([("_tmp_validator", err_kind)]));
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
            let errors = ValidationErrors(HashMap::from([("_tmp_validator", err_kind)]));
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
            let errors = ValidationErrors(HashMap::from([("_tmp_validator", err_kind)]));
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

pub trait Constraints {
    fn constraints() -> ValidationConstraints;

    fn is_collection() -> bool {
        false
    }
}

impl<T: Constraints> Constraints for &T {
    fn constraints() -> ValidationConstraints {
        T::constraints()
    }

    fn is_collection() -> bool {
        T::is_collection()
    }
}

impl<T: Constraints> Constraints for Option<T> {
    fn constraints() -> ValidationConstraints {
        T::constraints()
    }

    fn is_collection() -> bool {
        T::is_collection()
    }
}

macro_rules! impl_constraints {
    ($ty:ty) => {
        impl<T: Constraints> Constraints for $ty {
            fn constraints() -> ValidationConstraints {
                T::constraints()
            }

            fn is_collection() -> bool {
                true
            }
        }
    };
}

impl_constraints!(&[T]);
impl_constraints!(Vec<T>);
impl_constraints!(BTreeSet<T>);

impl<T: Constraints, const N: usize> Constraints for [T; N] {
    fn constraints() -> ValidationConstraints {
        T::constraints()
    }

    fn is_collection() -> bool {
        true
    }
}

impl<K, V: Constraints, S> Constraints for HashMap<K, V, S> {
    fn constraints() -> ValidationConstraints {
        V::constraints()
    }

    fn is_collection() -> bool {
        true
    }
}

impl<V: Constraints, S> Constraints for HashSet<V, S> {
    fn constraints() -> ValidationConstraints {
        V::constraints()
    }

    fn is_collection() -> bool {
        true
    }
}

impl<K, V: Constraints> Constraints for BTreeMap<K, V> {
    fn constraints() -> ValidationConstraints {
        V::constraints()
    }

    fn is_collection() -> bool {
        true
    }
}

#[cfg(feature = "indexmap")]
impl<K, V: Constraints> Constraints for IndexMap<K, V> {
    fn constraints() -> ValidationConstraints {
        V::constraints()
    }

    fn is_collection() -> bool {
        true
    }
}

#[cfg(feature = "indexmap")]
impl<V: Constraints> Constraints for IndexSet<V> {
    fn constraints() -> ValidationConstraints {
        V::constraints()
    }

    fn is_collection() -> bool {
        true
    }
}
