use std::{
    borrow::Cow,
    cell::{Ref, RefMut},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    rc::Rc,
    sync::Arc,
};

#[cfg(feature = "indexmap")]
use indexmap::{IndexMap, IndexSet};

/// Validates the length of the value given.
/// If the validator has `equal` set, it will ignore any `min` and `max` value.
///
/// If you apply it on String, don't forget that the length can be different
/// from the number of visual characters for Unicode
pub trait ValidateLength<T>
where
    T: PartialEq + PartialOrd,
{
    fn validate_length(&self, min: Option<T>, max: Option<T>, equal: Option<T>) -> bool {
        if let Some(length) = self.length() {
            if let Some(eq) = equal {
                return length == eq;
            } else {
                if let Some(m) = min {
                    if length < m {
                        return false;
                    }
                }
                if let Some(m) = max {
                    if length > m {
                        return false;
                    }
                }
            }
            true
        } else {
            true
        }
    }

    fn length(&self) -> Option<T>;
}

macro_rules! validate_type_that_derefs {
    ($type_:ty) => {
        impl<T> ValidateLength<u64> for $type_
        where
            T: ValidateLength<u64>,
        {
            fn length(&self) -> Option<u64> {
                T::length(self)
            }
        }
    };
}

validate_type_that_derefs!(&T);
validate_type_that_derefs!(Arc<T>);
validate_type_that_derefs!(Box<T>);
validate_type_that_derefs!(Rc<T>);
validate_type_that_derefs!(Ref<'_, T>);
validate_type_that_derefs!(RefMut<'_, T>);

macro_rules! validate_type_with_chars {
    ($type_:ty) => {
        impl ValidateLength<u64> for $type_ {
            fn length(&self) -> Option<u64> {
                Some(self.chars().count() as u64)
            }
        }
    };
}

validate_type_with_chars!(str);
validate_type_with_chars!(&str);
validate_type_with_chars!(String);

macro_rules! validate_type_with_len {
    ($type_:ty) => {
        validate_type_with_len!($type_,);
    };
    ($type_:ty, $($generic:ident),*$(,)*) => {
        impl<$($generic),*> ValidateLength<u64> for $type_ {
            fn length(&self) -> Option<u64> {
                Some(self.len() as u64)
            }
        }
    };
}

validate_type_with_len!([T], T);
validate_type_with_len!(BTreeSet<T>, T);
validate_type_with_len!(BTreeMap<K, V>, K, V);
validate_type_with_len!(HashSet<T, S>, T, S);
validate_type_with_len!(HashMap<K, V, S>, K, V, S);
validate_type_with_len!(Vec<T>, T);
validate_type_with_len!(VecDeque<T>, T);
#[cfg(feature = "indexmap")]
validate_type_with_len!(IndexSet<T>, T);
#[cfg(feature = "indexmap")]
validate_type_with_len!(IndexMap<K, V>, K, V);

impl<T> ValidateLength<u64> for Cow<'_, T>
where
    T: ToOwned + ?Sized,
    for<'a> &'a T: ValidateLength<u64>,
{
    fn length(&self) -> Option<u64> {
        self.as_ref().length()
    }
}

impl<T> ValidateLength<u64> for Option<T>
where
    T: ValidateLength<u64>,
{
    fn length(&self) -> Option<u64> {
        let Some(s) = self else {
            return None;
        };

        T::length(s)
    }
}

impl<T, const N: usize> ValidateLength<u64> for [T; N] {
    fn length(&self) -> Option<u64> {
        Some(N as u64)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::ValidateLength;

    #[test]
    fn test_validate_length_equal_overrides_min_max() {
        assert!("hello".validate_length(Some(1), Some(2), Some(5)));
    }

    #[test]
    fn test_validate_length_string_min_max() {
        assert!("hello".validate_length(Some(1), Some(10), None));
    }

    #[test]
    fn test_validate_length_string_min_only() {
        assert!(!"hello".validate_length(Some(10), None, None));
    }

    #[test]
    fn test_validate_length_string_max_only() {
        assert!(!"hello".validate_length(None, Some(1), None));
    }

    #[test]
    fn test_validate_length_cow() {
        let test: Cow<'static, str> = "hello".into();
        assert!(test.validate_length(None, None, Some(5)));

        let test: Cow<'static, str> = String::from("hello").into();
        assert!(test.validate_length(None, None, Some(5)));
    }

    #[test]
    fn test_validate_length_vec() {
        assert!(vec![1, 2, 3].validate_length(None, None, Some(3)));
    }

    #[test]
    fn test_validate_length_unicode_chars() {
        assert!("日本".validate_length(None, None, Some(2)));
    }

    #[test]
    fn test_validate_length_cow_unicode_chars() {
        let test: Cow<'static, str> = "日本".into();
        assert!(test.validate_length(None, None, Some(2)));

        let test: Cow<'static, str> = String::from("日本").into();
        assert!(test.validate_length(None, None, Some(2)));
    }

    #[test]
    fn test_validate_length_trait_equal_overrides_min_max() {
        assert!(String::from("hello").validate_length(Some(1), Some(2), Some(5)));
    }

    #[test]
    fn test_validate_length_trait_string_min_max() {
        assert!(String::from("hello").validate_length(Some(1), Some(10), None));
    }

    #[test]
    fn test_validate_length_trait_string_min_only() {
        assert!(!String::from("hello").validate_length(Some(10), None, None));
    }

    #[test]
    fn test_validate_length_trait_string_max_only() {
        assert!(!String::from("hello").validate_length(None, Some(1), None));
    }

    #[test]
    fn test_validate_length_trait_cow() {
        let test: Cow<'static, str> = "hello".into();
        assert!(test.validate_length(None, None, Some(5)));

        let test: Cow<'static, str> = String::from("hello").into();
        assert!(test.validate_length(None, None, Some(5)));
    }

    #[test]
    fn test_validate_length_trait_vec() {
        assert!(vec![1, 2, 3].validate_length(None, None, Some(3)));
    }

    #[test]
    fn test_validate_length_trait_unicode_chars() {
        assert!(String::from("日本").validate_length(None, None, Some(2)));
    }
}
