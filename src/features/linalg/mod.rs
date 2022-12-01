pub(crate) mod point;
mod tuple;
pub(crate) mod vector;

/// Tests in Chapter 1.
#[cfg(test)]
mod test {
    use crate::{Point, Point3, Vector, Vector3};
    struct Projectile {
        pos: Point3<f64>,
        velocity: Vector3<f64>,
    }
    struct Environment {
        gravity: Vector3<f64>,
        wind: Vector3<f64>,
    }

    fn tick(proj: Projectile, env: &Environment) -> Projectile {
        let new_pos = proj.pos + proj.velocity;
        let new_velocity = proj.velocity + env.gravity + env.wind;
        Projectile {
            pos: new_pos,
            velocity: new_velocity,
        }
    }
    #[test]
    fn virtual_canon() {
        let mut p = Projectile {
            pos: Point::new(0.0, 1.0, 0.0),
            velocity: Vector::new(1.0, 1.0, 0.0).normalized(),
        };
        let e = Environment {
            gravity: Vector::new(0.0, -0.1, 0.0),
            wind: Vector::new(-0.01, 0.0, 0.0),
        };
        let mut count = 1;
        while p.pos.y > 0.0 {
            p = tick(p, &e);
            println!("Position: {:?}", Into::<[f64; 3]>::into(p.pos));
            count += 1;
        }
        println!("Ended. Count = {}", count);
    }

    #[test]
    fn point_vector_creation() {
        let p = Point::new(4, -4, 3);
        let v = Vector::new(4, -4, 3);
        assert_eq!(p.w, 1);
        assert_eq!(v.w, 0);
    }
    #[test]
    fn add_tuples() {
        let a1 = Point::from([3, -2, 5]);
        let a2 = Vector::from([-2, 3, 1]);
        assert_eq!(a1 + a2, Point::<i32, 4>::from([1, 1, 6]));
    }
    #[test]
    fn subtracting_two_points() {
        let p1 = Point::new(3, 2, 1);
        let p2 = Point::new(5, 6, 7);
        assert_eq!(p1 - p2, Vector::new(-2, -4, -6))
    }
    #[test]
    fn subtracting_vec_from_point() {
        let p = Point::new(3, 2, 1);
        let v = Vector::new(5, 6, 7);
        assert_eq!(p - v, Point::from([-2, -4, -6]))
    }
    #[test]
    fn subtracting_two_vectors() {
        let v1: Vector3<i32> = Vector::from([3, 2, 1]);
        let v2: Vector3<i32> = Vector::from([5, 6, 7]);
        assert_eq!(v1 - v2, Vector::from([-2, -4, -6]));
        // Substraction of a point from vector is automatically
        // inhibited due to the type and trait checking of rust.
    }
    #[test]
    fn negate_vector() {
        let a = Vector::from([1, -2, 3, -4]);
        assert_eq!(-a, Vector::from([-1, 2, -3, 4]))
    }
    #[test]
    fn scalar_mul_div() {
        let v = Vector::from([1.0, -2.0, 3.0, -4.0]);
        assert_eq!(3.5 * v, Vector::from([3.5, -7.0, 10.5, -14.0]));
        assert_eq!(v / 2.0, Vector::from([0.5, -1.0, 1.5, -2.0]));
    }
    #[test]
    fn magnitude() {
        let v1: Vector<f64, 3> = Vector::from([1.0, 0.0, 0.0]);
        assert_eq!(1.0, v1.magnitude());
        let v2: Vector<f64, 3> = Vector::from([0.0, 1.0, 0.0]);
        assert_eq!(1.0, v2.magnitude());
        let v3: Vector<f64, 3> = Vector::from([1.0, 2.0, 3.0]);
        assert_eq!(14_f64.sqrt(), v3.magnitude());
        let v4: Vector<f64, 3> = Vector::from([-1.0, -2.0, -3.0]);
        assert_eq!(14_f64.sqrt(), v4.magnitude());
    }
    #[test]
    fn normalization() {
        let v1 = Vector::new(4.0, 0.0, 0.0);
        assert_eq!(v1.normalized(), Vector::new(1.0, 0.0, 0.0));
        let v2 = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(
            v2.normalized(),
            Vector::new(
                1.0 / 14_f64.sqrt(),
                2.0 / 14_f64.sqrt(),
                3.0 / 14_f64.sqrt()
            )
        );
        let v3 = Vector::new(1.0, 2.0, 3.0);
        let norm = v3.normalized();
        assert_eq!(1.0, norm.magnitude());
    }
    #[test]
    fn dot_product() {
        let v1 = Vector::new(1, 2, 3);
        let v2 = Vector::new(2, 3, 4);
        assert_eq!(v1.dot(&v2), 20);
    }
    #[test]
    fn cross_product() {
        let v1 = Vector::new(1, 2, 3);
        let v2 = Vector::new(2, 3, 4);
        assert_eq!(v1.cross(&v2), Vector::new(-1, 2, -1));
        assert_eq!(v2.cross(&v1), Vector::new(1, -2, 1));
        let x: Vector<i32, 4> = Vector::unit_x();
        let y: Vector<i32, 4> = Vector::unit_y();
        assert_eq!(x.cross(&y), Vector::<i32, 4>::unit_z());
    }
}
