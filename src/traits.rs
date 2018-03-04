use std::ops::{AddAssign,SubAssign,MulAssign,DivAssign};
use num_traits::Num;
use num_traits::float::Float;

pub trait Base
    : Num
    + Copy
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
{    
}

pub trait BaseFloat
    : Base
    + Float
{
}

macro_rules! implement_base_traits {
    ({ $($type:ty),* }) => {
        $( impl Base for $type { } )*
    }
}
implement_base_traits!({ i8, i16, i32, i64, u8, u16, u32, u64, f32, f64 });

macro_rules! implement_base_float_traits {
    ({ $($type:ty),* }) => {
        $( impl BaseFloat for $type { } )*
    }
}
implement_base_float_traits!({ f32, f64 });
