/// Validates whether the given Option is Some
pub trait ValidateRequired {
    fn validate_required(&self) -> bool {
        self.is_some()
    }

    fn is_some(&self) -> bool;
}

impl<T> ValidateRequired for Option<T> {
    fn is_some(&self) -> bool {
        self.is_some()
    }
}
