extern crate rayon;

use mandelbrot::*;
use num::Complex;
use rayon::prelude::*;
use std::io::Write;
use std::str::FromStr;

fn parse_pair<T: FromStr>(s: &str, sep: char) -> Option<(T, T)> {
    match s.find(sep) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("1x2", 'x'), Some((1, 2)));
    assert_eq!(parse_pair::<i32>("2", 'x'), None);
    assert_eq!(parse_pair::<i32>(",", ','), None);
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("123,321"),
        Some(Complex {
            re: 123.0,
            im: 321.0
        })
    );
    assert_eq!(parse_complex("123,"), None);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        writeln!(
            std::io::stderr(),
            "Usage: mandelbrot FILE PIXELS UPPER_LEFT LOWER_RIGHT"
        )
        .unwrap();
        writeln!(
            std::io::stderr(),
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        )
        .unwrap();
        std::process::exit(1);
    }

    let bounds = parse_pair::<usize>(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];
    {
        let bands: Vec<(usize, &mut [u8])> = pixels.chunks_mut(bounds.0).enumerate().collect();
        bands.into_par_iter().weight_max().for_each(|(i, band)| {
            let top = i;
            let band_bounds = (bounds.0, 1);
            let band_upper_left = pix_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                pix_to_point(bounds, (bounds.0, top + 1), upper_left, lower_right);
            render(band, band_bounds, band_upper_left, band_lower_right);
        });
    }

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}
