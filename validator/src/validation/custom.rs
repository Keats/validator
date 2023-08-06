pub trait ValidateCustom {
    fn validate_custom(&self) -> bool;
    fn validate_custom_args(&self, args: T) -> bool;
}
