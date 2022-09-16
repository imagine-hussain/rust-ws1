// use bmp;
use io::{stdout, Write};
use std::io;

#[derive(Debug)]
struct Canvas {
    path: String,
    width: u32,
    height: u32,
    image: bmp::Image,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

fn draw_pixel(path: String) {
    let pathc = path.clone();
    let mut image = match open_canvas(path) {
        Ok(image) => image,
        Err(_) => new_canvas(pathc, 100, 100),
    };

    let p1 = Point { x: 0, y: 0 };
    let p2 = Point { x: 20, y: 20 };
    let p3 = Point { x: 30, y: 30 };

    match image.draw_pixel(p1, None) {
        Ok(_) => (),
        Err(_) => println!("Error drawing pixel"),
    };
    match image.draw_pixel(p2, None) {
        Ok(_) => (),
        Err(_) => println!("Error drawing pixel"),
    };
    image.draw_pixel(p3, None).expect("Error drawing pixel");
    image.save(None).expect("This should save correctly.");
}

fn draw_diagonal_image(path: String) {
    let pathc = path.clone();
    let mut image = match open_canvas(path) {
        Ok(image) => image,
        Err(_) => new_canvas(pathc, 100, 100),
    };
    image.draw_line(&Point { x: 0, y: 0 }, &Point { x: image.width - 1, y: image.height - 1}, None)
        .expect("Error drawing diagonal line");
    image.draw_line(&Point { x: 0, y: image.height - 1}, &Point { x: image.width - 1, y: 0 }, None)
        .expect("Error drawing diagonal line");
    image.save(None).expect("This should save correctly.");
}

fn draw_outlined_square(path: String) {
    let pathc = path.clone();
    let mut image = match open_canvas(path) {
        Ok(image) => image,
        Err(_) => new_canvas(pathc, 100, 100),
    };
    image.draw_outlined_square(Point { x: 10, y: 10 }, 20, None)
        .expect("Error drawing square");
    image.save(None).expect("This should save correctly.");
}

fn draw_filled_rectangle(path: String) {
    let pathc = path.clone();
    let mut image = match open_canvas(path) {
        Ok(image) => image,
        Err(_) => new_canvas(pathc, 100, 100),
    };
    let start = Point { x: 10, y: 10 };
    let outline = bmp::Pixel { r: 255, g: 0, b: 0 };
    let fill = bmp::Pixel { r: 0, g: 0, b: 100 };
    image.draw_filled_rectangle(start, 10, 20, Some(outline), Some(fill))
        .expect("This should draw a rectangle");
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
        "diagonals\n" => draw_diagonal_image(path),
        "outlined_square\n" => draw_outlined_square(path),
        "fill_rectangle\n" => draw_filled_rectangle(path),
        _ => {
            eprintln!("The operation {op} was not recognised!");
        }
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
fn new_canvas(path: String, width: u32, height: u32) -> Canvas {
    let image = bmp::Image::new(width, height);
    Canvas {
        path,
        width,
        height,
        image,
    }
}

fn open_canvas(path: String) -> Result<Canvas, bmp::BmpError> {
    let image = match bmp::open(&path) {
        Ok(i) => i,
        Err(e) => return Err(e),
    };
    Ok(Canvas {
        path,
        width: image.get_width(),
        height: image.get_height(),
        image,
    })
}

fn image_to_canvas(image: bmp::Image, path: String) -> Canvas {
    Canvas {
        path,
        width: image.get_width(),
        height: image.get_height(),
        image,
    }
}

#[derive(Debug)]
enum CanvasError {
    Bmp(bmp::BmpError),
    Io(io::Error),
    InvalidPoint(String),
    InvalidPoints(String),
    OutBounds(Point),
}

impl Canvas {
    fn draw_pixel(&mut self, p: Point, color: Option<bmp::Pixel>) -> Result<(), CanvasError> {
        if self.out_bounds(&p) {
            return Err(CanvasError::OutBounds(p));
        }
        let pixel = match color {
            Some(c) => c,
            None => bmp::Pixel::new(255, 255, 255),
        };
        self.image.set_pixel(p.x, p.y, pixel);
        Ok(())
    }

    fn out_bounds(&self, p: &Point) -> bool {
        p.x > self.width || p.y > self.height
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

    fn draw_line_vertical(&mut self, p1: &Point, p2: &Point, color: Option<bmp::Pixel>) -> Result<(), CanvasError> {
        // Draw a line with a constant `x` value
        if p1.x != p2.x {
            return Err(CanvasError::InvalidPoint(String::from(
                "Points must have the same x value",
            )));
        }
        let (start, end) = if p1.y < p2.y { (p1, p2) } else { (p2, p2) };
        for i in start.y..end.y + 1 {
            let p = Point { x: p1.x, y: i };
            self.draw_pixel(p, color).expect("Error drawing pixel");
        }
        Ok(())
    }

    fn draw_line_horizontal(&mut self, p1: &Point, p2: &Point, color: Option<bmp::Pixel>) -> Result<(), CanvasError> {
        // Horizontal line has fixed `y` value
        if p1.y != p2.y {
            return Err(CanvasError::InvalidPoint(String::from(
                "Points must have the same y value",
            )));
        }
        let y = p1.y;
        let (start, end) = if p1.x < p2.x { (p1, p2) } else { (p2, p2) };
        for x in start.x..end.x + 1 {
            let p = Point { x, y };
            self.draw_pixel(p, color).expect("Error drawing pixel");
        }
        Ok(())
    }

    fn draw_line(&mut self, p1: &Point, p2: &Point, color: Option<bmp::Pixel>) -> Result<(), crate::CanvasError> {
        // Increment along `x` values with gradient of y
        if self.out_bounds(p1) {
            return Err(CanvasError::OutBounds(*p1));
        }
        if self.out_bounds(p2) {
            return Err(CanvasError::OutBounds(*p2));
        }
        if p1.x == p2.x {
            return self.draw_line_vertical(p1, p2, color);
        }
        if p1.y == p2.y {
            return self.draw_line_horizontal(p1, p2, color);
        }

        let (start, end) = if p1.x < p2.x { (p1, p2) } else { (p2, p1) };

        let dy: f64 = end.y as f64 - start.y as f64;
        let dx: f64 = end.x as f64 - start.x as f64;
        let gradient = dy / dx;

        let mut j = start.y;
        for i in start.x..end.x + 1 {
            self.draw_pixel(Point { x: i, y: j }, color)?;
            j = (gradient * (i - start.x) as f64 + start.y as f64) as u32;
        }

        Ok(())
    }

    fn draw_outlined_square(
        &mut self, start: Point, size: u32, color: Option<bmp::Pixel>
    ) -> Result <(), CanvasError> {
        self.draw_outlined_rectangle(start, size, size, color)
    }

    fn draw_outlined_rectangle(
        &mut self, start: Point, height: u32, width: u32, color: Option<bmp::Pixel>
    ) -> Result<(), CanvasError> {
        if width == 0 || height == 0 {
            return Ok(());
        }

        let top_l = start;
        let top_r = Point { x: start.x + width, y: start.y };
        let bot_l = Point { x: start.x, y: start.y + height };
        let bot_r = Point { x: start.x + width, y: start.y + height };
        match self.draw_line(&top_l, &top_r, color) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        match self.draw_line(&top_r, &bot_r, color) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        match self.draw_line(&bot_l, &bot_r, color) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        match self.draw_line(&top_l, &bot_l, color) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        Ok(())
    }

    fn draw_filled_rectangle(
        &mut self, start: Point, height: u32, width: u32, fill: Option<bmp::Pixel>, outline: Option<bmp::Pixel>
    ) -> Result<(), CanvasError> {
        if width == 0 || height == 0 {
            return Ok(());
        }
        // Do fill before outline
        for i in start.x..start.x + width + 1 {
            for j in start.y..start.y + height + 1 {
                let p = Point { x: i, y: j };
                self.draw_pixel(p, fill)?;
            }
        }
        self.draw_outlined_rectangle(start, height, width, outline);
        Ok(())
    }
}
