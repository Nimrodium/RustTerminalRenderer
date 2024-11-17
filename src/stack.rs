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
pub type SpriteVector = Vec<Pixel>;
pub struct Layer {
    pub buffer: Vec<Pixel>,
    height: u16,
    width: u16,
}
pub struct FrameBuffer {
    pub buffer: Vec<Pixel>,
    height: u16,
    width: u16,
}

///Used to draw a pixel to the screen NOT framebuffer, this is for final commit
pub fn draw_pixel(x: u16, y: u16, color: Color, stdout: &mut Stdout, i: i16) -> io::Result<()> {
    //let element = format!("|{} ", i);
    queue!(
        stdout,
        cursor::MoveTo(x * 2, y),
        style::PrintStyledContent(PIXEL_ELEMENT.with(color))
    )?;

    Ok(())
}

///renders blank framebuffer
pub fn init_framebuffer(
    x_aspect: u16,
    y_aspect: u16,
    bg_color: Color,
) -> Result<FrameBuffer, String> {
    //let mut framebuffer: FrameBuffer = vec![];
    let mut framebuffer: FrameBuffer = FrameBuffer {
        buffer: vec![],
        height: y_aspect,
        width: x_aspect,
    };
    for y_framebuffer in 0..y_aspect {
        for x_framebuffer in 0..x_aspect {
            let working_pixel: Pixel = Pixel {
                x: x_framebuffer,
                y: y_framebuffer,
                layer: 0,
                color: bg_color,
                object_id: 0,
            };
            framebuffer.buffer.push(working_pixel);
        }
    }

    let expected = framebuffer.buffer.len();
    let real = (x_aspect * y_aspect) as usize;
    if expected != real {
        let why = format!(
            "Failed to define canvas\n    expected canvas length: {}\n    real canvas length: {}",
            expected, real,
        );
        Err(why)
    } else {
        Ok(framebuffer)
    }
}
///# to_worldspace
///## Translates sprite's positional data into worldspace
///takes in the worldspace x and y (relative to the top left corner of the sprite) and returns the moved pixel vector
pub fn to_worldspace(
    x_world: u16,
    y_world: u16,
    layer_world: u16,
    sprite: &Sprite,
    framebuffer: &FrameBuffer,
) -> SpriteVector {
    let mut pixels: SpriteVector = vec![];

    for pixel in sprite.pixels.iter() {
        let x_pixel = pixel.x + x_world;
        let y_pixel = pixel.y + y_world;
        //if these are higher than the x y aspect of framebuffer then skip creation
        if (x_pixel < framebuffer.width) && (y_pixel < framebuffer.height) {
            let working_pixel: Pixel = Pixel {
                x: pixel.x + x_world,
                y: pixel.y + y_world,
                layer: layer_world,
                color: pixel.color,
                object_id: pixel.object_id,
            };
            pixels.push(working_pixel);
        };
    }
    pixels
}
///# push_render
///## writes a frame to terminal
///takes in any vector of pixels and prints to the terminal.

pub fn framebuffer_write(
    x: u16,
    y: u16,
    layer: u16,
    sprite: &Sprite,
    framebuffer: &mut FrameBuffer,
) {
    let sprite_worldspace = to_worldspace(x, y, layer, sprite, framebuffer);
    for sprite_pixel in sprite_worldspace.iter() {
        let raw_index: usize =
            ((framebuffer.width as usize) * sprite_pixel.y as usize) + sprite_pixel.x as usize;
        let frame_pixel_opt = framebuffer.buffer.get_mut(raw_index);

        if let Some(frame_pixel) = frame_pixel_opt {
            frame_pixel.color = sprite_pixel.color;
            frame_pixel.layer = sprite_pixel.layer;
            frame_pixel.object_id = sprite_pixel.object_id;
        } else {
            println!("Framebuffer does not contain referenced pixel, ignoring...");
        }
    }
}

pub fn push_render(frame: Vec<Pixel>) {
    let mut stdout = io::stdout();
    //execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    for (i, pixel) in frame.iter().enumerate() {
        draw_pixel(pixel.x, pixel.y, pixel.color, &mut stdout, i as i16).unwrap();
    }
    stdout.flush().unwrap();
}

//Render API

/*
clear
update

*/
///# Renderer
///## RustTermRenderer Render Engine API
struct Renderer {
    basebuffer: FrameBuffer,
    stdout: std::io::Stdout,
}

impl Renderer {
    fn new(x: u16, y: u16, color: Color) -> Self {
        Renderer {
            basebuffer: init_framebuffer(x, y, color).unwrap(),
            stdout: std::io::stdout(),
        }
    }
    ///# Clear
    ///## Clears Screen
    fn clear() {
        println!("clear");
    }
    ///# Update
    ///## Pushes changes made to the buffers to the screen
    fn update() {
        println!("will update the view with the staged changes");
    }
    ///# Add Layer
    ///## Adds a new layer entry in the render pipeline
    fn add_layer(layer_id: u16, pos: u16) {
        println!("will create a new layer entry in the render queue");
    }
    ///# Remove Layer
    ///## Removes a layer entry in the render pipeline
    fn remove_layer(layer_id: u16) {
        println!("will remove the specified layer");
    }
    ///# Set Layer Visibility
    ///## Changes if the layer is included in the render pipeline
    fn set_layer_visibility(layer_id: u16, visible: bool) {
        println!("will set layer rendering, even pixels with render True will not render if the layer is not visible");
    }
    ///# Move layer
    ///## Moves a layers position in the render pipeline
    fn move_layer(layer_id: u16, new_pos: u16) {
        println!("changes layer queue sequence");
    }
    ///# Direct Write
    ///## Directly writes to a pixel in the specified layer, bypassing sprite logic
    fn direct_write(x: u16, y: u16, color: Color, layer_id: u16) {
        println!("Directly writes to a pixel, bypassing sprite logic");
    }
    ///# Draw Sprite
    ///## Draws a Sprite to the layer at the specified location
    fn draw_sprite(x: u16, y: u16, sprite: Sprite, layer: u16) {
        println!("will write the sprite vector to the specified layer");
    }
    ///# Reset
    ///## Resets the Renderer
    ///upon invoking the layer queue is cleared and the basebuffer is reinitialized
    fn reset() {
        println!(
            "will reset the renderer by deleting all renders and reinitializing the basebuffer"
        );
    }
    ///# Debug Mode
    ///## Toggles debug logging
    ///when enabled the rendering engine will write logs to renderer.log in the directory of the compiled executable
    fn debug_mode(toggle: bool) {
        println!("toggles debug logging");
    }
}
