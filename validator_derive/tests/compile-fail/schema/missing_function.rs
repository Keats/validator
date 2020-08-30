use validator::Validate;

#[derive(Validate)]
#[validate(schema())]
struct Test {
    s: i32,
}

fn hey(_: &Test) -> Option<(String, String)> {
    None
}

fn main() {}
