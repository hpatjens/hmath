#[cfg(test)]
mod tests {
    use vector::*;

    type Vec2  = Vector2<f32>;
    type Vec3  = Vector3<f32>;

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