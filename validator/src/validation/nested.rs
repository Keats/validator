use crate::{ValidateArgs, ValidationErrors, ValidationErrorsKind};
use std::collections::{BTreeMap, HashMap, HashSet};
pub trait ValidateNested<'v_a> {
    type Args;
    fn validate_nested(
        &self,
        field_name: &'static str,
        args: Self::Args,
    ) -> Result<(), ValidationErrors>;
}

impl<'v_a, T, U> ValidateNested<'v_a> for &T
where
    T: ValidateNested<'v_a, Args = U>,
{
    type Args = U;

    fn validate_nested(
        &self,
        field_name: &'static str,
        args: Self::Args,
    ) -> Result<(), ValidationErrors> {
        T::validate_nested(self, field_name, args)
    }
}

impl<'v_a, T, U> ValidateNested<'v_a> for Option<T>
where
    T: ValidateNested<'v_a, Args = U>,
{
    type Args = U;

    fn validate_nested(
        &self,
        field_name: &'static str,
        args: Self::Args,
    ) -> Result<(), ValidationErrors> {
        if let Some(nested) = self {
            nested.validate_nested(field_name, args)
        } else {
            Ok(())
        }
    }
}

impl<'v_a, T, U> ValidateNested<'v_a> for Vec<T>
where
    T: ValidateArgs<'v_a, Args = U> + ValidateNested<'v_a, Args = U>,
    U: Clone,
{
    type Args = U;
    fn validate_nested(
        &self,
        field_name: &'static str,
        args: Self::Args,
    ) -> Result<(), ValidationErrors> {
        let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

        for (index, item) in self.iter().enumerate() {
            if let Err(e) = item.validate_with_args(args.clone()) {
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

impl<'v_a, T, U> ValidateNested<'v_a> for &[T]
where
    T: ValidateArgs<'v_a, Args = U> + ValidateNested<'v_a, Args = U>,
    U: Clone,
{
    type Args = U;

    fn validate_nested(
        &self,
        field_name: &'static str,
        args: Self::Args,
    ) -> Result<(), ValidationErrors> {
        let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

        for (index, item) in self.iter().enumerate() {
            if let Err(e) = item.validate_with_args(args.clone()) {
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

impl<'v_a, T, const N: usize, U> ValidateNested<'v_a> for [T; N]
where
    T: ValidateArgs<'v_a, Args = U> + ValidateNested<'v_a, Args = U>,
    U: Clone,
{
    type Args = U;
    fn validate_nested(
        &self,
        field_name: &'static str,
        args: Self::Args,
    ) -> Result<(), ValidationErrors> {
        let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

        for (index, item) in self.iter().enumerate() {
            if let Err(e) = item.validate_with_args(args.clone()) {
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

impl<'v_a, K, V, S, U> ValidateNested<'v_a> for HashMap<K, V, S>
where
    V: ValidateArgs<'v_a, Args = U> + ValidateNested<'v_a, Args = U>,
    U: Clone,
{
    type Args = U;
    fn validate_nested(
        &self,
        field_name: &'static str,
        args: Self::Args,
    ) -> Result<(), ValidationErrors> {
        let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

        for (index, (_key, value)) in self.iter().enumerate() {
            if let Err(e) = value.validate_with_args(args.clone()) {
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

impl<'v_a, T, S, U> ValidateNested<'v_a> for HashSet<T, S>
where
    T: ValidateArgs<'v_a, Args = U> + ValidateNested<'v_a, Args = U>,
    U: Clone,
{
    type Args = U;
    fn validate_nested(
        &self,
        field_name: &'static str,
        args: Self::Args,
    ) -> Result<(), ValidationErrors> {
        let mut vec_err: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();

        for (index, value) in self.iter().enumerate() {
            if let Err(e) = value.validate_with_args(args.clone()) {
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
