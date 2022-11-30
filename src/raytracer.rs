use std::fs::File;

use image::{DynamicImage, Rgba, Pixel, GenericImage, ImageFormat};

use crate::{scene::{Scene, Sphere}, ray::Ray};

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba([0, 0, 0, 0]);

    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            if scene.sphere.intersect(&ray) {
                image.put_pixel(x, y, scene.sphere.color);
            } else {
                image.put_pixel(x, y, black);
            }
        }
    }

    image
}

pub fn write_image(filename: &str, image: DynamicImage) -> Result<(), Box<dyn std::error::Error>> {
    let mut f = File::create(filename)?;
    image.write_to(&mut f, ImageFormat::Png)?;

    Ok(())
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        // A line segment between the ray origin and the center of the sphere
        let l = self.center - ray.origin; 
        // use l as a hypotenuse and find the length of the ajacent side
        let adj2 = l.dot(&ray.direction);
        // Find the length-squared of the opposite side
        // This is equivalent to (l.length() * l.length()) - (adj2 * adj2)
        let d2 = l.dot(&l) - (adj2 * adj2);
        // If the length-squared is less than the radius squared, the ray intersects the sphere
        d2 < (self.radius * self.radius)
    }
}