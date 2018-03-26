use std::ops::{Mul,Neg};
use std::mem;

use traits::*;
use vector::*;

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
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
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

        // TODO(henk): Add MatrixAccessRef und MatrixAccessMut
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

impl<T: Base + Neg<Output=T>> Matrix4<T> {
    pub fn det(&self) -> T {
        let m = |r,c| self.row_elem(r, c);
        m(0, 0) * m(1, 1) * m(2, 2) * m(3, 3) + m(0, 0) * m(2, 1) * m(3, 2) * m(1, 3) + m(0, 0) * m(3, 1) * m(1, 2) * m(2, 3) +
        m(1, 0) * m(0, 1) * m(3, 2) * m(2, 3) + m(1, 0) * m(2, 1) * m(0, 2) * m(3, 3) + m(1, 0) * m(3, 1) * m(2, 2) * m(0, 3) +
        m(2, 0) * m(0, 1) * m(1, 2) * m(3, 3) + m(2, 0) * m(1, 1) * m(3, 2) * m(0, 3) + m(2, 0) * m(3, 1) * m(0, 2) * m(1, 3) +
        m(3, 0) * m(0, 1) * m(2, 2) * m(1, 3) + m(3, 0) * m(1, 1) * m(0, 2) * m(2, 3) + m(3, 0) * m(2, 1) * m(1, 2) * m(0, 3) -
        m(0, 0) * m(1, 1) * m(3, 2) * m(2, 3) - m(0, 0) * m(2, 1) * m(1, 2) * m(3, 3) - m(0, 0) * m(3, 1) * m(2, 2) * m(1, 3) -
        m(1, 0) * m(0, 1) * m(2, 2) * m(3, 3) - m(1, 0) * m(2, 1) * m(3, 2) * m(0, 3) - m(1, 0) * m(3, 1) * m(0, 2) * m(2, 3) -
        m(2, 0) * m(0, 1) * m(3, 2) * m(1, 3) - m(2, 0) * m(1, 1) * m(0, 2) * m(3, 3) - m(2, 0) * m(3, 1) * m(1, 2) * m(0, 3) -
        m(3, 0) * m(0, 1) * m(1, 2) * m(2, 3) - m(3, 0) * m(1, 1) * m(2, 2) * m(0, 3) - m(3, 0) * m(2, 1) * m(0, 2) * m(1, 3)
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.det();
        if det == T::zero() { // TODO(henk): How to compare
            None
        } else {
            let d = T::one() / det;
            let m = |r,c| self.row_elem(r, c);
            Some(Matrix4::from_components_row_major(
                d*(m(2, 2) * m(3, 3) * m(4, 4) + m(3, 2) * m(4, 3) * m(2, 4) + m(4, 2) * m(2, 3) * m(3, 4) - m(2, 2) * m(4, 3) * m(3, 4) - m(3, 2) * m(2, 3) * m(4, 4) - m(4, 2) * m(3, 3) * m(2, 4)),
                d*(m(2, 1) * m(4, 3) * m(3, 4) + m(3, 1) * m(2, 3) * m(4, 4) + m(4, 1) * m(3, 3) * m(2, 4) - m(2, 1) * m(3, 3) * m(4, 4) - m(3, 1) * m(4, 3) * m(2, 4) - m(4, 1) * m(2, 3) * m(3, 4)),
                d*(m(2, 1) * m(3, 2) * m(4, 4) + m(3, 1) * m(4, 2) * m(2, 4) + m(4, 1) * m(2, 2) * m(3, 4) - m(2, 1) * m(4, 2) * m(3, 4) - m(3, 1) * m(2, 2) * m(4, 4) - m(4, 1) * m(3, 2) * m(2, 4)),
                d*(m(2, 1) * m(4, 2) * m(3, 3) + m(3, 1) * m(2, 2) * m(4, 3) + m(4, 1) * m(3, 2) * m(2, 3) - m(2, 1) * m(3, 2) * m(4, 3) - m(3, 1) * m(4, 2) * m(2, 3) - m(4, 1) * m(2, 2) * m(3, 3)),
                d*(m(1, 2) * m(4, 3) * m(3, 4) + m(3, 2) * m(1, 3) * m(4, 4) + m(4, 2) * m(3, 3) * m(1, 4) - m(1, 2) * m(3, 3) * m(4, 4) - m(3, 2) * m(4, 3) * m(1, 4) - m(4, 2) * m(1, 3) * m(3, 4)),
                d*(m(1, 1) * m(3, 3) * m(4, 4) + m(3, 1) * m(4, 3) * m(1, 4) + m(4, 1) * m(1, 3) * m(3, 4) - m(1, 1) * m(4, 3) * m(3, 4) - m(3, 1) * m(1, 3) * m(4, 4) - m(4, 1) * m(3, 3) * m(1, 4)),
                d*(m(1, 1) * m(4, 2) * m(3, 4) + m(3, 1) * m(1, 2) * m(4, 4) + m(4, 1) * m(3, 2) * m(1, 4) - m(1, 1) * m(3, 2) * m(4, 4) - m(3, 1) * m(4, 2) * m(1, 4) - m(4, 1) * m(1, 2) * m(3, 4)),
                d*(m(1, 1) * m(3, 2) * m(4, 3) + m(3, 1) * m(4, 2) * m(1, 3) + m(4, 1) * m(1, 2) * m(3, 3) - m(1, 1) * m(4, 2) * m(3, 3) - m(3, 1) * m(1, 2) * m(4, 3) - m(4, 1) * m(3, 2) * m(1, 3)),
                d*(m(1, 2) * m(2, 3) * m(4, 4) + m(2, 2) * m(4, 3) * m(1, 4) + m(4, 2) * m(1, 3) * m(2, 4) - m(1, 2) * m(4, 3) * m(2, 4) - m(2, 2) * m(1, 3) * m(4, 4) - m(4, 2) * m(2, 3) * m(1, 4)),
                d*(m(1, 1) * m(4, 3) * m(2, 4) + m(2, 1) * m(1, 3) * m(4, 4) + m(4, 1) * m(2, 3) * m(1, 4) - m(1, 1) * m(2, 3) * m(4, 4) - m(2, 1) * m(4, 3) * m(1, 4) - m(4, 1) * m(1, 3) * m(2, 4)),
                d*(m(1, 1) * m(2, 2) * m(4, 4) + m(2, 1) * m(4, 2) * m(1, 4) + m(4, 1) * m(1, 2) * m(2, 4) - m(1, 1) * m(4, 2) * m(2, 4) - m(2, 1) * m(1, 2) * m(4, 4) - m(4, 1) * m(2, 2) * m(1, 4)),
                d*(m(1, 1) * m(4, 2) * m(2, 3) + m(2, 1) * m(1, 2) * m(4, 3) + m(4, 1) * m(2, 2) * m(1, 3) - m(1, 1) * m(2, 2) * m(4, 3) - m(2, 1) * m(4, 2) * m(1, 3) - m(4, 1) * m(1, 2) * m(2, 3)),
                d*(m(1, 2) * m(3, 3) * m(2, 4) + m(2, 2) * m(1, 3) * m(3, 4) + m(3, 2) * m(2, 3) * m(1, 4) - m(1, 2) * m(2, 3) * m(3, 4) - m(2, 2) * m(3, 3) * m(1, 4) - m(3, 2) * m(1, 3) * m(2, 4)),
                d*(m(1, 1) * m(2, 3) * m(3, 4) + m(2, 1) * m(3, 3) * m(1, 4) + m(3, 1) * m(1, 3) * m(2, 4) - m(1, 1) * m(3, 3) * m(2, 4) - m(2, 1) * m(1, 3) * m(3, 4) - m(3, 1) * m(2, 3) * m(1, 4)),
                d*(m(1, 1) * m(3, 2) * m(2, 4) + m(2, 1) * m(1, 2) * m(3, 4) + m(3, 1) * m(2, 2) * m(1, 4) - m(1, 1) * m(2, 2) * m(3, 4) - m(2, 1) * m(3, 2) * m(1, 4) - m(3, 1) * m(1, 2) * m(2, 4)),
                d*(m(1, 1) * m(2, 2) * m(3, 3) + m(2, 1) * m(3, 2) * m(1, 3) + m(3, 1) * m(1, 2) * m(2, 3) - m(1, 1) * m(3, 2) * m(2, 3) - m(2, 1) * m(1, 2) * m(3, 3) - m(3, 1) * m(2, 2) * m(1, 3))
            ))
        }
    }
}


