use std::{
    ops::{Mul,Neg},
    mem,
};

use crate::{
    traits::*,
    vector::*,
};

use serde::{Serialize, Deserialize};

pub use float_cmp::{Ulps,ApproxEq};

macro_rules! implement_one_to_one_assign_method {
    (fn $method_name:ident -> $type:ident { $($member:ident),* }) => {
        pub fn $method_name($($member: T),*) -> Self {
            $type { $($member: $member),* }
        }   
    }
}

macro_rules! implement_matrix {
    ($matrix_type:ident {
        dim: $dim:expr,
        vector_type: $vector_type:ident,
        cols: { 
            $($c_num:expr => $c:ident { $($ce_num:expr => $m_col_element:ident: $col_vec_member:ident),*}),*
        },
        rows: { 
            $($r_num:expr => $r:ident { $($re_num:expr => $m_row_element:ident: $row_vec_member:ident),* }),* 
        }
    }) => {
        #[repr(C)]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
        pub struct $matrix_type<T: Base> {
            $($(pub $m_col_element: T),*),*
        }

        impl<T: Base> $matrix_type<T> {
            #[inline]
            pub fn dims() -> usize {
                ($({let $c = 1; $c} +)* 0)
            }

            implement_one_to_one_assign_method!(
                fn from_components_row_major -> $matrix_type { 
                    $($($m_row_element),*),*
                }
            );

            implement_one_to_one_assign_method!(
                fn from_components_col_major -> $matrix_type { 
                    $($($m_col_element),*),*
                }
            );

            pub fn from_cols($($c: $vector_type<T>),*) -> Self {
                $matrix_type::from_components_col_major($($($c.$col_vec_member),*),*)
            }

            pub fn from_rows($($r: $vector_type<T>),*) -> Self {
                $matrix_type::from_components_row_major($($($r.$row_vec_member),*),*)
            }

            pub fn transpose(&self) -> Self {
                $matrix_type::from_components_row_major($($(self.$m_col_element),*),*)
            }

            pub fn col(&self, index: usize) -> $vector_type<T> {
                match index {
                    $($c_num => $vector_type::new( $(self.$m_col_element),* ),)*
                    _ => panic!("index out of range"),
                }
            }

            pub fn row(&self, index: usize) -> $vector_type<T> {
                match index {
                    $($r_num => $vector_type::new( $(self.$m_row_element),* ),)*
                    _ => panic!("index out of range")
                }
            }

            pub fn set_col_elem(&mut self, col_index: usize, elem_index: usize, value: T) {
                self.as_array()[col_index*$dim + elem_index] = value;
            }

            pub fn identity() -> Self {
                $matrix_type::from_diagonal($vector_type::one())
            }

            fn from_value(value: T) -> Self {
                $matrix_type::from_components_col_major($($({ let $m_col_element = value; $m_col_element}),*),*)
            }

            pub fn zero() -> Self {
                $matrix_type::from_value(T::zero())
            }

            pub fn one() -> Self {
                $matrix_type::from_value(T::one())
            }

            #[inline]
            pub fn to_array(&self) -> [T; $dim*$dim] {
                unsafe { mem::transmute_copy(self) }
            }
        }

        // m * m
        implement_binary_operator!(Mul<$matrix_type<T>> for $matrix_type<T>,
            fn mul(lhs, rhs) -> $matrix_type<T> {
                unsafe {
                    let mut res: $matrix_type<T> = mem::uninitialized();
                    for r in 0..$dim {
                        for c in 0..$dim {
                            let col = rhs.col(c);
                            let row = lhs.row(r);
                            res.set_col_elem(c, r, col.dot(row));
                        }
                    }
                    res
                }
            }
        );

        // m * v
        implement_binary_operator!(Mul<$vector_type<T>> for $matrix_type<T>,
            fn mul(lhs, rhs) -> $vector_type<T> {
                $vector_type::new(
                    $(lhs.row($r_num).dot(rhs)),*
                )
            }
        );

        impl<'a, T: Base> AsArray for &'a $matrix_type<T> {
            type Output = &'a [T; $dim*$dim];
            #[inline]
            fn as_array(self) -> Self::Output {
                unsafe { mem::transmute(self) }
            }
        }

        impl<'a, T: Base> AsArray for &'a mut $matrix_type<T> {
            type Output = &'a mut [T; $dim*$dim];
            #[inline]
            fn as_array(self) -> Self::Output {
                unsafe { mem::transmute(self) }
            }
        }

        impl<'a, T: Base> AsArrays for &'a $matrix_type<T> {
            type Output = &'a [[T; $dim]; $dim];
            #[inline]
            fn as_arrays(self) -> Self::Output {
                unsafe { mem::transmute(self) }
            }
        }

        impl<'a, T: Base> AsArrays for &'a mut $matrix_type<T> {
            type Output = &'a mut [[T; $dim]; $dim];
            #[inline]
            fn as_arrays(self) -> Self::Output {
                unsafe { mem::transmute(self) }
            }
        }

        impl<T: Base> MatrixAccess for $matrix_type<T> {
            type Output = T;
            #[inline]
            fn col_elem(self, col_index: usize, elem_index: usize) -> Self::Output {
                self.as_array()[col_index*$dim + elem_index]
            }
            #[inline]
            fn row_elem(self, row_index: usize, elem_index: usize) -> Self::Output {
                self.as_array()[elem_index*$dim + row_index]
            }
        }

        impl<'a, T: Base> MatrixAccess for &'a $matrix_type<T> {
            type Output = T;
            #[inline]
            fn col_elem(self, col_index: usize, elem_index: usize) -> Self::Output {
                self.as_array()[col_index*$dim + elem_index]
            }
            #[inline]
            fn row_elem(self, row_index: usize, elem_index: usize) -> Self::Output {
                self.as_array()[elem_index*$dim + row_index]
            }
        }

        impl<'a, T: Base> MatrixAccess for &'a mut $matrix_type<T> {
            type Output = T;
            #[inline]
            fn col_elem(self, col_index: usize, elem_index: usize) -> Self::Output {
                self.as_array()[col_index*$dim + elem_index]
            }
            #[inline]
            fn row_elem(self, row_index: usize, elem_index: usize) -> Self::Output {
                self.as_array()[elem_index*$dim + row_index]
            }
        }

        impl<'a, T: Base> MatrixAccessRef<'a> for &'a $matrix_type<T> {
            type Output = T;
            #[inline]
            fn col_elem_ref(self, col_index: usize, elem_index: usize) -> &'a Self::Output {
                &self.as_array()[col_index*$dim + elem_index]
            }
            #[inline]
            fn row_elem_ref(self, row_index: usize, elem_index: usize) -> &'a Self::Output {
                &self.as_array()[elem_index*$dim + row_index]
            }
        }

        impl<'a, T: Base> MatrixAccessRef<'a> for &'a mut $matrix_type<T> {
            type Output = T;
            #[inline]
            fn col_elem_ref(self, col_index: usize, elem_index: usize) -> &'a Self::Output {
                &self.as_array()[col_index*$dim + elem_index]
            }
            #[inline]
            fn row_elem_ref(self, row_index: usize, elem_index: usize) -> &'a Self::Output {
                &self.as_array()[elem_index*$dim + row_index]
            }
        }

        impl<'a, T: Base> MatrixAccessMut<'a> for &'a mut $matrix_type<T> {
            type Output = T;
            #[inline]
            fn col_elem_mut(self, col_index: usize, elem_index: usize) -> &'a mut Self::Output {
                &mut self.as_array()[col_index*$dim + elem_index]
            }
            #[inline]
            fn row_elem_mut(self, row_index: usize, elem_index: usize) -> &'a mut Self::Output {
                &mut self.as_array()[elem_index*$dim + row_index]
            }
        }

        impl<T: BaseFloat> ApproxEq for $matrix_type<T> 
            where T: ApproxEq<Flt=T>
        {
            type Flt = T;
            #[inline]
            fn approx_eq(&self, other: &Self, epsilon: T, ulps: <T as Ulps>::U) -> bool {
                //$($(self.$m_col_element.approx_eq(&other.$m_col_element, epsilon, ulps))&&*)&&*
                for i in 0..$dim*$dim {
                    if !self.as_array()[i].approx_eq(&other.as_array()[i], epsilon, ulps) {
                        return false;
                    }
                }
                return true;
            }
        }
    }
}

