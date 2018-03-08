#[cfg(test)]
mod tests {
    use vector::*;
    use traits::*;

    type Vec2  = Vector2<f32>;
    type Vec3  = Vector3<f32>;

    // --------------------------------------------------------------------------
    //
    // Vector2
    //
    // --------------------------------------------------------------------------

    // --------------------------------------------------------------------------
    // Add
    // --------------------------------------------------------------------------

    #[test]
    fn vector2_add_val_val() {
        assert_eq!(Vec2::new(1.0, 2.0) + Vec2::new(2.0, 3.0), Vec2::new(3.0, 5.0));
    }

    #[test]
    fn vector2_add_val_ref() {
        assert_eq!(Vec2::new(1.0, 2.0) + &Vec2::new(2.0, 3.0), Vec2::new(3.0, 5.0));
    }

    #[test]
    fn vector2_add_val_mutref() {
        assert_eq!(Vec2::new(1.0, 2.0) + &mut Vec2::new(2.0, 3.0), Vec2::new(3.0, 5.0));
    }

    #[test]
    fn vector2_add_ref_val() {
        assert_eq!(&Vec2::new(1.0, 2.0) + Vec2::new(2.0, 3.0), Vec2::new(3.0, 5.0));
    }

    #[test]
    fn vector2_add_ref_ref() {
        assert_eq!(&Vec2::new(1.0, 2.0) + &Vec2::new(2.0, 3.0), Vec2::new(3.0, 5.0));
    }

    #[test]
    fn vector2_add_ref_mutref() {
        assert_eq!(&Vec2::new(1.0, 2.0) + &mut Vec2::new(2.0, 3.0), Vec2::new(3.0, 5.0));
    }

    #[test]
    fn vector2_add_mutref_val() {
        assert_eq!(&mut Vec2::new(1.0, 2.0) + Vec2::new(2.0, 3.0), Vec2::new(3.0, 5.0));
    }

    #[test]
    fn vector2_add_mutref_ref() {
        assert_eq!(&mut Vec2::new(1.0, 2.0) + &Vec2::new(2.0, 3.0), Vec2::new(3.0, 5.0));
    }

    #[test]
    fn vector2_add_mutref_mutref() {
        assert_eq!(&mut Vec2::new(1.0, 2.0) + &mut Vec2::new(2.0, 3.0), Vec2::new(3.0, 5.0));
    }

    #[test]
    fn vector2_add_assign_val() {
        let mut v = Vec2::new(1.0, 2.0);
        v += Vec2::new(2.0, 3.0);
        assert_eq!(v, Vec2::new(3.0, 5.0));
    }

    #[test]
    fn vector2_add_assign_ref() {
        let mut v = Vec2::new(1.0, 2.0);
        v += &Vec2::new(2.0, 3.0);
        assert_eq!(v, Vec2::new(3.0, 5.0));
    }

    #[test]
    fn vector2_add_assign_mutref() {
        let mut v = Vec2::new(1.0, 2.0);
        v += &mut Vec2::new(2.0, 3.0);
        assert_eq!(v, Vec2::new(3.0, 5.0));
    }

    // --------------------------------------------------------------------------
    // Sub
    // --------------------------------------------------------------------------

    #[test]
    fn vector2_sub() {
        assert_eq!(Vec2::new(2.0, 5.0) - Vec2::new(1.0, 3.0), Vec2::new(1.0, 2.0));
    }

    #[test]
    fn vector2_sub_assign() {
        let mut v = Vec2::new(2.0, 5.0); 
        v -= Vec2::new(1.0, 3.0);
        assert_eq!(v, Vec2::new(1.0, 2.0));
    }

    // --------------------------------------------------------------------------
    // Mulvs
    // --------------------------------------------------------------------------

