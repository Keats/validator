use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate]
    inner: Inner,
}

impl Test {
    fn validate(&self) {
        // Method with the same name as Validate::validate(), we mustn't collide with it
    }
}

#[derive(Validate)]
struct Inner {
    #[validate(email)]
    value: String,
}

impl Inner {
    fn validate(&self) {
        // Method with the same name as Validate::validate(), we mustn't collide with it
    }
}

fn main() {}