implement_matrix!(
    Matrix2 {
        dim: 2,
        vector_type: Vector2,
        cols: {  
            0 => c0 { 0 => m00: x, 1 => m10: y },
            1 => c1 { 0 => m01: x, 1 => m11: y }
        },
        rows: {
            0 => r0 { 0 => m00: x, 1 => m01: y },
            1 => r1 { 0 => m10: x, 1 => m11: y }
        }
    }
);

implement_matrix!(
    Matrix3 {
        dim: 3,
        vector_type: Vector3,
        cols: {
            0 => c0 { 0 => m00: x, 1 => m10: y, 2 => m20: z },
            1 => c1 { 0 => m01: x, 1 => m11: y, 2 => m21: z },
            2 => c2 { 0 => m02: x, 1 => m12: y, 2 => m22: z }
        },
        rows: {
            0 => r0 { 0 => m00: x, 1 => m01: y, 2 => m02: z },
            1 => r1 { 0 => m10: x, 1 => m11: y, 2 => m12: z },
            2 => r2 { 0 => m20: x, 1 => m21: y, 2 => m22: z }
        }
    }
);

implement_matrix!(
    Matrix4 {
        dim: 4,
        vector_type: Vector4,
        cols: {
            0 => c0 { 0 => m00: x, 1 => m10: y, 2 => m20: z, 3 => m30: w },
            1 => c1 { 0 => m01: x, 1 => m11: y, 2 => m21: z, 3 => m31: w },
            2 => c2 { 0 => m02: x, 1 => m12: y, 2 => m22: z, 3 => m32: w },
            3 => c3 { 0 => m03: x, 1 => m13: y, 2 => m23: z, 3 => m33: w }
        },
        rows: {
            0 => r0 { 0 => m00: x, 1 => m01: y, 2 => m02: z, 3 => m03: w },
            1 => r1 { 0 => m10: x, 1 => m11: y, 2 => m12: z, 3 => m13: w },
            2 => r2 { 0 => m20: x, 1 => m21: y, 2 => m22: z, 3 => m23: w },
            3 => r3 { 0 => m30: x, 1 => m31: y, 2 => m32: z, 3 => m33: w }
        }
    }
);

