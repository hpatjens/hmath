#[cfg(test)]
mod tests {
    use matrix::*;
    use vector::*;
    use traits::*;

    use std::f32;    

    type Mat2 = Matrix2<f32>;
    type Mat3 = Matrix3<f32>;
    type Mat4 = Matrix4<f32>;

    type Vec2 = Vector2<f32>;
    type Vec3 = Vector3<f32>;
    type Vec4 = Vector4<f32>;

    // --------------------------------------------------------------------------
    //
    // Matrix2
    //
    // --------------------------------------------------------------------------

    // --------------------------------------------------------------------------
    // Contructors
    // --------------------------------------------------------------------------

    #[test]
    fn matrix2_from_components_row_major() {
        let m1 = Mat2::from_components_row_major(1.0, 2.0, 3.0 ,4.0);
        let m2 = Mat2{ m00: 1.0, m01: 2.0, m10: 3.0, m11: 4.0 };
        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix2_from_components_col_major() {
        let m1 = Mat2::from_components_col_major(1.0, 2.0, 3.0, 4.0);
        let m2 = Mat2{ m00: 1.0, m10: 2.0, m01: 3.0, m11: 4.0 };
        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix2_from_cols() {
        let m1 = Mat2::from_cols(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0));
        let m2 = Mat2{ m00: 1.0, m10: 2.0, m01: 3.0, m11: 4.0 };
        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix2_from_rows() {
        let m1 = Mat2::from_rows(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0));
        let m2 = Mat2{ m00: 1.0, m01: 2.0, m10: 3.0, m11: 4.0 };
        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix2_identity() {
        let m1 = Mat2::identity();
        let m2 = Mat2{ m00: 1.0, m01: 0.0, m10: 0.0, m11: 1.0 };
        assert_eq!(m1, m2);
    }

    // --------------------------------------------------------------------------
    // access
    // --------------------------------------------------------------------------

    #[test]
    fn matrix2_col() {
        assert_eq!(Mat2::identity().col(0), Vec2::new(1.0, 0.0));
    }

    #[test]
    fn matrix2_row() {
        assert_eq!(Mat2::identity().row(1), Vec2::new(0.0, 1.0));
    }

    // --------------------------------------------------------------------------
    // transpose
    // --------------------------------------------------------------------------

    #[test]
    fn maxtrix2_from_transpose1() {
        let m1 = Mat2::from_components_row_major(1.0, 2.0, 3.0, 4.0);
        let m2 = Mat2::from_components_col_major(1.0, 2.0, 3.0, 4.0);
        assert_eq!(m1, m2.transpose());
    }

    #[test]
    fn maxtrix2_from_transpose2() {
        let m1 = Mat2::from_components_row_major(1.0, 2.0, 3.0, 4.0);
        let m2 = Mat2::from_components_col_major(1.0, 2.0, 3.0, 4.0);
        assert_eq!(m1.transpose(), m2);
    }

    #[test]
    fn maxtrix2_from_transpose3() {
        let m1 = Mat2::from_components_row_major(1.0, 2.0, 3.0, 4.0);
        assert_eq!(m1, m1.transpose().transpose());
    }

    // --------------------------------------------------------------------------
    // Mul
    // --------------------------------------------------------------------------

    #[test]
    fn matrix2_mulmv_val_val() {
        let m = Mat2::from_components_row_major(1.0, 2.0, 3.0, 4.0);
        let v = Vec2::new(5.0, 6.0);
        assert_eq!(m*v, Vec2::new(17.0, 39.0));
    }

    #[test]
    fn matrix2_mulmv_ref_mutref() {
        let ref m = Mat2::from_components_row_major(1.0, 2.0, 3.0, 4.0);
        let ref mut v = Vec2::new(5.0, 6.0);
        assert_eq!(m*v, Vec2::new(17.0, 39.0));
    }

    // --------------------------------------------------------------------------
    //
    // Matrix3
    //
    // --------------------------------------------------------------------------

    // --------------------------------------------------------------------------
    // Contructors
    // --------------------------------------------------------------------------

    #[test]
    fn matrix3_from_components_row_major() {
        let m1 = Mat3::from_components_row_major(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let m2 = Mat3{ m00: 1.0, m01: 2.0, m02: 3.0, m10: 4.0, m11: 5.0, m12: 6.0, m20: 7.0, m21: 8.0, m22: 9.0 };
        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix3_from_components_col_major() {
        let m1 = Mat3::from_components_col_major(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let m2 = Mat3{ m00: 1.0, m10: 2.0, m20: 3.0, m01: 4.0, m11: 5.0, m21: 6.0, m02: 7.0, m12: 8.0, m22: 9.0 };
        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix3_from_cols() {
        let m1 = Mat3::from_cols(Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0), Vec3::new(7.0, 8.0, 9.0));
        let m2 = Mat3{ m00: 1.0, m10: 2.0, m20: 3.0, m01: 4.0, m11: 5.0, m21: 6.0, m02: 7.0, m12: 8.0, m22: 9.0 };
        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix3_from_rows() {
        let m1 = Mat3::from_rows(Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0), Vec3::new(7.0, 8.0, 9.0));
        let m2 = Mat3{ m00: 1.0, m01: 2.0, m02: 3.0, m10: 4.0, m11: 5.0, m12: 6.0, m20: 7.0, m21: 8.0, m22: 9.0 };
        assert_eq!(m1, m2);
    }

    // --------------------------------------------------------------------------
    // col & row & elem
    // --------------------------------------------------------------------------

    #[test]
    fn matrix3_col() {
        let m = Mat3::from_components_col_major(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m.col(1), Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn matrix3_row() {
        let m = Mat3::from_components_row_major(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m.row(2), Vec3::new(7.0, 8.0, 9.0));
    }

    #[test]
    fn matrix3_elems() {
        let m = Mat3::from_components_row_major(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m.row(1).elem(1), 5.0);
    }

    // --------------------------------------------------------------------------
    // col_elem
    // --------------------------------------------------------------------------

    #[test]
    fn matrix3_col_elem() {
        let mut m = Mat3::from_components_col_major(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        m.set_col_elem(1, 0, 40.0);
        assert_eq!(m.col(1).elem(0), 40.0);
    }

    #[test]
    fn matrix3_col_elem_val_to_val() {
        let m = Mat3::from_components_col_major(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m.col_elem(1, 1), 5.0);
    }

    #[test]
    fn matrix3_col_elem_ref_to_val() {
        let ref m = Mat3::from_components_col_major(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m.col_elem(1, 1), 5.0);
    }

    #[test]
    fn matrix3_col_elem_ref_to_ref() {
        let ref m = Mat3::from_components_col_major(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m.col_elem_ref(1, 1), &5.0);
    }

    #[test]
    fn matrix3_col_elem_mutref_to_val() {
        let ref mut m = Mat3::from_components_col_major(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m.col_elem(1, 1), 5.0);
    }

    #[test]
    fn matrix3_col_elem_mutref_to_ref() {
        let ref mut m = Mat3::from_components_col_major(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m.col_elem_ref(1, 1), &5.0);
    }

    #[test]
    fn matrix3_col_elem_mutref_to_mutref() {
        let ref mut m = Mat3::from_components_col_major(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m.col_elem_mut(1, 1), &mut 5.0);
    }

    #[test]
    fn matrix3_set_col_elem1() {
        let mut m = Mat3::from_components_col_major(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        m.set_col_elem(0, 0, 10.0);
        m.set_col_elem(2, 1, 80.0);
        assert_eq!(m.col_elem(0, 0), 10.0);
        assert_eq!(m.col_elem(2, 1), 80.0);
    }

    #[test]
    fn matrix3_set_col_elem2() {
        let mut m = Mat3::zero();
        for c in 0..3 {
            for r in 0..3 {
                m.set_col_elem(c, r, 1.0);
            }
        }
        for c in 0..3 {
            for r in 0..3 {
                println!("now: {:?}", m.col_elem(c, r));
            }
        }
        println!("this one: {:?}", m);
        assert_eq!(m, Mat3::one());
    }

    // --------------------------------------------------------------------------
    // approx_eq
    // --------------------------------------------------------------------------

    #[test]
    fn matrix3_approx_eq1() {
        let     m1 = Mat3::from_components_col_major(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let mut m2 = m1.clone();
        m2.set_col_elem(1, 0, 4.0 + f32::EPSILON);
        assert!(m1.approx_eq(&m2, f32::EPSILON, 0));
    }

    #[test]
    fn matrix3_approx_eq2() {
        let     m1 = Mat3::from_components_col_major(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let mut m2 = m1.clone();
        m2.set_col_elem(1, 0, 4.0 + 3.0*f32::EPSILON);
        assert!(!m1.approx_eq(&m2, f32::EPSILON, 0));
    }

    // --------------------------------------------------------------------------
    // arrays
    // --------------------------------------------------------------------------

    #[test]
    fn matrix3_as_array() {
        let m = Mat3::from_components_col_major(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let a = m.as_array();
        assert_eq!(a[0], 1.0);
        assert_eq!(a[1], 2.0);
        assert_eq!(a[2], 3.0);
        assert_eq!(a[3], 4.0);
        assert_eq!(a[4], 5.0);
        assert_eq!(a[5], 6.0);
        assert_eq!(a[6], 7.0);
        assert_eq!(a[7], 8.0);
        assert_eq!(a[8], 9.0);
    }

    // --------------------------------------------------------------------------
    // transpose
    // --------------------------------------------------------------------------

    #[test]
    fn matrix3_from_transpose1() {
        let m1 = Mat3::from_components_row_major(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let m2 = Mat3::from_components_col_major(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m1, m2.transpose());
    }

    // --------------------------------------------------------------------------
    // from_diagional
    // --------------------------------------------------------------------------

    #[test]
    fn matrix3_from_diagonal() {
        let m1 = Mat3::from_diagonal(Vec3::new(1.0, 2.0, 3.0));
        let m2 = Mat3 { m00: 1.0, m01: 0.0, m02: 0.0, m10: 0.0, m11: 2.0, m12: 0.0, m20: 0.0, m21: 0.0, m22: 3.0 };
        assert_eq!(m1, m2);
    }

    // --------------------------------------------------------------------------
    // Mul
    // --------------------------------------------------------------------------

    #[test]
    fn matrix3_mul_val_val1() {
        let m1 = Mat3::identity();
        let m2 = Mat3::from_components_row_major(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m1*m2, m2);
    }

    #[test]
    fn matrix3_mul_val_val2() {
        let m1 = Mat3::from_components_row_major(
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0, 
            7.0, 8.0, 9.0
        );
        let m2 = Mat3::from_components_row_major(
            1.0, 2.0, 3.0, 
            4.0, 5.0, 6.0, 
            7.0, 8.0, 9.0
        );
        let m3 = Mat3::from_components_row_major(
             30.0,  36.0,  42.0,
             66.0,  81.0,  96.0,
            102.0, 126.0, 150.0,
        );
        assert_eq!(m1*m2, m3);
    }

    // --------------------------------------------------------------------------
    // det
    // --------------------------------------------------------------------------

    #[test]
    fn matrix3_det1() {
        let m = Mat3::from_components_row_major(
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0
        );
        assert_eq!(m.det(), 0.0);
    }

    #[test]
    fn matrix3_det2() {
        let m = Mat3::from_components_row_major(
            2.0,  5.0,  2.0,
            3.0, -3.0,  1.0,
            1.0,  4.0, -4.0
        );
        assert_eq!(m.det(), 111.0);
    }

    #[test]
    fn matrix3_det3() {
        let m = Mat3::from_components_row_major(
             5.0, -1.0,  9.0,
            -1.0,  6.0, -1.0,
             9.0, -1.0,  7.0
        );
        assert_eq!(m.det(), -270.0);
    }

    // --------------------------------------------------------------------------
    // ApproxEq
    // --------------------------------------------------------------------------

    #[test]
    fn matrix3_approx_eq() {
        let m1 = Mat3::from_components_row_major(
            2.0,  5.0,  2.0,
            3.0, -3.0,  1.0,
            1.0,  4.0, -4.0
        );
        let m2 = Mat3::from_components_row_major(
            2.0,  5.0,  2.000000001,
            3.0, -3.0,  1.0,
            1.0,  4.00000001, -4.0
        );
        let m3 = Mat3::from_components_row_major(
            3.0,  5.0,  2.0,
            3.0, -3.0,  1.0,
            1.0,  4.0, -6.0
        );
        assert!(m1.approx_eq(&m2, ::std::f32::EPSILON, 2));
        assert!(!m1.approx_eq(&m3, ::std::f32::EPSILON, 2));
    }

    // --------------------------------------------------------------------------
    // inverse
    // --------------------------------------------------------------------------

    #[test]
    fn matrix3_inverse1() {
        let m = Mat3::from_components_row_major(
            2.0,  5.0,  2.0,
            3.0, -3.0,  1.0,
            1.0,  4.0, -4.0
        );
        assert!((m*m.inverse().unwrap()).approx_eq(&Mat3::identity(), f32::EPSILON, 2)); // TODO(henk): What?
    }


    // --------------------------------------------------------------------------
    //
    // Matrix4
    //
    // --------------------------------------------------------------------------

    #[test]
    fn matrix4_inverse0() {
        assert!(Mat4::identity().inverse().unwrap().approx_eq(&Mat4::identity(), ::std::f32::EPSILON, 2));
    }

    #[test]
    fn matrix4_inverse1() {
        let m = Mat4::from_components_row_major(
            1.0, -2.0, 3.0, 4.0,
            -2.0, 3.0, 4.0, 5.0,
            3.0, 4.0, 5.0, 6.0,
            4.0, 5.0, 6.0, 7.0
        );
        let result = Mat4::from_components_row_major(
            0.0, -4.0/16.0, 8.0/16.0, -4.0/16.0,
            -4.0/16.0, 0.0, 12.0/16.0, -8.0/16.0,
            8.0/16.0, 12.0/16.0, -160.0/16.0, 124.0/16.0,
            -4.0/16.0, -8.0/16.0, 124.0/16.0, -96.0/16.0
        );
        assert!(m.inverse().unwrap().approx_eq(&result, f32::EPSILON, 2));
    }


    #[test]
    fn matrix4_inverse2() {
        let m = Mat4::from_components_row_major(
            1.0, -2.0, 3.0, 4.0,
            -2.0, 3.0, 4.0, 5.0,
            3.0, 4.0, 5.0, 6.0,
            4.0, 5.0, 6.0, 7.0
        );
        assert!((m*m.inverse().unwrap()).approx_eq(&Mat4::identity(), f32::EPSILON, 2));
    }

    #[test]
    fn matrix4_inverse3() {
        let m = Mat4::from_components_row_major(
            1.0, 0.0, 12.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 4.0, -136.0,
            0.0, -2.0, 0.0, 1.0
        );
        let result = Mat4::from_components_row_major(
            1.0, -3264.0/4.0, -12.0/4.0, -1632.0/4.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 272.0/4.0, 1.0/4.0, 136.0/4.0,
            0.0, 8.0/4.0, 0.0, 1.0
        );
        println!("m ?= result ... {:?} ?= {:?}", m.inverse().unwrap(), result);
        assert!(m.inverse().unwrap().approx_eq(&result, ::std::f32::EPSILON, 2));
    }

    #[test]
    fn matrix4_inverse4() {
        let v = Mat4::new_look_at(Vec3::new(0.5, 1.0, 0.5), Vec3::zero(), Vec3::new(0.0, 1.0, 0.0));
        let p = Mat4::new_orthographic_from_dimensions(100.0, 80.0, -1.0, 1.0);
        let x = Vec4::new(0.6, 0.1, 3.3, 1.0);
        println!("p*v = {:?}", p*v);
        let u = (p*v*x).wdiv();
        let r = (p*v).inverse().unwrap()*Vec4::new(u.x, u.y, u.z, 1.0);
        println!("{:?} -> {:?}", x, r);
        assert!(x.approx_eq(&r, 2.0*::std::f32::EPSILON, 3));
    }
}