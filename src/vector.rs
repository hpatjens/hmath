use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,Neg};
use num_traits::{Num,Zero,One};
use num_traits::float::Float;

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
            fn zero() -> Self {
                $type {
                    $($member: T::zero()),*
                }
            }

            #[inline]
            fn one() -> Self {
                $type {
                    $($member: T::one()),*
                }
            }

            #[inline]
            fn cw_min(&self, other: Self) -> Self {
                unimplemented!()
            }

            #[inline]
            fn cw_max(&self, other: Self) -> Self {
                unimplemented!()
            }

            #[inline]
            fn cw_abs(&self, other: Self) -> Self {
                unimplemented!()
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

#[cfg(test)]
mod tests {
    use super::*;

    type Vec4  = Vector4<f32>;
    type Vec4i = Vector4<i32>;
    type Vec4u = Vector4<u32>;
    type Vec2  = Vector2<f32>;
    type Vec2i = Vector2<i32>;
    type Vec2u = Vector2<u32>;
    type Vec3  = Vector3<f32>;
    type Vec3i = Vector3<i32>;
    type Vec3u = Vector3<u32>;

    // --------------------------------------------------------------------------
    // Vector2
    // --------------------------------------------------------------------------

    #[test]
    fn vector2_add() {
        assert_eq!(Vec2::new(1.0, 2.0) + Vec2::new(2.0, 3.0), 
                   Vec2::new(3.0, 5.0));
    }

    #[test]
    fn vector2_add_ref() {
        let ref v1 = Vec2::new(1.0, 2.0);
        let ref v2 = Vec2::new(2.0, 3.0);
        assert_eq!(v1 + v2, 
                   Vec2::new(3.0, 5.0));
    }

    #[test]
    fn vector2_add_ref_rhs() {
        let ref v2 = Vec2::new(2.0, 3.0);
        assert_eq!(Vec2::new(1.0, 2.0) + v2, 
                   Vec2::new(3.0, 5.0));
    }

    #[test]
    fn vector2_add_ref_lhs() {
        let ref v1 = Vec2::new(1.0, 2.0);
        assert_eq!(v1 + Vec2::new(2.0, 3.0), 
                   Vec2::new(3.0, 5.0));
    }

    #[test]
    fn vector2_add_assign() {
        let mut v = Vec2::new(1.0, 2.0);
        v += Vec2::new(2.0, 3.0);
        assert_eq!(v, Vec2::new(3.0, 5.0));
    }

    #[test]
    fn vector2_sub() {
        assert_eq!(Vec2::new(2.0, 5.0) - Vec2::new(1.0, 3.0),
                   Vec2::new(1.0, 2.0));
    }

    #[test]
    fn vector2_sub_ref() {
        let ref v1 = Vec2::new(2.0, 5.0);
        let ref v2 = Vec2::new(1.0, 3.0);
        assert_eq!(v1 - v2,
                   Vec2::new(1.0, 2.0));
    }

    #[test]
    fn vector2_sub_lhs() {
        assert_eq!(2.0 * Vec2::new(1.0, 2.0),
                   Vec2::new(2.0, 4.0));
    }

    #[test]
    fn vector2_sub_lhs_ref() {
        let ref s = 2.0;
        let ref v = Vec2::new(1.0, 2.0);
        assert_eq!(s * v,
                   Vec2::new(2.0, 4.0));
    }

    #[test]
    fn vector2_sub_assign() {
        let mut v = Vec2::new(2.0, 5.0); 
        v -= Vec2::new(1.0, 3.0);
        assert_eq!(v,
                   Vec2::new(1.0, 2.0));
    }

    #[test]
    fn vector2_mul_rhs() {
        assert_eq!(Vec2::new(1.0, 2.0) * 2.0,
                   Vec2::new(2.0, 4.0));
    }

    #[test]
    fn vector2_mul_rhs_ref() {
        let ref s = 2.0;
        let ref v = Vec2::new(1.0, 2.0);
        assert_eq!(v * s,
                   Vec2::new(2.0, 4.0));
    }

    #[test]
    fn vector2_mul_assign_rhs() {
        let mut v = Vec2::new(1.0, 2.0);
        v *= 2.0;
        assert_eq!(v,
                   Vec2::new(2.0, 4.0));
    }

    #[test]
    fn vector2_mul_vec() {
        assert_eq!(Vec2::new(1.0, 2.0) * Vec2::new(2.0, 4.0),
                   Vec2::new(2.0, 8.0));
    }

    #[test]
    fn vector2_mul_assign_vec() {
        let mut v = Vec2::new(1.0, 2.0);
        v *= Vec2::new(2.0, 4.0);
        assert_eq!(v,
                   Vec2::new(2.0, 8.0));
    }

    #[test]
    fn vector2_div() {
        assert_eq!(Vec2::new(4.0, 6.0) / 2.0,
                   Vec2::new(2.0, 3.0));
    }

    #[test]
    fn vector2_div_ref() {
        let ref v = Vec2::new(4.0, 6.0);
        assert_eq!(v / 2.0,
                   Vec2::new(2.0, 3.0));
    }

    #[test]
    fn vector2_div_assign() {
        let mut v = Vec2::new(4.0, 6.0);
        v /= 2.0;
        assert_eq!(v,
                   Vec2::new(2.0, 3.0));
    }

    #[test]
    fn vector2_div_vec() {
        assert_eq!(Vec2::new(4.0, 6.0) / Vec2::new(2.0, 3.0),
                   Vec2::new(2.0, 2.0));
    }

    #[test]
    fn vector2_div_assign_vec() {
        let mut v = Vec2::new(4.0, 6.0);
        v /= Vec2::new(2.0, 3.0);
        assert_eq!(v,
                   Vec2::new(2.0, 2.0));
    }

    #[test]
    fn vector2_zero() {
        assert_eq!(Vec2::zero(),
                   Vec2::new(0.0, 0.0));
    }

    #[test]
    fn vector2_one() {
        assert_eq!(Vec2::one(),
                   Vec2::new(1.0, 1.0));
    }

    #[test]
    fn vector2_neg() {
        assert_eq!(-Vec2::new( 1.0,  2.0),
                    Vec2::new(-1.0, -2.0));
    }

    #[test]
    fn vector2_length1() {
        assert_eq!(Vec2::new(1.0, 0.0).length(),
                   1.0);
    }

    #[test]
    fn vector2_length2() {
        assert_eq!(Vec2::new(3.0, 4.0).length(),
                   5.0);
    }

    #[test]
    fn vector2_normalize1() {
        assert_eq!(Vec2::new(13.642102, 0.0).normalize(),
                   Vec2::new(1.0, 0.0));
    }

    #[test]
    fn vector2_normalize2() {
        assert_eq!(Vec2::new(9.197, 9.197).normalize(),
                   Vec2::new(1.546, 1.546).normalize());
    }

    // --------------------------------------------------------------------------
    // Vector3
    // --------------------------------------------------------------------------

    #[test]
    fn vector3_add() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) + Vec3::new(2.0, 3.0, 4.0),
                   Vec3::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn vector3_add_ref() {
        let ref v1 = Vec3::new(1.0, 2.0, 3.0);
        let ref v2 = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(v1 + v2,
                   Vec3::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn vector3_sub() {
        assert_eq!(Vec3::new(2.0, 5.0, 7.0) - Vec3::new(1.0, 3.0, 4.0),
                   Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn vector3_mul_rhs() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) * 2.0,
                   Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn vector3_mul_lhs() {
        assert_eq!(2.0 * Vec3::new(1.0, 2.0, 3.0),
                   Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn vector3_mul_ref() {
        let ref v = Vec3::new(1.0, 2.0, 3.0);
        let ref s = 2.0;
        assert_eq!(v * s,
                   Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn vector3_zero() {
        assert_eq!(Vec3::zero(),
                   Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn vector3_one() {
        assert_eq!(Vec3::one(),
                   Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn vector3_cross() {
        let ref v1 = Vec3::new(1.0, 0.0, 0.0);
        let ref v2 = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(v1.cross(v2),
                   Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn vector3_add_complex_fns() {
        struct App {
            vector: Vec3,
        }
        let mut app = App {
            vector: Vec3::zero(),
        };
        fn update(app: &mut App) {
            app.vector += Vec3::new(1.0, 2.0, 2.0) + Vec3::new(0.0, 0.0, 1.0);
            app.vector -= Vec3::new(0.0, 1.0, 2.0) - Vec3::new(0.0, 1.0, 0.0);
            app.vector *= 2.0 * 2.0;
            app.vector /= 4.0 / 2.0;
        }
        update(&mut app);
        assert_eq!(app.vector, Vec3::new(2.0, 4.0, 2.0));
    }
}