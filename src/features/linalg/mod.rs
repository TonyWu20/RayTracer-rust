pub(crate) mod point;
mod tuple;
pub(crate) mod vector;

#[cfg(test)]
mod test {
    use crate::{Point, Vector};

    #[test]
    fn point_vector_creation() {
        let p = Point::new(4, -4, 3);
        let v = Vector::new(4, -4, 3);
        assert_eq!(p.w, 1);
        assert_eq!(v.w, 0);
    }
    #[test]
    fn add_tuples() {
        let a1 = Vector::from([3, -2, 5, 1]);
        let a2 = Vector::from([-2, 3, 1, 0]);
        assert_eq!(a1 + a2, Vector::<i32, 4>::from([1, 1, 6, 1]));
    }
}
