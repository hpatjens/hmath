use std::ops::{AddAssign,SubAssign,MulAssign,DivAssign};
use num_traits::Num;
use num_traits::float::Float;
use float_cmp::{ApproxEq,Ulps};

pub trait Base
    : Num
    + Copy
    + AddAssign
    + ::std::fmt::Debug // TODO(henk): Remove this
    + SubAssign
    + MulAssign
    + DivAssign
    + PartialOrd
{
}

pub trait BaseFloat
    : Base
    + Float
    + ApproxEq
    + Ulps
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

pub trait Elem<T> {
    type Output;
    fn elem(self, index: T) -> Self::Output;
}

pub trait ElemMut<T> {
    type Output;
    fn elem_mut(self, index: T) -> Self::Output;
}

pub trait AsArray {
    type Output;
    fn as_array(self) -> Self::Output; // TODO(henk): How to design this in a way that defined the output as an array?
}

pub trait AsArrays {
    type Output;
    fn as_arrays(self) -> Self::Output; // TODO(henk): How to design this in a way that defined the output as an array?
}

pub trait MatrixAccess {
    type Output;
    fn col_elem(self, col_index: usize, elem_index: usize) -> Self::Output;
    fn row_elem(self, row_index: usize, elem_index: usize) -> Self::Output;
}

pub trait MatrixAccessRef<'a> {
    type Output;
    fn col_elem_ref(self, col_index: usize, elem_index: usize) -> &'a Self::Output;
    fn row_elem_ref(self, row_index: usize, elem_index: usize) -> &'a Self::Output;
}

pub trait MatrixAccessMut<'a> {
    type Output;
    fn col_elem_mut(self, col_index: usize, elem_index: usize) -> &'a mut Self::Output;
    fn row_elem_mut(self, row_index: usize, elem_index: usize) -> &'a mut Self::Output;
}