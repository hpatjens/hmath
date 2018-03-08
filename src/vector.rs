use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,Neg};
use num_traits::Signed;

use traits::*;

macro_rules! implement_binary_assign_operator {
    ($operator_trait:ident<$type_rhs:ty> for $type:ty,
        fn $function:ident($lhs:ident, $rhs:ident) {
            $body:expr
        }
    ) => {
        // val
        impl<T> $operator_trait<$type_rhs> for $type where T: Base {
            fn $function(&mut self, other: $type_rhs) {
                let ($lhs, $rhs) = (self, other); $body
            }
        }

        // &val
        impl<'a, T> $operator_trait<&'a $type_rhs> for $type where T: Base {
            fn $function(&mut self, other: &'a $type_rhs) {
                let ($lhs, $rhs) = (self, *other); $body
            }
        }

        // &mut val
        impl<'a, T> $operator_trait<&'a mut $type_rhs> for $type where T: Base {
            fn $function(&mut self, other: &'a mut $type_rhs) {
                let ($lhs, $rhs) = (self, *other); $body
            }
        }
    }
}

macro_rules! implement_unary_operator {
    // With simple constraint
    ($operator_trait:ident for $type:ty where T: $contraint_trait:ident,
        fn $function:ident($self:ident) -> $result_type:ty {
            $body:expr
        }
    ) => {
        impl<T> $operator_trait for $type where T: Base + $contraint_trait {
            type Output = $type;
            #[inline]
            fn $function(self) -> $type { let $self = self; $body }
        }

        impl<'a, T> $operator_trait for &'a $type where T: Base + $contraint_trait {
            type Output = $type;
            #[inline]
            fn $function(self) -> $type { let $self = self; $body }
        }

        impl<'a, T> $operator_trait for &'a mut $type where T: Base + $contraint_trait {
            type Output = $type;
            #[inline]
            fn $function(self) -> $type { let $self = self; $body }
        }
    };
    // With constraint that defines "Output"
    ($operator_trait:ident for $type:ty where T: $contraint_trait:ident<Output=T>,
        fn $function:ident($self:ident) -> $result_type:ty {
            $body:expr
        }
    ) => {
        impl<T> $operator_trait for $type where T: Base + $contraint_trait<Output=T> {
            type Output = $type;
            #[inline]
            fn $function(self) -> $type { let $self = self; $body }
        }

        impl<'a, T> $operator_trait for &'a $type where T: Base + $contraint_trait<Output=T> {
            type Output = $type;
            #[inline]
            fn $function(self) -> $type { let $self = self; $body }
        }

        impl<'a, T> $operator_trait for &'a mut $type where T: Base + $contraint_trait<Output=T> {
            type Output = $type;
            #[inline]
            fn $function(self) -> $type { let $self = self; $body }
        }
    }
}
 
macro_rules! implement_binary_operator {
    ($operator_trait:ident<$type_rhs:ty> for $type:ty,
        fn $function:ident($lhs:ident, $rhs:ident) -> $result_type:ty {
            $body:expr
        }
    ) => {
        // self op other
        impl<T> $operator_trait<$type_rhs> for $type where T: Base {
            type Output = $result_type;
            #[inline] 
            fn $function(self, other: $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (self, other); $body
            }
        }

        // self op &other
        impl<'b, T> $operator_trait<&'b $type_rhs> for $type where T: Base {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'b $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (self, *other); $body
            }
        }

        // self op &mut other
        impl<'b, T> $operator_trait<&'b mut $type_rhs> for $type where T: Base {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'b mut $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (self, *other); $body
            }
        }

        // &self op other
        impl<'a, T> $operator_trait<$type_rhs> for &'a $type where T: Base {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, other); $body
            }
        }

        // &self op &other
        impl<'a, T> $operator_trait<&'a $type_rhs> for &'a $type where T: Base {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'a $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, *other); $body
            }
        }

        // &self op &mut other
        impl<'a, T> $operator_trait<&'a mut $type_rhs> for &'a $type where T: Base {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'a mut $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, *other); $body
            }
        }

        // &mut self op other
        impl<'a, T> $operator_trait<$type_rhs> for &'a mut $type where T: Base {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, other); $body
            }
        }

        // &mut self op &other
        impl<'a, 'b, T> $operator_trait<&'b $type_rhs> for &'a mut $type where T: Base {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'b $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, *other); $body
            }
        }

        // &mut self op &mut other
        impl<'a, 'b, T> $operator_trait<&'b mut $type_rhs> for &'a mut $type where T: Base {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'b mut $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, *other); $body
            }
        }
    }
}