impl<T: Base> Matrix2<T> {
    pub fn from_diagonal(d: Vector2<T>) -> Self {
        Matrix2::from_components_row_major(
            d.x,       T::zero(),
            T::zero(), d.y      ,
        )
    }
}

impl<T: Base> Matrix3<T> {
    pub fn from_diagonal(d: Vector3<T>) -> Self {
        Matrix3::from_components_row_major(
            d.x,       T::zero(), T::zero(),
            T::zero(), d.y      , T::zero(),
            T::zero(), T::zero(), d.z      ,
        )
    }
}

impl<T: Base> Matrix4<T> {
    pub fn from_diagonal(d: Vector4<T>) -> Self {
        Matrix4::from_components_row_major(
            d.x,       T::zero(), T::zero(), T::zero(),
            T::zero(), d.y      , T::zero(), T::zero(),
            T::zero(), T::zero(), d.z      , T::zero(),
            T::zero(), T::zero(), T::zero(), d.w      ,
        )
    }
}

impl<T: Base + Neg<Output=T>> Matrix3<T> {
    pub fn det(&self) -> T {
        let m = |r,c| self.row_elem(r, c);
          m(0, 0) * m(1, 1) * m(2, 2)
        + m(0, 1) * m(1, 2) * m(2, 0)
        + m(0, 2) * m(1, 0) * m(2, 1)
        - m(2, 0) * m(1, 1) * m(0, 2)
        - m(2, 1) * m(1, 2) * m(0, 0)
        - m(2, 2) * m(1, 0) * m(0, 1)
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.det();
        if det == T::zero() { // TODO(henk): How to compare?
            None
        } else {
            let d = T::one() / det;
            let m = |r,c| self.row_elem(r, c);
            Some(Matrix3::from_components_row_major(
                d*(m(1, 1) * m(2, 2) - m(1, 2) * m(2, 1)),
                d*(m(0, 2) * m(2, 1) - m(0, 1) * m(2, 2)),
                d*(m(0, 1) * m(1, 2) - m(0, 2) * m(1, 1)),
                d*(m(1, 2) * m(2, 0) - m(1, 0) * m(2, 2)),
                d*(m(0, 0) * m(2, 2) - m(0, 2) * m(2, 0)),
                d*(m(0, 2) * m(1, 0) - m(0, 0) * m(1, 2)),
                d*(m(1, 0) * m(2, 1) - m(1, 1) * m(2, 0)),
                d*(m(0, 1) * m(2, 0) - m(0, 0) * m(2, 1)),
                d*(m(0, 0) * m(1, 1) - m(0, 1) * m(1, 0))
            ))
        }
    }
}

