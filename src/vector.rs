use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,Neg};
use num_traits::Signed;
use std::mem;

use traits::*;

pub use float_cmp::{Ulps,ApproxEq};

macro_rules! implement_vector {
    ($type:ident {
        dim: $dim:expr,
        elems: { $($num:expr => $member:ident),+ }
    }) => {
        //
        // DEFINE THE TYPE
        //
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        pub struct $type<T> where T: Base {
            $(pub $member: T),*
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
                ($({let $member = 1; $member} +)* 0)
            }

            #[inline]
            pub fn zero() -> Self {
                $type { $($member: T::zero()),* }
            }

            #[inline]
            pub fn one() -> Self {
                $type { $($member: T::one()),* }
            }

            // TODO(henk): Explain that one can use .as_array().clone() to create a new array.
            #[inline]
            pub fn as_array(&self) -> &[T; $dim] {
                unsafe { mem::transmute(self) }
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

        // --------------------------------------------------------------------------
        // Elem Trait
        // --------------------------------------------------------------------------

        impl<T: Base> Elem<usize> for $type<T> {
            type Output = T;
            #[inline]
            fn elem(self, index: usize) -> Self::Output {
                match index {
                    $($num => self.$member,)*
                    _ => panic!("index out of range")
                }
            }
        }

        impl<'a, T: Base> Elem<usize> for &'a $type<T> {
            type Output = &'a T;
            #[inline]
            fn elem(self, index: usize) -> Self::Output {
                match index {
                    $($num => &self.$member,)*
                    _ => panic!("index out of range")
                }
            }
        }

        impl<'a, T: Base> Elem<usize> for &'a mut $type<T> {
            type Output = &'a mut T;
            #[inline]
            fn elem(self, index: usize) -> Self::Output {
                match index {
                    $($num => &mut self.$member,)*
                    _ => panic!("index out of range")
                }
            }
        }

        
        impl<T: BaseFloat> ApproxEq for $type<T> 
            where T: ApproxEq<Flt=T>
        {
            type Flt = T;
            #[inline]
            fn approx_eq(&self, other: &Self, epsilon: T, ulps: <T as Ulps>::U) -> bool {
                $(self.$member.approx_eq(&other.$member, epsilon, ulps))&&*
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
/*
        implement_binary_operator!(Elem<usize> for $type<T>,
            fn elem(this, index) -> T {{
                this.to_array()[index]
            }}
        );
*/
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

implement_vector!(Vector2 { dim: 2, elems: { 0 => x, 1 => y } });
implement_vector!(Vector3 { dim: 3, elems: { 0 => x, 1 => y, 2 => z } });
implement_vector!(Vector4 { dim: 4, elems: { 0 => x, 1 => y, 2 => z, 3 => w } });

implement_binary_operator!(Cross<Vector3<T>> for Vector3<T>,
    fn cross(a, b) -> Vector3<T> {
        Vector3::new(
            a.y*b.z - a.z*b.y,
            a.z*b.x - a.x*b.z,
            a.x*b.y - a.y*b.x,
        )
    }
);

impl<T: Base> Vector4<T> {
    pub fn wdiv(&self) -> Vector3<T> {
        Vector3::new(self.x/self.w, self.y/self.w, self.z/self.w)
    }
}
