use std::collections::{BTreeMap, HashMap, HashSet};

use crate::{ValidateArgs, ValidationErrors, ValidationErrorsKind};

pub trait ValidateNested<T> {
    fn validate_nested(&self, field_name: &'static str, args: T) -> Result<(), ValidationErrors>;
}

impl<'v_a, T, U> ValidateNested<U> for T
where
    T: ValidateArgs<'v_a, Args = U>,
    U: Clone,
{
    fn validate_nested(&self, field_name: &'static str, args: U) -> Result<(), ValidationErrors> {
        let res = self.validate(args);

        if let Err(e) = res {
            let new_err = ValidationErrorsKind::Struct(Box::new(e));
            Err(ValidationErrors(HashMap::from([(field_name, new_err)])))
        } else {
            Ok(())
        }
    }
}

// impl<T, U> ValidateNested<U> for Option<T>
// where
//     T: ValidateNested<U>,
//     U: ValidateContext + Clone,
// {
//     fn validate_nested(&self, field_name: &'static str, args: U) -> Result<(), ValidationErrors> {
//         if let Some(s) = self {
//             s.validate_nested(field_name, args)
//         } else {
//             Ok(())
//         }
//     }
// }

impl<'v_a, T, U> ValidateNested<U> for Vec<T>
where
    T: ValidateArgs<'v_a, Args = U>,
    U: Clone,
{
    fn validate_nested(&self, field_name: &'static str, args: U) -> Result<(), ValidationErrors> {
        let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

        for (index, item) in self.iter().enumerate() {
            if let Err(e) = item.validate(args.clone()) {
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

impl<'v_a, T, U> ValidateNested<U> for &[T]
where
    T: ValidateArgs<'v_a, Args = U>,
    U: Clone,
{
    fn validate_nested(&self, field_name: &'static str, args: U) -> Result<(), ValidationErrors> {
        let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

        for (index, item) in self.iter().enumerate() {
            if let Err(e) = item.validate(args.clone()) {
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

impl<'v_a, T, const N: usize, U> ValidateNested<U> for [T; N]
where
    T: ValidateArgs<'v_a, Args = U>,
    U: Clone,
{
    fn validate_nested(&self, field_name: &'static str, args: U) -> Result<(), ValidationErrors> {
        let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

        for (index, item) in self.iter().enumerate() {
            if let Err(e) = item.validate(args.clone()) {
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

impl<'v_a, T, const N: usize, U> ValidateNested<U> for &[T; N]
where
    T: ValidateArgs<'v_a, Args = U>,
    U: Clone,
{
    fn validate_nested(&self, field_name: &'static str, args: U) -> Result<(), ValidationErrors> {
        let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

        for (index, item) in self.iter().enumerate() {
            if let Err(e) = item.validate(args.clone()) {
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

impl<'v_a, K, V, S, U> ValidateNested<U> for HashMap<K, V, S>
where
    V: ValidateArgs<'v_a, Args = U>,
    U: Clone,
{
    fn validate_nested(&self, field_name: &'static str, args: U) -> Result<(), ValidationErrors> {
        let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

        for (index, (_key, value)) in self.iter().enumerate() {
            if let Err(e) = value.validate(args.clone()) {
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

impl<'v_a, T, S, U> ValidateNested<U> for HashSet<T, S>
where
    T: ValidateArgs<'v_a, Args = U>,
    U: Clone,
{
    fn validate_nested(&self, field_name: &'static str, args: U) -> Result<(), ValidationErrors> {
        let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

        for (index, value) in self.iter().enumerate() {
            if let Err(e) = value.validate(args.clone()) {
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
