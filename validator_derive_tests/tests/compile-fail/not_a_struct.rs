use validator::Validate;

#[derive(Validate)]
pub enum NotAStruct {
    VariantA(i32),
    VariantB(String),
    VariantC(Vec<String>),
}

fn main() {}