impl<T: BaseFloat + ApproxEq<Flt=T>> Matrix4<T> {
    pub fn inverse(&self) -> Option<Self> {
        let (m00, m01, m02, m03) = (self.row_elem(0, 0), self.row_elem(0, 1), self.row_elem(0, 2), self.row_elem(0, 3));
        let (m10, m11, m12, m13) = (self.row_elem(1, 0), self.row_elem(1, 1), self.row_elem(1, 2), self.row_elem(1, 3));
        let (m20, m21, m22, m23) = (self.row_elem(2, 0), self.row_elem(2, 1), self.row_elem(2, 2), self.row_elem(2, 3));
        let (m30, m31, m32, m33) = (self.row_elem(3, 0), self.row_elem(3, 1), self.row_elem(3, 2), self.row_elem(3, 3));

        let d = T::one() / self.det();

        // TODO: Test for zero!

        Some(Self::from_components_row_major(
            d*(m11*m22*m33 + m12*m23*m31 + m13*m21*m32 - m11*m23*m32 - m12*m21*m33 - m13*m22*m31),
            d*(m01*m23*m32 + m02*m21*m33 + m03*m22*m31 - m01*m22*m33 - m02*m23*m31 - m03*m21*m32),
            d*(m01*m12*m33 + m02*m13*m31 + m03*m11*m32 - m01*m13*m32 - m02*m11*m33 - m03*m12*m31),
            d*(m01*m13*m22 + m02*m11*m23 + m03*m12*m21 - m01*m12*m23 - m02*m13*m21 - m03*m11*m22),
            d*(m10*m23*m32 + m12*m20*m33 + m13*m22*m30 - m10*m22*m33 - m12*m23*m30 - m13*m20*m32),
            d*(m00*m22*m33 + m02*m23*m30 + m03*m20*m32 - m00*m23*m32 - m02*m20*m33 - m03*m22*m30),
            d*(m00*m13*m32 + m02*m10*m33 + m03*m12*m30 - m00*m12*m33 - m02*m13*m30 - m03*m10*m32),
            d*(m00*m12*m23 + m02*m13*m20 + m03*m10*m22 - m00*m13*m22 - m02*m10*m23 - m03*m12*m20),
            d*(m10*m21*m33 + m11*m23*m30 + m13*m20*m31 - m10*m23*m31 - m11*m20*m33 - m13*m21*m30),
            d*(m00*m23*m31 + m01*m20*m33 + m03*m21*m30 - m00*m21*m33 - m01*m23*m30 - m03*m20*m31),
            d*(m00*m11*m33 + m01*m13*m30 + m03*m10*m31 - m00*m13*m31 - m01*m10*m33 - m03*m11*m30),
            d*(m00*m13*m21 + m01*m10*m23 + m03*m11*m20 - m00*m11*m23 - m01*m13*m20 - m03*m10*m21),
            d*(m10*m22*m31 + m11*m20*m32 + m12*m21*m30 - m10*m21*m32 - m11*m22*m30 - m12*m20*m31),
            d*(m00*m21*m32 + m01*m22*m30 + m02*m20*m31 - m00*m22*m31 - m01*m20*m32 - m02*m21*m30),
            d*(m00*m12*m31 + m01*m10*m32 + m02*m11*m30 - m00*m11*m32 - m01*m12*m30 - m02*m10*m31),
            d*(m00*m11*m22 + m01*m12*m20 + m02*m10*m21 - m00*m12*m21 - m01*m10*m22 - m02*m11*m20)
        ))
    }
}

