use std::{
    borrow::Cow,
    cell::{Ref, RefMut},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    rc::Rc,
    sync::Arc,
};

#[cfg(feature = "indexmap")]
use indexmap::{IndexMap, IndexSet};

/// The unit used to measure the length of a value with the `length` validator.
///
/// This only changes how string-like values are measured. For collections
/// (`Vec`, `HashMap`, arrays, ...) the length is always the number of elements,
/// regardless of the unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LengthUnit {
    /// Unicode scalar values, i.e. the number of `char`s (`str::chars().count()`).
    ///
    /// This is the default and matches the behaviour of previous versions.
    #[default]
    Chars,
    /// UTF-16 code units.
    ///
    /// This matches how JavaScript's `String.length` and the `maxlength`
    /// attribute of an HTML form field count characters, which is useful when
    /// validating input coming from a web form.
    Utf16,
    /// Bytes (the length of the UTF-8 encoding for strings).
    Bytes,
}

/// Validates the length of the value given.
/// If the validator has `equal` set, it will ignore any `min` and `max` value.
///
/// If you apply it on String, don't forget that the length can be different
/// from the number of visual characters for Unicode. By default the length is
/// the number of Unicode scalar values (`char`s); use
/// [`validate_length_with_unit`](Self::validate_length_with_unit) to measure it
/// in UTF-16 code units or bytes instead.
pub trait ValidateLength<T>
where
    T: PartialEq + PartialOrd,
{
    fn validate_length(&self, min: Option<T>, max: Option<T>, equal: Option<T>) -> bool {
        self.validate_length_with_unit(LengthUnit::Chars, min, max, equal)
    }

    /// Same as [`validate_length`](Self::validate_length) but measures the
    /// length of string-like values using the given [`LengthUnit`].
    fn validate_length_with_unit(
        &self,
        unit: LengthUnit,
        min: Option<T>,
        max: Option<T>,
        equal: Option<T>,
    ) -> bool {
        if let Some(length) = self.length_with_unit(unit) {
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

    /// Returns the length of the value measured in the given [`LengthUnit`].
    ///
    /// The default implementation ignores the unit and returns [`length`](Self::length),
    /// which is the correct behaviour for every type where the unit is
    /// meaningless (collections, arrays, ...). String-like types override this
    /// to honour the unit.
    fn length_with_unit(&self, _unit: LengthUnit) -> Option<T> {
        self.length()
    }
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

            fn length_with_unit(&self, unit: LengthUnit) -> Option<u64> {
                T::length_with_unit(self, unit)
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

            fn length_with_unit(&self, unit: LengthUnit) -> Option<u64> {
                Some(match unit {
                    LengthUnit::Chars => self.chars().count() as u64,
                    LengthUnit::Utf16 => self.encode_utf16().count() as u64,
                    LengthUnit::Bytes => self.len() as u64,
                })
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

    fn length_with_unit(&self, unit: LengthUnit) -> Option<u64> {
        self.as_ref().length_with_unit(unit)
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

    fn length_with_unit(&self, unit: LengthUnit) -> Option<u64> {
        let Some(s) = self else {
            return None;
        };

        T::length_with_unit(s, unit)
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

    use super::{LengthUnit, ValidateLength};

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

    #[test]
    fn test_validate_length_with_unit_chars_is_default() {
        // "🖐🏽" is 2 chars, 4 UTF-16 code units and 8 bytes.
        let s = "🖐🏽";
        assert!(s.validate_length(None, None, Some(2)));
        assert!(s.validate_length_with_unit(LengthUnit::Chars, None, None, Some(2)));
    }

    #[test]
    fn test_validate_length_with_unit_utf16() {
        let s = "🖐🏽";
        assert!(s.validate_length_with_unit(LengthUnit::Utf16, None, None, Some(4)));
        assert!(!s.validate_length_with_unit(LengthUnit::Utf16, None, None, Some(2)));
    }

    #[test]
    fn test_validate_length_with_unit_bytes() {
        let s = "🖐🏽";
        assert!(s.validate_length_with_unit(LengthUnit::Bytes, None, None, Some(8)));
        assert!(s.validate_length_with_unit(LengthUnit::Bytes, Some(5), Some(10), None));
    }

    #[test]
    fn test_validate_length_with_unit_ascii_all_equal() {
        // For plain ASCII every unit yields the same count.
        let s = String::from("hello");
        for unit in [LengthUnit::Chars, LengthUnit::Utf16, LengthUnit::Bytes] {
            assert!(s.validate_length_with_unit(unit, None, None, Some(5)));
        }
    }

    #[test]
    fn test_validate_length_with_unit_through_wrappers() {
        // The unit must be honoured through references, `Option` and `Cow`.
        let s = String::from("日本");
        let by_ref: &String = &s;
        assert!(by_ref.validate_length_with_unit(LengthUnit::Bytes, None, None, Some(6)));
        assert!(Some(String::from("日本")).validate_length_with_unit(
            LengthUnit::Bytes,
            None,
            None,
            Some(6)
        ));
        let cow: Cow<'static, str> = "日本".into();
        assert!(cow.validate_length_with_unit(LengthUnit::Bytes, None, None, Some(6)));
    }

    #[test]
    fn test_validate_length_with_unit_ignored_for_collections() {
        // The unit is meaningless for collections: element count is used.
        assert!(vec![1, 2, 3].validate_length_with_unit(LengthUnit::Bytes, None, None, Some(3)));
    }
}
