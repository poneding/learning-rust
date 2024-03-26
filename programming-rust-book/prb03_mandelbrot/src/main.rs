extern crate crossbeam;
extern crate image;
extern crate num;

use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder};
use std::fs::File;
use std::str::FromStr;

use num::Complex;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPER_LEFT LOWER_RIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("Error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("Error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("Error parsing lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1]; // vec![v; n], v is the initial value, n is the length
                                                   // render(&mut pixels, bounds, upper_left, lower_right);
                                                   // use crossbeam to parallelize the rendering
    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;
    {
        let bands = pixels
            .chunks_mut(rows_per_band * bounds.0)
            .collect::<Vec<&mut [u8]>>();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        })
        .expect("Failed to spawn threads");
    }
    write_image(&args[1], &pixels, bounds);
}

#[allow(dead_code)] // suppress warnings
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) {
    let output = File::create(filename).expect("Failed to create file");
    let encoder = PngEncoder::new(output);
    encoder
        .write_image(
            &pixels,
            bounds.0 as u32,
            bounds.1 as u32,
            ExtendedColorType::L8,
        )
        .expect("Failed to encode image");
}

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

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);
    for r in 0..bounds.1 {
        for c in 0..bounds.0 {
            let point = pixel_to_point(bounds, (c, r), upper_left, lower_right);
            pixels[r * bounds.0 + c] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

#[test]
fn test_pixel_to_point() {
    assert!(
        pixel_to_point(
            (100, 100),
            (25, 75),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ) == Complex { re: -0.5, im: -0.5 }
    );
}

fn _build_vector() -> Vec<i16> {
    let mut v = Vec::new(); // recognize the type of v is Vec<i16>
    v.push(1);
    v.push(2);
    v
}

struct _S {
    x: i32,
    y: i32,
}

struct _T(i32, char);

struct _E; // empty struct

enum _Attend {
    OnTime,
    Late(u32),
}

fn _dmeo_type() {
    let a = _Attend::Late(10);
    match a {
        _Attend::OnTime => println!("On time"),
        _Attend::Late(minutes) => println!("Late by {} minutes", minutes),
    }

    let _b = _Attend::OnTime;

    let _a_tuple = (10, 'A'); // (i32, char)

    let _empty_tuple = (); // empty tuple

    let _late30 = Box::new(_Attend::Late(30)); // Box<_Attend>

    let _str1 = "Hello, world!"; // &str
    let _string1 = _str1.to_string(); // String

    let _str2 = &_string1[0..4]; // &str

    let _arr1: [i32; 5] = [1, 2, 3, 4, 5]; // [i32; 5]
    let mut _vec1 = vec![1, 2, 3, 4, 5]; // Vec<i32>

    let _slice1 = &_arr1[1..3]; // &[i32]
    let _slice2 = &mut _vec1[1..3]; // &mut [i32]

    let _max = std::i32::MAX;
    // let _maxplus1 = _max + 1; // overflow
    let _maxplus1 = _max.wrapping_add(1); // no panic, the result is std::i32::MIN
}

#[test]
fn _test_demo_type() {
    _dmeo_type();
}

#[test]
fn _test_demo_conv() {
    assert_eq!(10_i8 as u16, 10_u16);

    assert_eq!(1000_i16 as u8, 232_u8); // 1000 % 256 = 232
    assert_eq!(65535_u32 as i16, -1_i16);
}

#[test]
fn _test_demo_vec() {
    let _v0 = vec![1, 2, 3]; // Vec<i32>, also can new with Vec::new(), then push elements
    println!("_v0: {:?}", _v0);

    let mut _v1 = Vec::new(); // Vec<T>
    _v1.push(1);
    println!("_v1: {:?}", _v1);

    let mut _v2 = vec![1, 2, 3]; // Vec<i32>
    _v2.insert(1, 11);
    println!("_v2: {:?}", _v2);
    _v2.remove(2);
    println!("_v2: {:?}", _v2);

    let _v3 = vec![0; 10]; // Vec<i32> with 10 elements, each element is 0

    let _v4 = vec![0; 10].iter().map(|x| x + 1).collect::<Vec<i32>>();
    let _v5 = vec![1; 10].iter().fold(0, |acc, x| acc + x);
    println!("_v5: {:?}", _v5);

    let mut _v6 = Vec::with_capacity(3);
    _v6.push(1);

    println!("_v6 capacity: {:?}", _v6.capacity()); // 3
    println!("_v6 len: {:?}", _v6.len()); // 1

    let mut _v7 = vec!["a", "b"];
    assert_eq!(_v7.pop(), Some("b")); // pop return Options<T>: Some(v) or None
    assert_eq!(_v7.pop(), Some("a"));
    assert_eq!(_v7.pop(), None);

    let mut _v8 = vec!["a", "b", "c"];
    _v8 = _v8.iter().skip(1).map(|s| *s).collect();
    for s in _v8.iter() {
        println!("s in _v8: {}", s);
    }

    let _v81 = vec!["a", "b", "c"];
    assert_eq!(_v81.concat(), "abc"); // "abc"
    assert_eq!(_v81.join(","), "a,b,c"); // "a,b,c"

    let _v9: [f64; 3] = [1.0, 2.0, 3.0];
}

#[test]
fn _test_demo_string() {
    println!(
        "Hello,
Rust!"
    ); // newline

    println!(r"C:\Program Files\Microsoft"); // raw string

    println!(r###"Hello, "Rust"!"###); // raw string with multiple #

    // byte string
    let method = b"GET"; // &[u8; 3], slice of bytes
    assert_eq!(method, &[b'G', b'E', b'T']);

    let hello = "Hello"; // &str
    let hello_string = hello.to_string(); // String
    let ello = &hello_string[1..]; // &str
    println!("ello: {}", ello);

    let poodles = "🐩_🐩";
    println!("poodles: {}", poodles);
    println!("poodles.len(): {}", poodles.len()); // 9
    println!("poodles.chars().count(): {}", poodles.chars().count()); // 3

    let mut _s0 = String::new(); // String::new() -> String
    _s0.push('H');
    _s0.push_str("ello");
    assert_eq!(String::from("Hello"), _s0); // String::from(&str) -> String

    _s0.pop(); // remove the last character
    assert_eq!(_s0, "Hell");

    let _s1 = String::with_capacity(4); // String::with_capacity(usize) -> String

    let _format_string = format!("{}°{:02}'{:02}", 24, 5, 23); // format! macro -> String
    assert_eq!(_format_string, "24°05'23".to_string()); // format! macro
}
