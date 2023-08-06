fn custom_validation(value: String, arg: i32) -> Result<(), String> {
    Ok(())
}

struct TestStruct {
    value: String,
}

pub trait ValidateArgs {
    fn validate<F>(&self, value_closure: F) -> Result<(), String>
    where
        F: Fn(String) -> Result<(), String>;
}

impl ValidateArgs for TestStruct {
    fn validate<F>(&self, value_closure: F) -> Result<(), String>
    where
        F: Fn(String) -> Result<(), String>,
    {
        value_closure(self.value.clone())
    }
}

fn main() {
    let test = TestStruct { value: "Test".to_string() };
    let custom_closure = |val| custom_validation(val, 123);

    let res = test.validate(|v| custom_validation(v, 123));

    println!("Hello, world!");
}
