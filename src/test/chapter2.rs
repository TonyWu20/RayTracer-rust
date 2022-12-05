#[cfg(test)]
mod color_test {
    use approx::assert_relative_eq;

    use crate::features::colors::Color;
    #[test]
    fn mul_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_relative_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }
}
#[cfg(test)]
mod canvas_test {
    use crate::{
        features::{
            canvas::{
                ppm_canvas::{PPMCanvas, PPMColor},
                Canvas, CanvasIndexError,
            },
            colors::Color,
        },
        Point3, Vector3,
    };
    use approx::assert_relative_eq;
    use std::fs;

    #[test]
    fn create_canvas() {
        let canvas: Canvas<900, 550> = Canvas::default();
        for row in canvas.pixels() {
            for p in row {
                assert_relative_eq!(p, Color::<f64>::default())
            }
        }
    }
    #[test]
    fn writing_pixel() {
        let mut canvas: Canvas<10, 20> = Canvas::default();
        let red = Color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(2, 3, Color::new(1.0, 0.0, 0.0)).unwrap();
        assert_relative_eq!(red, canvas.pixel_at(2, 3).unwrap());
        let write_to_out_bound = canvas.write_pixel(10, 5, red);
        assert_eq!(
            write_to_out_bound.unwrap_err(),
            CanvasIndexError::new(10, 5, 10, 20)
        );
    }

    #[test]
    fn to_ppm_canvas() {
        let mut canvas: Canvas<10, 2> = Canvas::default();
        for x in 0..10 {
            for y in 0..2 {
                canvas.write_pixel(x, y, Color::new(1.0, 0.8, 0.6)).unwrap();
            }
        }
        let ppm_canvas: PPMCanvas<10, 2> = canvas.into();
        for row in ppm_canvas.pixels() {
            for p in row {
                assert_eq!(p, PPMColor(Color::new(255_u8, 204_u8, 153_u8)))
            }
        }
    }
    #[test]
    fn split_long_lines() {
        let mut canvas: Canvas<10, 2> = Canvas::default();
        for x in 0..10 {
            for y in 0..2 {
                canvas.write_pixel(x, y, Color::new(1.0, 0.8, 0.6)).unwrap();
            }
        }
        let ppm_canvas: PPMCanvas<10, 2> = canvas.into();
        println!("{}", ppm_canvas);
        fs::write("test_ppm.ppm", format!("{}", ppm_canvas)).unwrap();
    }
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
    fn draw_projectile() {
        let start = Point3::new(0.0, 1.0, 0.0);
        let velocity = Vector3::new(1.0, 1.8, 0.0).normalized() * 11.25;
        let mut p = Projectile {
            pos: start,
            velocity,
        };
        let gravity = Vector3::new(0.0, -0.1, 0.0);
        let wind = Vector3::new(-0.01, 0.0, 0.0);
        let e = Environment { gravity, wind };
        let mut canvas: Canvas<400, 320> = Canvas::default();
        let p_color = Color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(0, 550 - 1, p_color).unwrap();
        while p.pos.y > 0.0 && p.pos.x < 400.0 && p.pos.y < 320.0 {
            p = tick(p, &e);
            let (cp_x, cp_y) = (p.pos.x as usize, canvas.height() - p.pos.y as usize);
            canvas.write_pixel(cp_x, cp_y, p_color).unwrap();
        }
        let ppm_canvas: PPMCanvas<400, 320> = canvas.into();
        fs::write("chapter2_proj_draw.ppm", format!("{}", ppm_canvas)).unwrap();
    }
}
