use std::fs;

use raytracer_rust::{features::colors::Color, PPMCanvas, Point3, RawCanvas, Vector3};

fn main() {
    draw_projectile();
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
fn draw_projectile() {
    let start = Point3::new(0.0, 1.0, 0.0);
    let velocity = Vector3::new(1.0, 1.8, 0.0).normalized() * 11.0;
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
    println!("{}", canvas.height());
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
