extern crate image;
extern crate num;

use image::png::PNGEncoder;
use image::ColorType;
use num::Complex;
use std::fs::File;

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

pub fn pix_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (w, h) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * w / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * h / bounds.1 as f64,
    }
}

#[test]
fn test_pix_to_point() {
    assert_eq!(
        pix_to_point(
            (100, 100),
            (25, 75),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: -0.5 }
    )
}

pub fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);
    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pix_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 + col] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            }
        }
    }
}

pub fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;
    Ok(())
}
