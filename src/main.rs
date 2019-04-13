extern crate image;

use image::{ImageBuffer};

use std::hash::{Hash, Hasher};
use std::collections::HashMap;

const G: f64 = 6.67*10e-11;

struct Body {
    name: String,
    mass: f64,   // Mass in kg
    radius: f64, // radius in meters
}

impl Hash for Body {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Body {
    fn eq(&self, other: &Body) -> bool {
        self.name == other.name
    }
}
impl Eq for Body {}

#[derive(Hash, Eq, PartialEq)]
struct Position {
    x: u128,      // position x on plane
    y: u128,      // position y on plane
}

struct Plane {
    bodies: HashMap<Body, Position>,
    width: usize,
    height: usize,
}

fn main() {
    let mut _moon_gravity: u8 = 0;

    let mut plane = Plane {
        width: 1920,
        height: 1080,
        bodies: HashMap::new(),
    };

    let earth = Body {
        name: String::from("earth"),
        mass: 5.972 * 10e24,
        radius: 6371.0 * 10e3,
    };

    let _moon_earth_distance: f64 = 384402.0 * 10e3;
    let _meters_per_pixel: f64 = earth.radius / 12.0;

    plane.bodies.insert(
        earth,
        Position {x: plane.width as u128 / 3, y: plane.height as u128 / 2}
    );

    let moon = Body {
        name: String::from("moon"),
        mass: 7.348 * 10e22,
        radius: 1737.5 * 10e3,
    };

    plane.bodies.insert(
        moon, Position {x: plane.width as u128 * 2/3, y: plane.height as u128 / 2}
    );

    let mut image : image::RgbImage = ImageBuffer::new(plane.width as u32, plane.height as u32);

    for y in 0..plane.height {
        for x in 0..plane.width {
            let mut _distance: f64 = 0.0;
            let mut _gravity: f64 = 0.0;
            let mut _bright: u8 = 0;
            for (key, val) in plane.bodies.iter() {
                _distance = distance_to_body(x as u32, y as u32, _meters_per_pixel, &val);
                _gravity = (G * key.mass) / _distance.powi(2);
                _bright = (clamp(0.0, 255.0, _bright as f64 + (_gravity * 10000000.0) / 255.0)) as u8;
            }

            image.get_pixel_mut(x as u32, y as u32).data = [
                _bright,
                _bright,
                _bright,
            ];
        }
    }
    image.save("output.png").unwrap();
}

fn clamp(low: f64, high: f64, value: f64) -> f64 {
    if value > high {
        return high;
    }
    if value < low {
        return low;
    }
    return value;
}

fn distance_to_body(x: u32, y: u32, meters_per_pixel: f64, position: &Position) -> f64 {
    let _diff_x: i32 = (position.x as i32) - (x as i32);
    let _diff_y: i32 = (position.y as i32) - (y as i32);

    ((_diff_y as f64 * meters_per_pixel).powi(2) + (_diff_x as f64 * meters_per_pixel).powi(2)).sqrt()
}
