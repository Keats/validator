/// Validates whether the given Option is Some
#[must_use]
pub fn validate_required<T: ValidateRequired>(val: &T) -> bool {
    val.is_some()
}

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