impl<T: Base + Neg<Output=T>> Matrix4<T> {
    pub fn det(&self) -> T {
        let (m00, m01, m02, m03) = (self.row_elem(0, 0), self.row_elem(0, 1), self.row_elem(0, 2), self.row_elem(0, 3));
        let (m10, m11, m12, m13) = (self.row_elem(1, 0), self.row_elem(1, 1), self.row_elem(1, 2), self.row_elem(1, 3));
        let (m20, m21, m22, m23) = (self.row_elem(2, 0), self.row_elem(2, 1), self.row_elem(2, 2), self.row_elem(2, 3));
        let (m30, m31, m32, m33) = (self.row_elem(3, 0), self.row_elem(3, 1), self.row_elem(3, 2), self.row_elem(3, 3));
        
        m00*m11*m22*m33 + m00*m12*m23*m31 + m00*m13*m21*m32 +
        m01*m10*m23*m32 + m01*m12*m20*m33 + m01*m13*m22*m30 +
        m02*m10*m21*m33 + m02*m11*m23*m30 + m02*m13*m20*m31 +
        m03*m10*m22*m31 + m03*m11*m20*m32 + m03*m12*m21*m30 -
        m00*m11*m23*m32 - m00*m12*m21*m33 - m00*m13*m22*m31 -
        m01*m10*m22*m33 - m01*m12*m23*m30 - m01*m13*m20*m32 -
        m02*m10*m23*m31 - m02*m11*m20*m33 - m02*m13*m21*m30 -
        m03*m10*m21*m32 - m03*m11*m22*m30 - m03*m12*m20*m31
    }

