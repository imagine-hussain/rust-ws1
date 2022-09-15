use bmp;
use std::io::{stdout, Write};

fn draw_pixel(path: &str) {
    let mut image = match bmp::open(path) {
        Ok(i) => i,
        Err(_) => bmp::Image::new(100, 100),
    };

    image.set_pixel(50, 50, bmp::Pixel::new(255, 255, 255));

    image.save(path).expect("This should save correctly.");
}


fn draw_x(path: &str) { 
    let mut image = match bmp::open(path) {
        Ok(i) => i,
        Err(_) => bmp::Image::new(100, 100)
    };
    
    for i in 0..100 {
        image.set_pixel(i, i, bmp::Pixel::new(255, 255, 255));
        image.set_pixel(i, 99 - i, bmp::Pixel::new(255, 255, 255));
    }
    image.save(path).expect("Image couldn't save");

}


fn draw_line(image: &mut bmp::Image, u_x: u32, u_y: u32, v_x: u32, v_y: u32, canvas: bmp::Pixel) {
    let mut gradient = (v_y - u_y) / (v_x - u_x);
    for x in u_x..v_x {
        let y = gradient * (x - u_x) + u_y;
        image.set_pixel(x, y, canvas);
    }

} 

fn draw_house(path: &str) { 
    let mut image = match bmp::open(path) {
        Ok(i) => i,
        Err(_) => bmp::Image::new(100, 100)
    };

    draw_rectangle(image, 0, 40, 100, 100, bmp::Pixel::new(255, 255, 255));
    draw_rectangle(image, 20, 20, 80, 40, bmp::Pixel::new(255, 255, 255));
    draw_line(image, 20, 20, 50, 0, bmp::Pixel::new(255, 255, 255));
    draw_line(image, 50, 0, 80, 20, bmp::Pixel::new(255, 255, 255));
}


fn draw_pixel_at_xy(image: &mut bmp::Image, x: u32, y: u32) {
    image.set_pixel(x, y, bmp::Pixel::new(255, 255, 255));
}

fn draw_outlined_square(path: &str, width: u32) {
    let mut image = match bmp::open(path) {
        Ok(i) if i.get_width() == i.get_height() => i,
        _ => bmp::Image::new(width, width)
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
        Ok(i) if i.get_width() == i.get_height() => i,
        _ => bmp::Image::new(width, width)
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

fn draw_rectangle(image: &mut bmp::Image, x: u32, y: u32, w: u32, h: u32, c: bmp::Pixel) {
    for x_pixel in x..w {
        for y_pixel in y..h {
            image.set_pixel(x_pixel, y_pixel, c);
        }
    }
}

fn draw_finnish_flag(path: &str) {
    let mut image = bmp::Image::new(180, 110);

    draw_rectangle(&mut image, 0, 0, 180, 110, bmp::Pixel::new(255, 255, 255));
    draw_rectangle(&mut image, 50, 0, 80, 110, bmp::Pixel::new(0, 0, 255));
    draw_rectangle(&mut image, 0, 40, 180, 70, bmp::Pixel::new(0, 0, 255));

    image.save(path).expect("Image couldn't save");
}

fn read() -> String {
    let mut op = String::new();
    std::io::stdin().read_line(&mut op).unwrap();

    op
}

enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

fn draw_diagonal_line(image: bmp::Image, start: Option<Corner>, color: Option<bmp::Pixel>) -> Result<bmp::Image, String> {
    if image.get_width() != image.get_height() {
        return Err(String::from("Image is not square"));
    }
    if image.get_width() == 0 {
        return Err(String::from("Image is empty"));
    }

    let color = match color {
        Some(c) => c,
        None => bmp::Pixel::new(255, 255, 255),
    };
    let start: Corner = match start {
        Some(s) => s,
        None => Corner::TopLeft,
    };
    let x: u32 = match start {
        Corner::TopLeft => 0,
        Corner::TopRight => image.get_width() - 1,
        Corner::BottomLeft => 0,
        Corner::BottomRight => image.get_width() - 1,
    };
    let y: u32 = match start {
        Corner::TopLeft => 0,
        Corner::TopRight => 0,
        Corner::BottomLeft => image.get_height() - 1,
        Corner::BottomRight => image.get_height() - 1,
    };
    loop {
        if out_bounds(image, x, y) {
            break;
        }
        image.set_pixel(x, y, color);
        let (x, y) = diagonal_step(start, x, y);
    }
    return Ok(image);
}

fn out_bounds(image: bmp::Image, x: u32, y: u32) -> bool {
    // ayo, no implicity bool cnversion?
    x <= 0 || x >= image.get_width() || y < 0 || y >= image.get_height()
}

fn diagonal_step(start, x, y) -> (u32, u32) {
    // Increment for a diagonal step based on starting direction
    match start {
        Corner::TopLeft => (x + 1, y + 1),
        Corner::TopRight => (x - 1, y + 1),
        Corner::BottomLeft => (x + 1, y - 1),
        Corner::BottomRight => (x - 1, y - 1),
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("You must provide a path.");

    print!("Which operation? ");
    // We use "flush" so that we see the question before the answer.
    // We can only use `flush` when we use `Write` too -- don't worry why yet!
    stdout().flush().unwrap();

    let op = read();

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
        "diagonal\n" => match read().as_str() {
            "right\n" => todo!("diagonal right"),
            "left\n" => todo!("diagonal left"),
            _ => eprintln!("Expected left or right for diagonal"),
        },
        "cross\n" => todo!("cross"),
        "house\n" => todo!("house"),
        "outline\n" => {
            let coords = read();
            let parts: Vec<_> = coords.split_whitespace().collect();
            if parts.len() == 4 {
                todo!("square outline")
            } else {
                eprintln!("Expected \"t l w h\" as coordinates to square outline")
            }
        }
        "flag\n" => match read().as_str() {
            "rainbow\n" => todo!("rainbow"),
            "finland\n" => todo!("finland"),
            "iceland\n" => todo!("iceland"),
            "aboriginal\n" => todo!("aboriginal"),
            _ => eprintln!("Expected rainbow | finland | iceland | aboriginal for diagonal"),
        },
        "sine\n" => todo!("sine"),
        _ => eprintln!("The operation {op} was not recognised!"),
    }
}
