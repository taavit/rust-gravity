extern crate image;

use image::{ImageBuffer};

const G:f64 = 6.67*10e-11;
const EARTH_MASS:f64 = 5.972 * 10e24;
const MOON_MASS:f64 = 7.348 * 10e22;
const EARTH_RADIUS:f64 = 6371.0 * 10e3;

const MOON_EARTH_DISTANCE: f64 = 384402.0 * 10e3;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

const EARTH_CENTER_X: u32 = WIDTH / 3;
const EARTH_CENTER_Y: u32 = HEIGHT / 2;

const M_PER_PIXEL: f64 = EARTH_RADIUS / 12.0;

const MOON_CENTER_X: u32 = ((MOON_EARTH_DISTANCE) / M_PER_PIXEL) as u32 + EARTH_CENTER_X;
const MOON_CENTER_Y: u32 = EARTH_CENTER_Y;

fn main() {
    let mut image : image::RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    let mut x = 0;
    let mut y = 0;
    let mut _distance: f64 = 0.0;
    let mut _gravity: f64 = 0.0;
    let mut _bright: u8 = 0;

    let mut _m_distance: f64 = 0.0;
    let mut _m_gravity: f64 = 0.0;
    let mut _m_bright: u8 = 0;

    let mut _moon_gravity: u8 = 0;

    println!("Moon center: {},{}\n", MOON_CENTER_X, MOON_CENTER_Y);
    println!("Earth center: {},{}\n", EARTH_CENTER_X, EARTH_CENTER_Y);

    while y < HEIGHT {
        while x < WIDTH {
            _distance = distance(x, y);
            _gravity = (G * EARTH_MASS) / _distance.powi(2);
            _bright = (clamp(0.0, 255.0, (_gravity * 100000.0) / 255.0)) as u8;

            _m_distance = moon_distance(x, y);
            _m_gravity = (G * MOON_MASS) / _m_distance.powi(2);
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
            x = x + 1;
        }
        y = y +1;
        x = 0;
    }
    image.save("output.png").unwrap();
}

fn distance(x: u32, y: u32) -> f64 {
    let _diff_x: i32 = (EARTH_CENTER_X as i32) - (x as i32);
    let _diff_y: i32 = (EARTH_CENTER_Y as i32) - (y as i32);
    ((_diff_y as f64 * M_PER_PIXEL).powi(2) + (_diff_x as f64 * M_PER_PIXEL).powi(2)).sqrt()
}

fn moon_distance(x: u32, y: u32) -> f64 {
    let _diff_x: i32 = (MOON_CENTER_X as i32) - (x as i32);
    let _diff_y: i32 = (MOON_CENTER_Y as i32) - (y as i32);
    ((_diff_y as f64 * M_PER_PIXEL).powi(2) + (_diff_x as f64 * M_PER_PIXEL).powi(2)).sqrt()
}

fn clamp(low: f64, high: f64, value: f64) -> f64{
    if value > high {
        return high;
    }
    if value < low {
        return low;
    }
    return value;
}