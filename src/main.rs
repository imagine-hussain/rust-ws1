// use bmp;
use std::io;
use io::{stdout, Write};

struct Canvas {
    path: String,
    width: u32,
    height: u32,
    image: bmp::Image
}

fn draw_pixel(path: String) {
    let pathc = path.clone();
    let mut image = match open_canvas(path) {
        Ok(image) => image,
        Err(_) => new_canvas(pathc, 100, 100),
    };
    match image.draw_pixel(50, 50, None) {
        Ok(_) => (),
        Err(_) => println!("Error drawing pixel"),
    };
    match image.draw_pixel(0, 0, None) {
        Ok(_) => (),
        Err(_) => println!("Error drawing pixel"),
    };
    image.draw_pixel(0, 50, None).expect("Error drawing pixel");
    image.save(None).expect("This should save correctly.");
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
        "pixel\n" => draw_pixel(path),
        _ =>  {
            eprintln!("The operation {op} was not recognised!");
        },
    }

    // let op = stdin.read();

    // match op.as_str() {
    //     "pixel\n" => draw_pixel(path.as_str()),
    //     "outlined square\n" => {
    //         let width = get_width();
    //         draw_outlined_square(&path, width);
    //     },
    //     "filled square\n" => {
    //         let width = get_width();
    //         draw_filled_square(&path, width, bmp::Pixel::new(255, 255, 255));
    //     },
    //     "diagonal\n" => match read().as_str() {
    //         "right\n" => draw_diagonal_line(path.as_str(), Some(Corner::TopRight), None),
    //         "left\n" => draw_diagonal_line(path.as_str(), Some(Corner::TopLeft), None),
    //         _ => eprintln!("Expected left or right for diagonal"),
    //     },
    //     "cross\n" => todo!("cross"),
    //     "house\n" => todo!("house"),
    //     "outline\n" => {
    //         let coords = read();
    //         let parts: Vec<_> = coords.split_whitespace().collect();
    //         if parts.len() == 4 {
    //             todo!("square outline")
    //         } else {
    //             eprintln!("Expected \"t l w h\" as coordinates to square outline")
    //         }
    //     }
    //     "flag\n" => match read().as_str() {
    //         "rainbow\n" => todo!("rainbow"),
    //         "finland\n" => todo!("finland"),
    //         "iceland\n" => todo!("iceland"),
    //         "aboriginal\n" => todo!("aboriginal"),
    //         _ => eprintln!("Expected rainbow | finland | iceland | aboriginal for diagonal"),
    //     },
    //     "sine\n" => todo!("sine"),
    //     _ => eprintln!("The operation {op} was not recognised!"),
    // }
}

// Used for creating
fn new_canvas (path: String, width: u32, height: u32) -> Canvas {
    let image = bmp::Image::new(width, height);
    Canvas { path, width, height, image }
}

fn open_canvas(path: String) -> Result<Canvas, bmp::BmpError> {
    let image = match bmp::open(&path) {
        Ok(i) => i,
        Err(e) => return Err(e),
    };
    Ok(Canvas { path, width: image.get_width(), height: image.get_height(), image })
}

fn image_to_canvas(image: bmp::Image, path: String) -> Canvas {
    Canvas { path, width: image.get_width(), height: image.get_height(), image }
}

#[derive(Debug)]
enum CanvasError {
    BmpError(bmp::BmpError),
    IoError(io::Error),
    OutBoundsError
}

impl Canvas {
    fn draw_pixel(&mut self, x: u32, y: u32, color: Option<bmp::Pixel>) -> Result<(), CanvasError> {
        if self.out_bounds(x, y) {
            return Err(CanvasError::OutBoundsError);
        }
        let pixel = match color {
            Some(c) => c,
            None => bmp::Pixel::new(255, 255, 255)
        };
        self.image.set_pixel(x, y, pixel);
        Ok(())
    }

    fn out_bounds(&self, x: u32, y: u32) -> bool {
        x > self.width || y > self.height
    }

    fn save_image(&self) -> io::Result<()> {
        self.image.save(&self.path)
    }

    fn save(&self, path: Option<String>) -> Result<(), io::Error> {
        let path = match path {
            Some(p) => p,
            None => self.path.clone(),
        };
        match self.image.save(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
