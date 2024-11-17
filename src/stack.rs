//stack.rs
///# Stack Module
///##   Handles commiting data to the terminal
use crate::sprite::{Pixel, Sprite};
use crossterm::{
    cursor, execute, queue,
    style::{self, Color, Stylize},
    terminal,
};
use std::io::{self, Stdout, Write};
//██
//▆
///string to represent each pixel by.
const PIXEL_ELEMENT: &str = "██";

///FrameBuffer type holds worldspace before commit
pub type Canvas = Vec<Pixel>;

///Used to draw a pixel to the screen NOT framebuffer, this is for final commit
pub fn draw_pixel(x: u16, y: u16, color: Color, stdout: &mut Stdout) -> io::Result<()> {
    queue!(
        stdout,
        cursor::MoveTo(x * 2, y),
        style::PrintStyledContent(PIXEL_ELEMENT.with(color))
    )?;

    Ok(())
}
///# to_worldspace
///## Translates sprite's positional data into worldspace
///takes in the worldspace x and y (relative to the top left corner of the sprite) and returns the moved pixel vector

///renders blank canvas
pub fn build_canvas(x_aspect: u16, y_aspect: u16, bg_color: Color) -> Result<Canvas, String> {
    let mut canvas: Canvas = vec![];

    for y_canvas in 0..=y_aspect {
        for x_canvas in 0..=x_aspect {
            let working_pixel: Pixel = Pixel {
                x: x_canvas,
                y: y_canvas,
                layer: 0,
                color: bg_color,
                object_id: 0,
            };
            canvas.push(working_pixel);
        }
    }

    let expected = canvas.len();
    let real = (x_aspect * y_aspect) as usize;
    if expected != real {
        let why = format!(
            "Failed to define canvas\n    expected canvas length: {}\n    real canvas length: {}",
            expected, real,
        );
        Err(why)
    } else {
        Ok(canvas)
    }
}
pub fn to_worldspace(
    x_world: u16,
    y_world: u16,
    layer_world: u16,
    pixel_vector: Vec<Pixel>,
) -> Vec<Pixel> {
    let mut pixels: Vec<Pixel> = vec![];

    for pixel in pixel_vector.iter() {
        let working_pixel: Pixel = Pixel {
            x: pixel.x + x_world,
            y: pixel.y + y_world,
            layer: layer_world,
            color: pixel.color,
            object_id: pixel.object_id,
        };
        pixels.push(working_pixel);
    }
    pixels
}
///# push_render
///## writes a frame to terminal
///takes in any vector of pixels and prints to the terminal.
pub fn push_render(frame: Vec<Pixel>) {
    let mut stdout = io::stdout();
    //execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    for pixel in frame.iter() {
        draw_pixel(pixel.x, pixel.y, pixel.color, &mut stdout).unwrap();
    }
    stdout.flush().unwrap();
}
