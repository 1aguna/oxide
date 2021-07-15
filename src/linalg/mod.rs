pub mod vector;
pub mod point;

// TODO: Move to a better place and seperate "integration tests"
#[cfg(test)]
mod test {
    use crate::linalg::{
        point::Point,
        vector::Vector,
    };

    fn  test_vector_len_sqr() {
        let v = Vector::new(1f32, 2f32, 3f32);
        assert!(v.length_sqr() == 1f32 + 4f32 + 9f32);
    }

    fn test_vector_len() {
        let v = Vector::new(1f32, 0f32, 0f32);
        assert!(v.length() - 0f32 < 0.00001);

        let v = Vector::new(-1f32, -2f32, -3f32);
        assert!(v.length() - (14 as f32).sqrt() < 0.0001);

    }

    fn test_vector_index() {
        let mut v = Vector::new(1f32, 2f32, 3f32);

        assert!(v[0] == 1f32 && v[1] == 2f32 && v[2] == 3f32);
        let x = &mut v[1];
        *x = 5f32;

        assert!(v[1] == 5f32);
    }

    fn test_vector_subtraction() {
        let v1 = Vector::new(3f32, 2f32, 1f32);
        let v2 = Vector::new(5f32, 6f32, 7f32);
        let ans = Vector::new(-2f32, -4f32, -6f32);

        assert!(v1 - v2 == ans);
    }

    fn test_vector_negate() {
        let v1 = Vector::new(3f32, 2f32, 1f32);
        let ans = Vector::new(-3f32, -2f32, -1f32);

        assert!(-v1 == ans);
    }

    fn test_vector_mult_scalar() {
        let v1 = Vector::new(3f32, 2f32, 1f32);
        let ans = Vector::new(6f32, 4f32, 2f32);

        assert!(v1 * 2f32 == ans);
        assert!(2f32 * v1 == ans);
    }

    fn test_point_distance_sqr() {
        let a = Point::new(0f32, 0f32, 0f32);
        let b = Point::new(3f32, 4f32, 0f32);
        assert!(b.distance_sqr(&a) == 25f32);
    }

    fn test_point_subtract() {
        let mut v = Vector::new(-2f32, -4f32, -6f32);

        let p1 = Point::new(3f32, 2f32, 1f32);
        let p2 = Point::new(5f32, 6f32, 7f32);

        assert!(p1-p2 == v);
    }
}