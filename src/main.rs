use bmp;
use std::io::{stdout, Write};

fn draw_pixel(path: &str) {
    let mut image = match bmp::open(path) {
        Ok(i) => i,
        Err(_) => bmp::Image::new(100, 100)
    };

    image.set_pixel(50, 50, bmp::Pixel::new(255, 255, 255));

    image.save(path).expect("This should save correctly.");
}

fn draw_pixel_at_xy(image: &mut bmp::Image, x: u32, y: u32) {
    image.set_pixel(x, y, bmp::Pixel::new(255, 255, 255));
}

fn draw_outlined_square(path: &str, width: u32) {
    let mut image = match bmp::open(path) {
        Ok(i) => i,
        Err(_) => bmp::Image::new(width, width)
    };

    fill_image(&mut image, bmp::Pixel::new(0, 0, 0));

    (1..width).for_each(|i| {
        draw_pixel_at_xy(&mut image, i, 0);
        draw_pixel_at_xy(&mut image, i, width - 1);
        draw_pixel_at_xy(&mut image, 0, i);
        draw_pixel_at_xy(&mut image, width - 1, i);
    });

    image.save(path).expect("This should save correctly.");
}

fn fill_image(image: &mut bmp::Image, c: bmp::Pixel) {
    (1..image.get_width()).for_each(|i| {
        (1..image.get_height()).for_each(|j| {
            image.set_pixel(i, j, c);
        });
    });
}

fn draw_filled_square(path: &str, width: u32, c: bmp::Pixel) {
    let mut image = match bmp::open(path) {
        Ok(i) => i,
        Err(_) => bmp::Image::new(width, width)
    };

    fill_image(&mut image, c);
    image.save(path).expect("This should save correctly.");
}

fn get_width() -> u32 {
    print!("Width? ");
    stdout().flush().unwrap();
    let mut width_str = String::new();
    std::io::stdin().read_line(&mut width_str).unwrap();
    let width = width_str.strip_suffix('\n').unwrap().parse::<u32>().unwrap();
    width
}

fn main() {
    let path = std::env::args().nth(1).expect("You must provide a path.");

    print!("Which operation? ");
    // We use "flush" so that we see the question before the answer.
    // We can only use `flush` when we use `Write` too -- don't worry why yet!
    stdout().flush().unwrap();
    let mut op = String::new();
    std::io::stdin().read_line(&mut op).unwrap();

    match op.as_str() {
        "pixel\n" => draw_pixel(path.as_str()),
        "outlined square\n" => {
            let width = get_width();
            draw_outlined_square(&path, width);
        },
        "filled square\n" => {
            let width = get_width();
            draw_filled_square(&path, width, bmp::Pixel::new(255, 255, 255));
        },
        _ =>  {
            eprintln!("The operation {op} was not recognised!");
        },
    }
}
