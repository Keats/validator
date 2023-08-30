use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

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

impl HasLen for String {
    fn length(&self) -> u64 {
        self.chars().count() as u64
    }
}

impl<'a> HasLen for &'a String {
    fn length(&self) -> u64 {
        self.chars().count() as u64
    }
}

impl<'a> HasLen for &'a str {
    fn length(&self) -> u64 {
        self.chars().count() as u64
    }
}

impl<'a> HasLen for Cow<'a, str> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

impl<T> HasLen for Vec<T> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

impl<'a, T> HasLen for &'a Vec<T> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

impl<T> HasLen for &[T] {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

impl<T, const N: usize> HasLen for [T; N] {
    fn length(&self) -> u64 {
        N as u64
    }
}

impl<T, const N: usize> HasLen for &[T; N] {
    fn length(&self) -> u64 {
        N as u64
    }
}

impl<'a, K, V, S> HasLen for &'a HashMap<K, V, S> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

impl<K, V, S> HasLen for HashMap<K, V, S> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

impl<'a, T, S> HasLen for &'a HashSet<T, S> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

impl<T, S> HasLen for HashSet<T, S> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

impl<'a, K, V> HasLen for &'a BTreeMap<K, V> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

impl<K, V> HasLen for BTreeMap<K, V> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

impl<'a, T> HasLen for &'a BTreeSet<T> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

impl<T> HasLen for BTreeSet<T> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

#[cfg(feature = "indexmap")]
impl<'a, K, V> HasLen for &'a IndexMap<K, V> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

#[cfg(feature = "indexmap")]
impl<K, V> HasLen for IndexMap<K, V> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

#[cfg(feature = "indexmap")]
impl<'a, T> HasLen for &'a IndexSet<T> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

#[cfg(feature = "indexmap")]
impl<T> HasLen for IndexSet<T> {
    fn length(&self) -> u64 {
        self.len() as u64
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
