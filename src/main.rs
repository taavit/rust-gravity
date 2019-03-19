extern crate image;

use image::{ImageBuffer};

const G: f64 = 6.67*10e-11;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

struct Body {
    mass: f64,   // Mass in kg
    radius: f64, // radius in meters
    x: u32,      // position x on plane
    y: u32,      // position y on plane
}

fn main() {
    let mut image : image::RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    let mut _distance: f64 = 0.0;
    let mut _gravity: f64 = 0.0;
    let mut _bright: u8 = 0;

    let mut _m_distance: f64 = 0.0;
    let mut _m_gravity: f64 = 0.0;
    let mut _m_bright: u8 = 0;

    let mut _moon_gravity: u8 = 0;

    let earth = Body {
        mass: 5.972 * 10e24,
        radius: 6371.0 * 10e3,
        x: WIDTH / 3,
        y: HEIGHT / 2,
    };

    let _moon_earth_distance: f64 = 384402.0 * 10e3;
    let _meters_per_pixel: f64 = earth.radius / 12.0;

    let moon = Body {
        mass: 7.348 * 10e22,
        radius: 1737.5 * 10e3,
        x: ((_moon_earth_distance) / _meters_per_pixel) as u32 + earth.x,
        y: earth.y
    };

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            _distance = distance_to_body(x, y, _meters_per_pixel, &earth);
            _gravity = (G * earth.mass) / _distance.powi(2);
            _bright = (clamp(0.0, 255.0, (_gravity * 100000.0) / 255.0)) as u8;

            _m_distance = distance_to_body(x, y, _meters_per_pixel, &moon);
            _m_gravity = (G * moon.mass) / _m_distance.powi(2);
            _m_bright = (clamp(0.0, 255.0, (_m_gravity * 100000.0) / 255.0)) as u8;

            if _m_gravity > _gravity {
                _moon_gravity = 255;
            } else {
                _moon_gravity = 0;
            }

            image.get_pixel_mut(x, y).data = [
                _bright,
                _m_bright,
                _moon_gravity,
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

fn distance_to_body(x: u32, y: u32, meters_per_pixel: f64, body: &Body) -> f64 {
    let _diff_x: i32 = (body.x as i32) - (x as i32);
    let _diff_y: i32 = (body.y as i32) - (y as i32);

    ((_diff_y as f64 * meters_per_pixel).powi(2) + (_diff_x as f64 * meters_per_pixel).powi(2)).sqrt()
}
