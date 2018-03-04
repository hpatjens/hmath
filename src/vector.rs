use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,Neg};
use num_traits::Signed;

use traits::*;

macro_rules! implement_binary_assign_operator {
    ($operator_trait:ident<$type_rhs:ty> for $type:ty,
        fn $function:ident($lhs:ident, $rhs:ident) {
            $body:expr
        }
    ) => {
        impl<T> $operator_trait<$type_rhs> for $type
            where T: Base + $operator_trait<T>
        {
            fn $function(&mut self, other: $type_rhs) {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }
    }
}

macro_rules! implement_binary_operator {
    ($operator_trait:ident<$type_rhs:ty> for $type:ty,
        fn $function:ident($lhs:ident, $rhs:ident) -> $result_type:ty {
            $body:expr
        }
    ) => {
        //
        // IMPLEMENT OPERATOR FOR TYPE WITHOUT REFERENCE
        //
        impl<T> $operator_trait<$type_rhs> for $type
            where T: Base + $operator_trait<Output=T>
        {
            type Output = $result_type;

            #[inline]
            fn $function(self, other: $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }

        //
        // IMPLEMENT OPERATOR FOR TYPE WITH REFERENCE
        //
        impl<'a, T> $operator_trait<&'a $type_rhs> for &'a $type
            where T: Base + $operator_trait<Output=T>
        {
            type Output = $result_type;

            #[inline]
            fn $function(self, other: &'a $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, *other);
                $body
            }
        }

        //
        // IMPLEMENT OPERATOR FOR TYPE WITH REFERENCE ON RHS
        //
        impl<'a, T> $operator_trait<&'a $type_rhs> for $type
            where T: Base + $operator_trait<Output=T>
        {
            type Output = $result_type;

            #[inline]
            fn $function(self, other: &'a $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (self, *other);
                $body
            }
        }

        //
        // IMPLEMENT OPERATOR FOR TYPE WITH REFERENCE ON LHS
        //
        impl<'a, T> $operator_trait<$type_rhs> for &'a $type
            where T: Base + $operator_trait<Output=T>
        {
            type Output = $result_type;

            #[inline]
            fn $function(self, other: $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, other);
                $body
            }
        }
    }
}

macro_rules! implement_binary_operator_specific {
    ($operator_trait:ident<$type_rhs:ty> for $type:ty,
        fn $function:ident($lhs:ident, $rhs:ident) -> $result_type:ty {
            $body:expr
        }
    ) => {
        //
        // IMPLEMENT OPERATOR FOR TYPE WITHOUT REFERENCE
        //
        impl $operator_trait<$type_rhs> for $type {
            type Output = $result_type;

            #[inline]
            fn $function(self, other: $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }

        //
        // IMPLEMENT OPERATOR FOR TYPE WITH REFERENCE
        //
        impl<'a> $operator_trait<&'a $type_rhs> for &'a $type {
            type Output = $result_type;

            #[inline]
            fn $function(self, other: &'a $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, *other);
                $body
            }
        }

        //
        // IMPLEMENT OPERATOR FOR TYPE WITH REFERENCE ON RHS
        //
        impl<'a> $operator_trait<&'a $type_rhs> for $type {
            type Output = $result_type;

            #[inline]
            fn $function(self, other: &'a $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (self, *other);
                $body
            }
        }

        //
        // IMPLEMENT OPERATOR FOR TYPE WITH REFERENCE ON LHS
        //
        impl<'a> $operator_trait<$type_rhs> for &'a $type {
            type Output = $result_type;

            #[inline]
            fn $function(self, other: $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, other);
                $body
            }
        }
    }
}

