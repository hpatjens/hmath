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
    + PartialOrd
{
}

pub trait BaseFloat
    : Base
    + Float
{
}

macro_rules! implement_base_traits {
    { $($type:ty),* } => {
        $( impl Base for $type { } )*
    }
}
implement_base_traits!{ i8, i16, i32, i64, u8, u16, u32, u64, f32, f64 }

macro_rules! implement_base_float_traits {
    { $($type:ty),* } => {
        $( impl BaseFloat for $type { } )*
    }
}
implement_base_float_traits!{ f32, f64 }


pub trait Cross<RHS = Self> {
    type Output;
    fn cross(self, rhs: RHS) -> Self::Output;
}

pub trait Dot<RHS> {
    type Output;
    fn dot(self, rhs: RHS) -> Self::Output;
}

pub trait CwMin<RHS = Self> {
    type Output;
    fn cw_min(self, rhs: RHS) -> Self::Output;
}

pub trait CwMax<RHS = Self> {
    type Output;
    fn cw_max(self, rhs: RHS) -> Self::Output;
}

pub trait CwAbs<RHS = Self> {
    type Output;
    fn cw_abs(self) -> Self::Output;
}