    pub fn inverse1(&self) -> Option<Self> {
        let det = self.det();
        if det == T::zero() { // TODO(henk): How to compare
            None
        } else {
            let d = T::one() / det;
            let m = |r,c| self.row_elem(r, c);
            Some(Matrix4::from_components_row_major(
                d*(m(1, 1) * m(2, 2) * m(3, 3) + m(2, 1) * m(3, 2) * m(1, 3) + m(3, 1) * m(1, 2) * m(2, 3) - m(1, 1) * m(3, 2) * m(2, 3) - m(2, 1) * m(1, 2) * m(3, 3) - m(3, 1) * m(2, 2) * m(1, 3)),
                d*(m(1, 0) * m(3, 2) * m(2, 3) + m(2, 0) * m(1, 2) * m(3, 3) + m(3, 0) * m(2, 2) * m(1, 3) - m(1, 0) * m(2, 2) * m(3, 3) - m(2, 0) * m(3, 2) * m(1, 3) - m(3, 0) * m(1, 2) * m(2, 3)),
                d*(m(1, 0) * m(2, 1) * m(3, 3) + m(2, 0) * m(3, 1) * m(1, 3) + m(3, 0) * m(1, 1) * m(2, 3) - m(1, 0) * m(3, 1) * m(2, 3) - m(2, 0) * m(1, 1) * m(3, 3) - m(3, 0) * m(2, 1) * m(1, 3)),
                d*(m(1, 0) * m(3, 1) * m(2, 2) + m(2, 0) * m(1, 1) * m(3, 2) + m(3, 0) * m(2, 1) * m(1, 2) - m(1, 0) * m(2, 1) * m(3, 2) - m(2, 0) * m(3, 1) * m(1, 2) - m(3, 0) * m(1, 1) * m(2, 2)),
                d*(m(0, 1) * m(3, 2) * m(2, 3) + m(2, 1) * m(0, 2) * m(3, 3) + m(3, 1) * m(2, 2) * m(0, 3) - m(0, 1) * m(2, 2) * m(3, 3) - m(2, 1) * m(3, 2) * m(0, 3) - m(3, 1) * m(0, 2) * m(2, 3)),
                d*(m(0, 0) * m(2, 2) * m(3, 3) + m(2, 0) * m(3, 2) * m(0, 3) + m(3, 0) * m(0, 2) * m(2, 3) - m(0, 0) * m(3, 2) * m(2, 3) - m(2, 0) * m(0, 2) * m(3, 3) - m(3, 0) * m(2, 2) * m(0, 3)),
                d*(m(0, 0) * m(3, 1) * m(2, 3) + m(2, 0) * m(0, 1) * m(3, 3) + m(3, 0) * m(2, 1) * m(0, 3) - m(0, 0) * m(2, 1) * m(3, 3) - m(2, 0) * m(3, 1) * m(0, 3) - m(3, 0) * m(0, 1) * m(2, 3)),
                d*(m(0, 0) * m(2, 1) * m(3, 2) + m(2, 0) * m(3, 1) * m(0, 2) + m(3, 0) * m(0, 1) * m(2, 2) - m(0, 0) * m(3, 1) * m(2, 2) - m(2, 0) * m(0, 1) * m(3, 2) - m(3, 0) * m(2, 1) * m(0, 2)),
                d*(m(0, 1) * m(1, 2) * m(3, 3) + m(1, 1) * m(3, 2) * m(0, 3) + m(3, 1) * m(0, 2) * m(1, 3) - m(0, 1) * m(3, 2) * m(1, 3) - m(1, 1) * m(0, 2) * m(3, 3) - m(3, 1) * m(1, 2) * m(0, 3)),
                d*(m(0, 0) * m(3, 2) * m(1, 3) + m(1, 0) * m(0, 2) * m(3, 3) + m(3, 0) * m(1, 2) * m(0, 3) - m(0, 0) * m(1, 2) * m(3, 3) - m(1, 0) * m(3, 2) * m(0, 3) - m(3, 0) * m(0, 2) * m(1, 3)),
                d*(m(0, 0) * m(1, 1) * m(3, 3) + m(1, 0) * m(3, 1) * m(0, 3) + m(3, 0) * m(0, 1) * m(1, 3) - m(0, 0) * m(3, 1) * m(1, 3) - m(1, 0) * m(0, 1) * m(3, 3) - m(3, 0) * m(1, 1) * m(0, 3)),
                d*(m(0, 0) * m(3, 1) * m(1, 2) + m(1, 0) * m(0, 1) * m(3, 2) + m(3, 0) * m(1, 1) * m(0, 2) - m(0, 0) * m(1, 1) * m(3, 2) - m(1, 0) * m(3, 1) * m(0, 2) - m(3, 0) * m(0, 1) * m(1, 2)),
                d*(m(0, 1) * m(2, 2) * m(1, 3) + m(1, 1) * m(0, 2) * m(2, 3) + m(2, 1) * m(1, 2) * m(0, 3) - m(0, 1) * m(1, 2) * m(2, 3) - m(1, 1) * m(2, 2) * m(0, 3) - m(2, 1) * m(0, 2) * m(1, 3)),
                d*(m(0, 0) * m(1, 2) * m(2, 3) + m(1, 0) * m(2, 2) * m(0, 3) + m(2, 0) * m(0, 2) * m(1, 3) - m(0, 0) * m(2, 2) * m(1, 3) - m(1, 0) * m(0, 2) * m(2, 3) - m(2, 0) * m(1, 2) * m(0, 3)),
                d*(m(0, 0) * m(2, 1) * m(1, 3) + m(1, 0) * m(0, 1) * m(2, 3) + m(2, 0) * m(1, 1) * m(0, 3) - m(0, 0) * m(1, 1) * m(2, 3) - m(1, 0) * m(2, 1) * m(0, 3) - m(2, 0) * m(0, 1) * m(1, 3)),
                d*(m(0, 0) * m(1, 1) * m(2, 2) + m(1, 0) * m(2, 1) * m(0, 2) + m(2, 0) * m(0, 1) * m(1, 2) - m(0, 0) * m(2, 1) * m(1, 2) - m(1, 0) * m(0, 1) * m(2, 2) - m(2, 0) * m(1, 1) * m(0, 2))
            ))
        }
    }
}

