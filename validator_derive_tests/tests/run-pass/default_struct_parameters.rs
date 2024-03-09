use validator::Validate;

#[derive(Validate)]
struct DefaultParameters<T = ()> {
    a: T,
}

fn main() {}
