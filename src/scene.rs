use image::Rgba;
use nalgebra::Vector3;

pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub color: Rgba<u8>,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphere,
}