macro_rules! implement_binary_operator_non_generic {
    ($operator_trait:ident<$type_rhs:ty> for $type:ty,
        fn $function:ident($lhs:ident, $rhs:ident) -> $result_type:ty {
            $body:expr
        }
    ) => {
        // self op other
        impl $operator_trait<$type_rhs> for $type {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (self, other); $body
            }
        }

        // self op &other
        impl<'b> $operator_trait<&'b $type_rhs> for $type {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'b $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (self, *other); $body
            }
        }

        // self op &mut other
        impl<'b> $operator_trait<&'b mut $type_rhs> for $type {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'b mut $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (self, *other); $body
            }
        }

        // &self op other
        impl<'a> $operator_trait<$type_rhs> for &'a $type {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, other); $body
            }
        }

        // &self op &other
        impl<'a, 'b> $operator_trait<&'b $type_rhs> for &'a $type {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'b $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, *other); $body
            }
        }

        // &self op &mut other
        impl<'a, 'b> $operator_trait<&'b mut $type_rhs> for &'a $type {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'b mut $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, *other); $body
            }
        }

        // &mut self op other
        impl<'a> $operator_trait<$type_rhs> for &'a mut $type {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, other); $body
            }
        }

        // &mut self op &other
        impl<'a, 'b> $operator_trait<&'b $type_rhs> for &'a mut $type {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'b $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, *other); $body
            }
        }

        // &mut self op &mut other
        impl<'a, 'b> $operator_trait<&'b mut $type_rhs> for &'a mut $type {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'b mut $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, *other); $body
            }
        }
    }
}

macro_rules! implement_specific_operators_for_vector {
    ($type:ident { $($member:ident),+ } for $specific_type:ty) => {
        // s * v
        implement_binary_operator_non_generic!(Mul<$type<$specific_type>> for $specific_type,
            fn mul(scalar, vector) -> $type<$specific_type> {
                $type::new( $(scalar * vector.$member),* )
            }
        );

        // s / v
        implement_binary_operator_non_generic!(Div<$type<$specific_type>> for $specific_type,
            fn div(scalar, vector) -> $type<$specific_type> {
                $type::new( $(scalar / vector.$member),* )
            }
        );
    }
}

macro_rules! implement_vector {
    ($type:ident { $($member:ident),+ }) => {
        //
        // DEFINE THE TYPE
        //
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        pub struct $type<T> where T: Base {
            $($member: T),*
        }

        //
        // IMPLEMENTATION WHEN BASE TRAIT
        //
        impl<T: Base> $type<T>  {
            pub fn new($($member: T),*) -> Self {
                $type { $($member: $member),* }
            }

            #[inline]
            pub fn dims() -> usize {
                let mut count = 0;
                $(let $member = 1; count += $member;)*
                count
            }

            #[inline]
            pub fn zero() -> Self {
                $type { $($member: T::zero()),* }
            }

            #[inline]
            pub fn one() -> Self {
                $type { $($member: T::one()),* }
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

        // --------------------------------------------------------------------------
        // Own traits
        // --------------------------------------------------------------------------

        // v cw_min v
        implement_binary_operator!(CwMin<$type<T>> for $type<T>,
            fn cw_min(lhs, rhs) -> $type<T> {
                $type::new( $(if lhs.$member < rhs.$member { lhs.$member } else { rhs.$member }),* )
            }
        );

        // v cw_max v
        implement_binary_operator!(CwMax<$type<T>> for $type<T>,
            fn cw_max(lhs, rhs) -> $type<T> {
                $type::new( $(if rhs.$member < lhs.$member { lhs.$member } else { rhs.$member }),* )
            }
        );

        // v dot v
        implement_binary_operator!(Dot<$type<T>> for $type<T>,
            fn dot(lhs, rhs) -> T {{
                let mut sum = T::zero();
                $(sum += lhs.$member * rhs.$member;)*
                sum
            }}
        );

        implement_unary_operator!(Neg for $type<T> where T: Neg<Output=T>,
            fn neg(this) -> $type<T> {
                $type::new(
                    $(-this.$member),*
                )
            }
        );

        implement_unary_operator!(CwAbs for $type<T> where T: Signed,
            fn cw_abs(this) -> $type<T> {
                $type::new( $(this.$member.abs()),* )
            }
        );

        // --------------------------------------------------------------------------
        // Operators that cannot be implemented generically
        // --------------------------------------------------------------------------

        implement_specific_operators_for_vector!($type { $($member),* } for i8 );
        implement_specific_operators_for_vector!($type { $($member),* } for i16);
        implement_specific_operators_for_vector!($type { $($member),* } for i32);
        implement_specific_operators_for_vector!($type { $($member),* } for i64);
        implement_specific_operators_for_vector!($type { $($member),* } for u8 );
        implement_specific_operators_for_vector!($type { $($member),* } for u16);
        implement_specific_operators_for_vector!($type { $($member),* } for u32);
        implement_specific_operators_for_vector!($type { $($member),* } for u64);
        implement_specific_operators_for_vector!($type { $($member),* } for f32);
        implement_specific_operators_for_vector!($type { $($member),* } for f64);
    }
}

implement_vector!(Vector2 { x, y });
implement_vector!(Vector3 { x, y, z });
implement_vector!(Vector4 { x, y, z, w });

implement_binary_operator!(Cross<Vector3<T>> for Vector3<T>,
    fn cross(a, b) -> Vector3<T> {
        Vector3::new(
            a.y*b.z - a.z*b.y,
            a.z*b.x - a.x*b.z,
            a.x*b.y - a.y*b.x,
        )
    }
);
