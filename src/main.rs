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

fn draw_rectangle(image: &mut bmp::Image, x: u32, y: u32, w: u32, h: u32, c: bmp::Pixel) {
    for x_pixel in x..w {
        for y_pixel in y..h {
            image.set_pixel(x_pixel, y_pixel, c);
        }
    }
}

fn draw_finnish_flag(path: &str) {
    let mut image = bmp::Image::new(110, 180);

    draw_rectangle(&mut image, 0, 0, 110, 180, bmp::Pixel::new(0, 0, 255));

    image.save(path).expect("Image couldn't save");
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
        _ =>  {
            eprintln!("The operation {op} was not recognised!");
        },
    }
}
