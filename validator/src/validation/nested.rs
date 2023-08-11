use std::collections::{BTreeMap, HashMap, HashSet};

use crate::{Validate, ValidationErrors, ValidationErrorsKind};

pub trait ValidateNested {
    fn validate_nested(&self, field_name: &'static str) -> Result<(), ValidationErrors>;
}

impl<T> ValidateNested for T
where
    T: Validate,
{
    fn validate_nested(&self, _field_name: &'static str) -> Result<(), ValidationErrors> {
        self.validate()
    }
}

impl<T> ValidateNested for Option<T>
where
    T: Validate,
{
    fn validate_nested(&self, _field_name: &'static str) -> Result<(), ValidationErrors> {
        if let Some(s) = self {
            s.validate()
        } else {
            Ok(())
        }
    }
}

impl<T> ValidateNested for Option<Option<T>>
where
    T: Validate,
{
    fn validate_nested(&self, _field_name: &'static str) -> Result<(), ValidationErrors> {
        if let Some(s) = self {
            if let Some(s) = s {
                s.validate()
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}

impl<T> ValidateNested for Vec<T>
where
    T: Validate,
{
    fn validate_nested(&self, field_name: &'static str) -> Result<(), ValidationErrors> {
        let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

        for (index, item) in self.iter().enumerate() {
            if let Err(e) = item.validate() {
                vec_err.insert(index, Box::new(e));
            }
        }

        let err_kind = ValidationErrorsKind::List(vec_err);
        let errors = ValidationErrors(HashMap::from([(field_name, err_kind)]));

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl<T> ValidateNested for Option<Vec<T>>
where
    T: Validate,
{
    fn validate_nested(&self, field_name: &'static str) -> Result<(), ValidationErrors> {
        if let Some(vec) = self {
            vec.validate_nested(field_name)
        } else {
            Ok(())
        }
    }
}

impl<T> ValidateNested for Option<Option<Vec<T>>>
where
    T: Validate,
{
    fn validate_nested(&self, field_name: &'static str) -> Result<(), ValidationErrors> {
        if let Some(vec) = self {
            if let Some(vec) = vec {
                vec.validate_nested(field_name)
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}

impl<T> ValidateNested for &[T]
where
    T: Validate,
{
    fn validate_nested(&self, field_name: &'static str) -> Result<(), ValidationErrors> {
        let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

        for (index, item) in self.iter().enumerate() {
            if let Err(e) = item.validate() {
                vec_err.insert(index, Box::new(e));
            }
        }

        let err_kind = ValidationErrorsKind::List(vec_err);
        let errors = ValidationErrors(HashMap::from([(field_name, err_kind)]));

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl<T> ValidateNested for Option<&[T]>
where
    T: Validate,
{
    fn validate_nested(&self, field_name: &'static str) -> Result<(), ValidationErrors> {
        if let Some(val) = self {
            val.validate_nested(field_name)
        } else {
            Ok(())
        }
    }
}

impl<T> ValidateNested for Option<Option<&[T]>>
where
    T: Validate,
{
    fn validate_nested(&self, field_name: &'static str) -> Result<(), ValidationErrors> {
        if let Some(val) = self {
            if let Some(val) = val {
                val.validate_nested(field_name)
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}

impl<T, const N: usize> ValidateNested for [T; N]
where
    T: Validate,
{
    fn validate_nested(&self, field_name: &'static str) -> Result<(), ValidationErrors> {
        let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

        for (index, item) in self.iter().enumerate() {
            if let Err(e) = item.validate() {
                vec_err.insert(index, Box::new(e));
            }
        }

        let err_kind = ValidationErrorsKind::List(vec_err);
        let errors = ValidationErrors(HashMap::from([(field_name, err_kind)]));

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl<T, const N: usize> ValidateNested for Option<[T; N]>
where
    T: Validate,
{
    fn validate_nested(&self, field_name: &'static str) -> Result<(), ValidationErrors> {
        if let Some(val) = self {
            val.validate_nested(field_name)
        } else {
            Ok(())
        }
    }
}

impl<T, const N: usize> ValidateNested for Option<Option<[T; N]>>
where
    T: Validate,
{
    fn validate_nested(&self, field_name: &'static str) -> Result<(), ValidationErrors> {
        if let Some(val) = self {
            if let Some(val) = val {
                val.validate_nested(field_name)
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}

impl<T, const N: usize> ValidateNested for &[T; N]
where
    T: Validate,
{
    fn validate_nested(&self, field_name: &'static str) -> Result<(), ValidationErrors> {
        let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

        for (index, item) in self.iter().enumerate() {
            if let Err(e) = item.validate() {
                vec_err.insert(index, Box::new(e));
            }
        }

        let err_kind = ValidationErrorsKind::List(vec_err);
        let errors = ValidationErrors(HashMap::from([(field_name, err_kind)]));

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl<T, const N: usize> ValidateNested for Option<&[T; N]>
where
    T: Validate,
{
    fn validate_nested(&self, field_name: &'static str) -> Result<(), ValidationErrors> {
        if let Some(val) = self {
            val.validate_nested(field_name)
        } else {
            Ok(())
        }
    }
}

impl<T, const N: usize> ValidateNested for Option<Option<&[T; N]>>
where
    T: Validate,
{
    fn validate_nested(&self, field_name: &'static str) -> Result<(), ValidationErrors> {
        if let Some(val) = self {
            if let Some(val) = val {
                val.validate_nested(field_name)
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}

impl<K, V, S> ValidateNested for HashMap<K, V, S>
where
    V: Validate,
{
    fn validate_nested(&self, field_name: &'static str) -> Result<(), ValidationErrors> {
        let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

        for (index, (_key, value)) in self.iter().enumerate() {
            if let Err(e) = value.validate() {
                vec_err.insert(index, Box::new(e));
            }
        }

        let err_kind = ValidationErrorsKind::List(vec_err);
        let errors = ValidationErrors(HashMap::from([(field_name, err_kind)]));

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl<K, V, S> ValidateNested for Option<HashMap<K, V, S>>
where
    V: Validate,
{
    fn validate_nested(&self, field_name: &'static str) -> Result<(), ValidationErrors> {
        if let Some(val) = self {
            val.validate_nested(field_name)
        } else {
            Ok(())
        }
    }
}

impl<K, V, S> ValidateNested for Option<Option<HashMap<K, V, S>>>
where
    V: Validate,
{
    fn validate_nested(&self, field_name: &'static str) -> Result<(), ValidationErrors> {
        if let Some(val) = self {
            if let Some(val) = val {
                val.validate_nested(field_name)
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}

impl<T, S> ValidateNested for HashSet<T, S>
where
    T: Validate,
{
    fn validate_nested(&self, field_name: &'static str) -> Result<(), ValidationErrors> {
        let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

        for (index, value) in self.iter().enumerate() {
            if let Err(e) = value.validate() {
                vec_err.insert(index, Box::new(e));
            }
        }

        let err_kind = ValidationErrorsKind::List(vec_err);
        let errors = ValidationErrors(HashMap::from([(field_name, err_kind)]));

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl<T, S> ValidateNested for Option<HashSet<T, S>>
where
    T: Validate,
{
    fn validate_nested(&self, field_name: &'static str) -> Result<(), ValidationErrors> {
        if let Some(val) = self {
            val.validate_nested(field_name)
        } else {
            Ok(())
        }
    }
}

impl<T, S> ValidateNested for Option<Option<HashSet<T, S>>>
where
    T: Validate,
{
    fn validate_nested(&self, field_name: &'static str) -> Result<(), ValidationErrors> {
        if let Some(val) = self {
            if let Some(val) = val {
                val.validate_nested(field_name)
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}
