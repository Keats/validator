use validator::Validate;

use std::num::{NonZeroI32, NonZeroI64, NonZeroU32, NonZeroU64};

#[derive(Validate)]
struct Test {
    #[validate(range(min = 1, max = 2.2))]
    s: isize,
    #[validate(range(min = 1, max = 2))]
    s2: usize,
    #[validate(range(min = 18, max = 22))]
    s3: i32,
    #[validate(range(min = 18, max = 22))]
    s4: i64,
    #[validate(range(min = 18, max = 22))]
    s5: u32,
    #[validate(range(min = 18, max = 22))]
    s6: u64,
    #[validate(range(min = 18.1, max = 22))]
    s7: i8,
    #[validate(range(min = 18.0, max = 22))]
    s8: u8,
    #[validate(range(min = 18.0, max = 22))]
    s9: Option<u8>,
    #[validate(range(min = 18.0))]
    s10: Option<u8>,
    #[validate(range(max = 18.0))]
    s11: Option<u8>,
    #[validate(range(min = 18, max = 22))]
    s12: NonZeroI32,
    #[validate(range(min = 18, max = 22))]
    s13: NonZeroU32,
    #[validate(range(min = 18.1, max = 22))]
    s14: NonZeroI32,
    #[validate(range(min = 18.1, max = 22))]
    s15: NonZeroU32,
    #[validate(range(min = 18, max = 22))]
    s16: Option<NonZeroI64>,
    #[validate(range(min = 18, max = 22))]
    s17: Option<NonZeroU64>,
}

fn main() {}