    #[test]
    fn vector2_mulvs_val_val() {
        assert_eq!(Vec2::new(1.0, 2.0) * 2.0, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn vector2_mulvs_val_ref() {
        assert_eq!(Vec2::new(1.0, 2.0) * &2.0, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn vector2_mulvs_val_mutref() {
        assert_eq!(Vec2::new(1.0, 2.0) * &mut 2.0, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn vector2_mulvs_ref_val() {
        assert_eq!(&Vec2::new(1.0, 2.0) * 2.0, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn vector2_mulvs_ref_ref() {
        assert_eq!(&Vec2::new(1.0, 2.0) * &2.0, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn vector2_mulvs_ref_mutref() {
        assert_eq!(&Vec2::new(1.0, 2.0) * &mut 2.0, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn vector2_mulvs_mutref_val() {
        assert_eq!(&mut Vec2::new(1.0, 2.0) * 2.0, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn vector2_mulvs_mutref_ref() {
        assert_eq!(&mut Vec2::new(1.0, 2.0) * &2.0, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn vector2_mulvs_mutref_mutref() {
        assert_eq!(&mut Vec2::new(1.0, 2.0) * &mut 2.0, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn vector2_mulvs_assign_val() {
        let mut v = Vec2::new(1.0, 2.0);
        v *= 2.0;
        assert_eq!(v, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn vector2_mulvs_assign_ref() {
        let mut v = Vec2::new(1.0, 2.0);
        v *= &2.0;
        assert_eq!(v, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn vector2_mulvs_assign_mutref() {
        let mut v = Vec2::new(1.0, 2.0);
        v *= &mut 2.0;
        assert_eq!(v, Vec2::new(2.0, 4.0));
    }

    // --------------------------------------------------------------------------
    // Mulvv
    // --------------------------------------------------------------------------

    #[test]
    fn vector2_mulvv_val_val() {
        assert_eq!(Vec2::new(1.0, 2.0) * Vec2::new(2.0, 4.0), Vec2::new(2.0, 8.0));
    }

    #[test]
    fn vector2_mulvv_ref_ref() {
        assert_eq!(&Vec2::new(1.0, 2.0) * &Vec2::new(2.0, 4.0), Vec2::new(2.0, 8.0));
    }

    #[test]
    fn vector2_mulvv_mutref_mutref() {
        assert_eq!(&mut Vec2::new(1.0, 2.0) * &mut Vec2::new(2.0, 4.0), Vec2::new(2.0, 8.0));
    }

    #[test]
    fn vector2_mulvv_assign_val() {
        let mut v = Vec2::new(1.0, 2.0);
        v *= Vec2::new(2.0, 4.0);
        assert_eq!(v, Vec2::new(2.0, 8.0));
    }

    #[test]
    fn vector2_mulvv_assign_ref() {
        let mut v = Vec2::new(1.0, 2.0);
        v *= &Vec2::new(2.0, 4.0);
        assert_eq!(v, Vec2::new(2.0, 8.0));
    }

    #[test]
    fn vector2_mulvv_assign_mutref() {
        let mut v = Vec2::new(1.0, 2.0);
        v *= &mut Vec2::new(2.0, 4.0);
        assert_eq!(v, Vec2::new(2.0, 8.0));
    }

    // --------------------------------------------------------------------------
    // Divvs
    // --------------------------------------------------------------------------

    #[test]
    fn vector2_divvs_val_val() {
        assert_eq!(Vec2::new(4.0, 6.0) / 2.0, Vec2::new(2.0, 3.0));
    }

    #[test]
    fn vector2_divvs_assign_val() {
        let mut v = Vec2::new(4.0, 6.0);
        v /= 2.0;
        assert_eq!(v, Vec2::new(2.0, 3.0));
    }

    // --------------------------------------------------------------------------
    // Divvv
    // --------------------------------------------------------------------------

    #[test]
    fn vector2_divvv_val_val() {
        assert_eq!(Vec2::new(4.0, 6.0) / Vec2::new(2.0, 3.0), Vec2::new(2.0, 2.0));
    }

    #[test]
    fn vector2_divvv_assign_val_val() {
        let mut v = Vec2::new(4.0, 6.0);
        v /= Vec2::new(2.0, 3.0);
        assert_eq!(v, Vec2::new(2.0, 2.0));
    }

    // --------------------------------------------------------------------------
    // Single Traits
    // --------------------------------------------------------------------------   

    #[test]
    fn vector2_zero() {
        assert_eq!(Vec2::zero(), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn vector2_one() {
        assert_eq!(Vec2::one(), Vec2::new(1.0, 1.0));
    }

    #[test]
    fn vector2_neg_val() {
        assert_eq!(-Vec2::new( 1.0,  2.0), Vec2::new(-1.0, -2.0));
    }

    #[test]
    fn vector2_neg_ref() {
        assert_eq!(-(&Vec2::new( 1.0,  2.0)), Vec2::new(-1.0, -2.0));
    }

    #[test]
    fn vector2_neg_mutref() {
        assert_eq!(-(&mut Vec2::new( 1.0,  2.0)), Vec2::new(-1.0, -2.0));
    }

    // --------------------------------------------------------------------------
    // Magnitude
    // --------------------------------------------------------------------------   

    #[test]
    fn vector2_length1() {
        assert_eq!(Vec2::new(1.0, 0.0).length(), 1.0);
    }

    #[test]
    fn vector2_length2() {
        assert_eq!(Vec2::new(3.0, 4.0).length(), 5.0);
    }

    #[test]
    fn vector2_normalize1() {
        assert_eq!(Vec2::new(13.642102, 0.0).normalize(), Vec2::new(1.0, 0.0));
    }

    #[test]
    fn vector2_normalize2() {
        assert_eq!(Vec2::new(9.197, 9.197).normalize(), Vec2::new(1.546, 1.546).normalize());
    }

    // --------------------------------------------------------------------------
    //
    // Vector3
    //
    // --------------------------------------------------------------------------

    #[test]
    fn vector3_add() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) + Vec3::new(2.0, 3.0, 4.0), Vec3::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn vector3_sub() {
        assert_eq!(Vec3::new(2.0, 5.0, 7.0) - Vec3::new(1.0, 3.0, 4.0), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn vector3_mul() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) * 2.0, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn vector3_zero() {
        assert_eq!(Vec3::zero(), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn vector3_one() {
        assert_eq!(Vec3::one(), Vec3::new(1.0, 1.0, 1.0));
    }

    // --------------------------------------------------------------------------
    // Non-generically defined operators
    // --------------------------------------------------------------------------

    #[test]
    fn vector3_mulsv_val_val() {
        assert_eq!(2.0*Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn vector3_mulsv_val_ref() {
        assert_eq!(2.0*(&Vec3::new(1.0, 2.0, 3.0)), Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn vector3_mulsv_val_mutref() {
        assert_eq!(2.0*(&mut Vec3::new(1.0, 2.0, 3.0)), Vec3::new(2.0, 4.0, 6.0));
    }
    
    #[test]
    fn vector3_mulsv_ref_val() {
        assert_eq!((&2.0)*Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn vector3_mulsv_ref_ref() {
        assert_eq!((&2.0)*(&Vec3::new(1.0, 2.0, 3.0)), Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn vector3_mulsv_ref_mutref() {
        assert_eq!((&2.0)*(&mut Vec3::new(1.0, 2.0, 3.0)), Vec3::new(2.0, 4.0, 6.0));
    }
    
    #[test]
    fn vector3_mulsv_mutref_val() {
        assert_eq!((&mut 2.0)*Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn vector3_mulsv_mutref_ref() {
        assert_eq!((&mut 2.0)*(&Vec3::new(1.0, 2.0, 3.0)), Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn vector3_mulsv_mutref_mutref() {
        assert_eq!((&mut 2.0)*(&mut Vec3::new(1.0, 2.0, 3.0)), Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn vector3_divsv_val_val() {
        assert_eq!(6.0/Vec3::new(1.0, 2.0, 3.0), Vec3::new(6.0, 3.0, 2.0));
    }
    

    // --------------------------------------------------------------------------
    // Cross
    // --------------------------------------------------------------------------

    #[test]
    fn vector3_cross_val_val() {
        assert_eq!(Vec3::new(1.0, 0.0, 0.0).cross(Vec3::new(0.0, 1.0, 0.0)), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn vector3_cross_val_ref() {
        assert_eq!(Vec3::new(1.0, 0.0, 0.0).cross(&Vec3::new(0.0, 1.0, 0.0)), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn vector3_cross_val_mutref() {
        assert_eq!(Vec3::new(1.0, 0.0, 0.0).cross(&mut Vec3::new(0.0, 1.0, 0.0)), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn vector3_cross_ref_val() {
        assert_eq!((&Vec3::new(1.0, 0.0, 0.0)).cross(Vec3::new(0.0, 1.0, 0.0)), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn vector3_cross_ref_ref() {
        assert_eq!((&Vec3::new(1.0, 0.0, 0.0)).cross(&Vec3::new(0.0, 1.0, 0.0)), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn vector3_cross_ref_mutref() {
        assert_eq!((&Vec3::new(1.0, 0.0, 0.0)).cross(&mut Vec3::new(0.0, 1.0, 0.0)), Vec3::new(0.0, 0.0, 1.0));
    }
    
    #[test]
    fn vector3_cross_mutref_val() {
        assert_eq!((&mut Vec3::new(1.0, 0.0, 0.0)).cross(Vec3::new(0.0, 1.0, 0.0)), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn vector3_cross_mutref_ref() {
        assert_eq!((&mut Vec3::new(1.0, 0.0, 0.0)).cross(&Vec3::new(0.0, 1.0, 0.0)), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn vector3_cross_mutref_mutref() {
        assert_eq!((&mut Vec3::new(1.0, 0.0, 0.0)).cross(&mut Vec3::new(0.0, 1.0, 0.0)), Vec3::new(0.0, 0.0, 1.0));
    }

    // --------------------------------------------------------------------------
    // cw_min
    // --------------------------------------------------------------------------

    #[test]
    fn vector3_cw_min_val_val() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).cw_min(Vec3::new(3.0, 2.0, 1.0)), Vec3::new(1.0, 2.0, 1.0));
    }

    #[test]
    fn vector3_cw_min_val_ref() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).cw_min(&Vec3::new(3.0, 2.0, 1.0)), Vec3::new(1.0, 2.0, 1.0));
    }

    #[test]
    fn vector3_cw_min_val_mutref() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).cw_min(&mut Vec3::new(3.0, 2.0, 1.0)), Vec3::new(1.0, 2.0, 1.0));
    }

    #[test]
    fn vector3_cw_min_ref_val() {
        assert_eq!((&Vec3::new(1.0, 2.0, 3.0)).cw_min(Vec3::new(3.0, 2.0, 1.0)), Vec3::new(1.0, 2.0, 1.0));
    }

    #[test]
    fn vector3_cw_min_ref_ref() {
        assert_eq!((&Vec3::new(1.0, 2.0, 3.0)).cw_min(&Vec3::new(3.0, 2.0, 1.0)), Vec3::new(1.0, 2.0, 1.0));
    }

    #[test]
    fn vector3_cw_min_ref_mutref() {
        assert_eq!((&Vec3::new(1.0, 2.0, 3.0)).cw_min(&mut Vec3::new(3.0, 2.0, 1.0)), Vec3::new(1.0, 2.0, 1.0));
    }

    #[test]
    fn vector3_cw_min_mutref_val() {
        assert_eq!((&mut Vec3::new(1.0, 2.0, 3.0)).cw_min(Vec3::new(3.0, 2.0, 1.0)), Vec3::new(1.0, 2.0, 1.0));
    }

    #[test]
    fn vector3_cw_min_mutref_ref() {
        assert_eq!((&mut Vec3::new(1.0, 2.0, 3.0)).cw_min(&Vec3::new(3.0, 2.0, 1.0)), Vec3::new(1.0, 2.0, 1.0));
    }

    #[test]
    fn vector3_cw_min_mutref_mutref() {
        assert_eq!((&mut Vec3::new(1.0, 2.0, 3.0)).cw_min(&mut Vec3::new(3.0, 2.0, 1.0)), Vec3::new(1.0, 2.0, 1.0));
    }

    // --------------------------------------------------------------------------
    // cw_max
    // --------------------------------------------------------------------------

    #[test]
    fn vector3_cw_max_val_val() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).cw_max(Vec3::new(3.0, 2.0, 1.0)), Vec3::new(3.0, 2.0, 3.0));
    }

    // --------------------------------------------------------------------------
    // cw_abs
    // --------------------------------------------------------------------------

    #[test]
    fn vector3_cw_abs_val_val() {
        assert_eq!(Vec3::new(-1.5, 2.2, -3.6).cw_abs(), Vec3::new(1.5, 2.2, 3.6));
    }

    // --------------------------------------------------------------------------
    // dot
    // --------------------------------------------------------------------------

    #[test]
    fn vector3_dot_val_val() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).dot(Vec3::new(3.0, 2.0, 1.0)), 10.0);
    }

    #[test]
    fn vector3_dot_ref_ref() {
        assert_eq!((&Vec3::new(1.0, 2.0, 3.0)).dot(&Vec3::new(3.0, 2.0, 1.0)), 10.0);
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