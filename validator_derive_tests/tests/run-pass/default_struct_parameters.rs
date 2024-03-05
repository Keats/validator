use validator::Validate;

#[derive(Validate)]
struct Test<T = ()> {
    a: T,
}

fn main() {}