impl<T: BaseFloat> Matrix3<T> {
    pub fn new_normal_matrix(model_matrix: &Matrix3<T>) -> Option<Self> {
        if let Some(inv) = model_matrix.inverse() {
            Some(Matrix3::from(inv.transpose()))
        } else {
            None
        }
    }
}

impl<T: BaseFloat> From<Matrix4<T>> for Matrix3<T> {
    fn from(m: Matrix4<T>) -> Self {
        Matrix3::from_components_row_major(
            m.m00, m.m01, m.m02,
            m.m10, m.m11, m.m12,
            m.m20, m.m21, m.m22
        )
    }
}

impl<T: BaseFloat> Matrix4<T> {
    pub fn new_translation(x: T, y: T, z: T) -> Self {
        Matrix4::from_components_row_major(
            T::one() , T::zero(), T::zero(), x,
            T::zero(), T::one() , T::zero(), y,
            T::zero(), T::zero(), T::one() , z,
            T::zero(), T::zero(), T::zero(), T::one()
        )
    }

    pub fn new_translation_from_vector(v: Vector3<T>) -> Self {
        Self::new_translation(v.x, v.y, v.z)
    }

    pub fn new_scale(x: T, y: T, z: T) -> Self {
        Matrix4::from_components_row_major(
            x        , T::zero(), T::zero(), T::zero(),
            T::zero(), y        , T::zero(), T::zero(),
            T::zero(), T::zero(), z        , T::zero(),
            T::zero(), T::zero(), T::zero(), T::one(),
        )
    }

    pub fn new_scale_from_vector(v: Vector3<T>) -> Self {
        Self::new_scale(v.x, v.y, v.z)
    }

    pub fn new_scale_uniform(s: T) -> Self {
        Self::new_scale(s, s, s)
    }

    pub fn new_rotation(axis: Vector3<T>, angle: T) -> Self {
        let cos_a = T::cos(angle);
        let sin_a = T::sin(angle);
        let k = T::one() - cos_a;
        Matrix4::from_components_row_major(
            axis.x * axis.x * k + cos_a         , axis.x * axis.y * k - axis.z * sin_a, axis.x * axis.z * k + axis.y * sin_a, T::zero(),
            axis.y * axis.x * k + axis.z * sin_a, axis.y * axis.y * k + cos_a         , axis.y * axis.z * k - axis.x * sin_a, T::zero(),
            axis.z * axis.x * k - axis.y * sin_a, axis.z * axis.y * k + axis.x * sin_a, axis.z * axis.z * k + cos_a         , T::zero(),
            T::zero()                           , T::zero()                           , T::zero()                           , T::one()
        )
    }

