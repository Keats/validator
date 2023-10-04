use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    rc::Rc,
    sync::Arc,
};
use std::cell::{Ref, RefMut};
use std::collections::VecDeque;

#[cfg(feature = "indexmap")]
use indexmap::{IndexMap, IndexSet};

use crate::types::ValidationErrors;

/// Trait to implement if one wants to make the `length` validator
/// work for more types
///
/// A bit sad it's not there by default in Rust
pub trait HasLen {
    fn length(&self) -> u64;
}

macro_rules! impl_type_that_derefs {
    ($type_:ty) => {
        impl<T> HasLen for $type_
        where T: HasLen {
            fn length(&self) -> u64 {
                T::length(self)
            }
        }
    };
}

impl_type_that_derefs!(&T);
impl_type_that_derefs!(Arc<T>);
impl_type_that_derefs!(Box<T>);
impl_type_that_derefs!(Rc<T>);
impl_type_that_derefs!(Ref<'_, T>);
impl_type_that_derefs!(RefMut<'_, T>);

macro_rules! impl_type_with_chars {
    ($type_:ty) => {
        impl HasLen for $type_ {
            fn length(&self) -> u64 {
                self.chars().count() as u64
            }
        }
    };
}

impl_type_with_chars!(str);
impl_type_with_chars!(&str);
impl_type_with_chars!(String);

macro_rules! impl_type_with_len {
    ($type_:ty, $($generic:ident),*$(,)*) => {
        impl<$($generic),*> HasLen for $type_ {
            fn length(&self) -> u64 {
                self.len() as u64
            }
        }
    };
}

impl_type_with_len!([T], T);
impl_type_with_len!(BTreeSet<T>, T);
impl_type_with_len!(BTreeMap<K, V>, K, V);
impl_type_with_len!(HashSet<T, S>, T, S);
impl_type_with_len!(HashMap<K, V, S>, K, V, S);
impl_type_with_len!(Vec<T>, T);
impl_type_with_len!(VecDeque<T>, T);
#[cfg(feature = "indexmap")]
impl_type_with_len!(IndexSet<T>, T);
#[cfg(feature = "indexmap")]
impl_type_with_len!(IndexMap<K, V>, K, V);

impl<'cow, T> HasLen for Cow<'cow, T>
    where T: ToOwned + ?Sized,
          for<'a> &'a T: HasLen {
    fn length(&self) -> u64 {
        self.as_ref().length()
    }
}

impl<T, const N: usize> HasLen for [T; N] {
    fn length(&self) -> u64 {
        N as u64
    }
}

/// This is the original trait that was implemented by deriving `Validate`. It will still be
/// implemented for struct validations that don't take custom arguments. The call is being
/// forwarded to the `ValidateArgs<'v_a>` trait.
pub trait Validate {
    fn validate(&self) -> Result<(), ValidationErrors>;
}

impl<T: Validate> Validate for &T {
    fn validate(&self) -> Result<(), ValidationErrors> {
        T::validate(*self)
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
        T: ValidateArgs<'v_a, Args=U>,
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