macro_rules! implement_vector {
    ($type:ident { $($member:ident),+ }) => {
        //
        // DEFINE THE TYPE
        //
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        pub struct $type<T>
            where T: Base
        {
            $($member: T),*
        }

        //
        // IMPLEMENTATION WHEN BASE TRAIT
        //
        impl<T> $type<T> 
            where T: Base
        {
            pub fn new($($member: T),*) -> Self {
                $type {
                    $($member: $member),*
                }
            }

            #[inline]
            pub fn dims() -> usize {
                let mut count = 0;
                $(let $member = 1; count += $member;)*
                count
            }

            #[inline]
            pub fn zero() -> Self {
                $type {
                    $($member: T::zero()),*
                }
            }

            #[inline]
            pub fn one() -> Self {
                $type {
                    $($member: T::one()),*
                }
            }

            #[inline]
            pub fn cw_min(&self, other: Self) -> Self
                where T: PartialOrd
            {
                $type::new(
                    $(if self.$member < other.$member { self.$member } else { other.$member }),*
                )
            }

            #[inline]
            pub fn cw_max(&self, other: Self) -> Self 
                where T: PartialOrd
            {
                $type::new(
                    $(if other.$member < self.$member { other.$member } else { self.$member }),*
                )
            }

            #[inline]
            pub fn cw_abs(&self) -> Self 
                where T: Signed
            {
                $type::new(
                    $(self.$member.abs()),*
                )
            }
        }

        //
        // IMPLEMENTATION WHEN BASEFLOAT TRAIT
        //
        impl<T> $type<T>
            where T: BaseFloat
        {
            #[inline]
            pub fn length(&self) -> T 
                where T: AddAssign + Mul<Output=T>
            {
                let mut sum = T::zero();
                $(sum += self.$member * self.$member;)*
                T::sqrt(sum)
            }

            #[inline]
            pub fn normalize(&self) -> $type<T> 
                where T: Div<Output=T> + Mul<Output=T>
            {
                self / self.length()
            }
        }

        //
        // DEFAULT TRAIT
        //
        impl<T> Default for $type<T> 
            where T: Base
        {
            fn default() -> Self {
                $type::new(
                    $({ let $member = T::zero(); $member }),*
                )
            }
        }

        //
        // NEGATE OPERATOR
        //
        impl<T> Neg for $type<T> 
            where T: Base + Neg<Output=T>
        {
            type Output = $type<T>;

            fn neg(self) -> $type<T> {
                $type::new(
                    $(-self.$member),*
                )
            }
        }

        //
        // IMPLEMENT BINARY OPERATORS
        //

        // v + v
        implement_binary_operator!(Add<$type<T>> for $type<T>,
            fn add(lhs, rhs) -> $type<T> {
                $type::new( $(lhs.$member + rhs.$member),* )
            }
        );

        // v += v
        implement_binary_assign_operator!(AddAssign<$type<T>> for $type<T>,
            fn add_assign(lhs, rhs) {{
                $(lhs.$member += rhs.$member;)*
            }}
        );

        // v - v
        implement_binary_operator!(Sub<$type<T>> for $type<T>,
            fn sub(lhs, rhs) -> $type<T> {
                $type::new( $(lhs.$member - rhs.$member),* )
            }
        );

        // v -= v
        implement_binary_assign_operator!(SubAssign<$type<T>> for $type<T>,
            fn sub_assign(lhs, rhs) {{
                $(lhs.$member -= rhs.$member;)*
            }}
        );

        // v * s
        implement_binary_operator!(Mul<T> for $type<T>,
            fn mul(vector, scalar) -> $type<T> {
                $type::new( $(vector.$member * scalar),* )
            }
        );

        // v *= s
        implement_binary_assign_operator!(MulAssign<T> for $type<T>,
            fn mul_assign(vector, scalar) {{
                $(vector.$member *= scalar;)*
            }}
        );

        // v * v
        implement_binary_operator!(Mul<$type<T>> for $type<T>,
            fn mul(lhs, rhs) -> $type<T> {
                $type::new( $(lhs.$member * rhs.$member),* )
            }
        );

        // v *= v
        implement_binary_assign_operator!(MulAssign<$type<T>> for $type<T>,
            fn mul_assign(lhs, rhs) {{
                $(lhs.$member *= rhs.$member;)*
            }}
        );

        // v / s
        implement_binary_operator!(Div<T> for $type<T>,
            fn div(vector, scalar) -> $type<T> {
                $type::new( $(vector.$member / scalar),* )
            }
        );

        // v /= s
        implement_binary_assign_operator!(DivAssign<T> for $type<T>,
            fn div_assign(vector, scalar) {{
                $(vector.$member /= scalar;)*
            }}
        );

        // v / v
        implement_binary_operator!(Div<$type<T>> for $type<T>,
            fn div(lhs, rhs) -> $type<T> {
                $type::new( $(lhs.$member / rhs.$member),* )
            }
        );

        // v /= s
        implement_binary_assign_operator!(DivAssign<$type<T>> for $type<T>,
            fn div_assign(lhs, rhs) {{
                $(lhs.$member /= rhs.$member;)*
            }}
        );





        implement_binary_operator_specific!(Mul<$type<f32>> for f32,
            fn mul(scalar, vector) -> $type<f32> {
                $type::new(
                    $(vector.$member * scalar),*
                )
            }
        );
    }
}

implement_vector!(Vector2 { x, y });
implement_vector!(Vector3 { x, y, z });
implement_vector!(Vector4 { x, y, z, w });


impl<T> Vector3<T> 
    where T: Base + Sub<Output=T> + Mul<T, Output=T>
{
    #[inline]
    pub fn cross(&self, other: &Vector3<T>) -> Vector3<T> {
        let (a, b) = (self, other);
        Vector3::new(
            a.y*b.z - a.z*b.y,
            a.z*b.x - a.x*b.z,
            a.x*b.y - a.y*b.x,
        )
    }
}