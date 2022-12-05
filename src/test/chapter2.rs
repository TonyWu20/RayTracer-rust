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
                CanvasIndexError, RawCanvas,
            },
            colors::Color,
        },
        Point3, Vector3,
    };
    use approx::assert_relative_eq;
    use std::fs;

    #[test]
    fn create_canvas() {
        let canvas: RawCanvas<90, 55, f64> = RawCanvas::default();
        println!("{}", canvas.height());
        for &p in canvas.pixels() {
            assert_relative_eq!(p, Color::<f64>::default())
        }
    }
    #[test]
    fn writing_pixel() {
        let mut canvas: RawCanvas<10, 20, f64> = RawCanvas::default();
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
        let mut canvas: RawCanvas<10, 2, f64> = RawCanvas::default();
        for x in 0..10 {
            for y in 0..2 {
                canvas.write_pixel(x, y, Color::new(1.0, 0.8, 0.6)).unwrap();
            }
        }
        let ppm_canvas: PPMCanvas<10, 2> = canvas.into();
        for &p in ppm_canvas.pixels() {
            assert_eq!(p, PPMColor::new(255_u8, 204_u8, 153_u8))
        }
    }
    #[test]
    fn split_long_lines() {
        let mut canvas: RawCanvas<10, 2, f64> = RawCanvas::default();
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
        const WIDTH: usize = 900;
        const HEIGHT: usize = 550;
        let mut canvas: RawCanvas<WIDTH, HEIGHT, f64> = RawCanvas::default();
        let p_color = Color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(0, canvas.height() - 1, p_color).unwrap();
        while p.pos.y > 0.0 {
            p = tick(p, &e);
            if (p.pos.x as usize) < WIDTH && (p.pos.y as usize) < HEIGHT {
                let cp_x = p.pos.x as usize;
                let cp_y = (canvas.height() - 1) as f64 - p.pos.y;
                if cp_y > 0.0 && (cp_y as usize) < canvas.height() {
                    canvas.write_pixel(cp_x, cp_y as usize, p_color).unwrap();
                }
            }
        }
        let ppm_canvas: PPMCanvas<WIDTH, HEIGHT> = canvas.into();
        fs::write("chapter2_proj_draw.ppm", format!("{}", ppm_canvas)).unwrap();
    }
}
