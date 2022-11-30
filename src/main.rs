use image::Rgba;
use nalgebra::Vector3;
use raytracer::{scene::{Sphere, Scene}, raytracer::{render, write_image}};

fn main() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
            center: Vector3::new(0.0, 0.0, -5.0),
            radius: 1.0,
            color: Rgba::from([0, 44, 100, 40]),
        },
    };

    write_image("output.png", render(&scene)).unwrap();
}