    pub fn new_rotation_x(angle: T) -> Self {
        Matrix4::from_components_row_major(
            T::one() , T::zero()    ,  T::zero()    , T::zero(),
            T::zero(), T::cos(angle), -T::sin(angle), T::zero(),
            T::zero(), T::sin(angle),  T::cos(angle), T::zero(),
            T::zero(), T::zero()    ,  T::zero()    , T::one()
        )
    }    

    pub fn new_rotation_y(angle: T) -> Self {
        Matrix4::from_components_row_major(
             T::cos(angle), T::zero(), T::sin(angle), T::zero(),
             T::zero()    , T::one() , T::zero()    , T::zero(),
            -T::sin(angle), T::zero(), T::cos(angle), T::zero(),
             T::zero()    , T::zero(), T::zero()    , T::one()
        )
    }

    pub fn new_rotation_z(angle: T) -> Self {
        Matrix4::from_components_row_major(
            T::cos(angle), -T::sin(angle), T::zero(), T::zero(),
            T::sin(angle),  T::cos(angle), T::zero(), T::zero(),
            T::zero()    ,  T::zero()    , T::one() , T::zero(),
            T::zero()    ,  T::zero()    , T::zero(), T::one()
        )
    }

    pub fn new_perspective_from_bounds(left: T, right: T, bottom: T, top: T, z_near: T, z_far: T) -> Self {
        let two = T::one() + T::one();
        let w = right - left;
        let h = top - bottom;
        let d = z_far - z_near;
        Matrix4::from_components_row_major(
            two * z_near / w, T::zero()       , (right + left) / w   , T::zero(),
            T::zero()       , two * z_near / h, (top + bottom) / h   , T::zero(),
            T::zero()       , T::zero()       , -(z_far + z_near) / d, -two * z_far*z_near / d,
            T::zero()       , T::zero()       , -T::one()            , T::zero()
        )
    }

    pub fn new_perspective_from_fov(fov: T, aspect: T, z_near: T, z_far: T) -> Self {
        let pi = T::from(::std::f32::consts::PI).unwrap();
        let circ = T::from(360.0).unwrap();
        let right = z_near * T::tan(fov * pi / circ);
        let left = -right;
        let top = right / aspect;
        let bottom = -top;
        Self::new_perspective_from_bounds(left, right, bottom, top, z_near, z_far)
    }

    pub fn new_orthographic_from_bounds(left: T, right: T, bottom: T, top: T, z_near: T, z_far: T) -> Self {
        let two = T::one() + T::one();
        let w = right - left;
        let h = top - bottom;
        let d = z_far - z_near;
        Matrix4::from_components_row_major(
            two / w  , T::zero(), T::zero(), -(right + left) / w,
            T::zero(), two / h  , T::zero(), -(top + bottom) / h,
            T::zero(), T::zero(), -two / d , -(z_far + z_near) / d,
            T::zero(), T::zero(), T::zero(), T::one()
        )
    }

    pub fn new_orthographic_from_dimensions(width: T, height: T, z_near: T, z_far: T) -> Self {
        let two = T::one() + T::one();
        let d = z_far - z_near;
        Matrix4::from_components_row_major(
            two / width, T::zero()   , T::zero(), T::zero(),
            T::zero()  , two / height, T::zero(), T::zero(),
            T::zero()  , T::zero()   , -two / d , -(z_far + z_near) / d,
            T::zero()  , T::zero()   , T::zero(), T::one()
        )
    }

    pub fn new_look_at(eye: Vector3<T>, target: Vector3<T>, up: Vector3<T>) -> Self {
        let f = (target - eye).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f).normalize();
        Matrix4::from_components_row_major(
             s.x      ,  s.y     ,  s.z     , -(s.x * eye.x + s.y * eye.y + s.z * eye.z),
             u.x      ,  u.y     ,  u.z     , -(u.x * eye.x + u.y * eye.y + u.z * eye.z),
            -f.x      , -f.y     , -f.z     ,   f.x * eye.x + f.y * eye.y + f.z * eye.z ,
             T::zero(), T::zero(), T::zero(), T::one()
        )
    }
}
