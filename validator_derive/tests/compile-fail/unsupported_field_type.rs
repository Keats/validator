use validator::Validate;

#[derive(Validate)]
struct Values {
    values: [u8; 10],
}

fn main() {}
