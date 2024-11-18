//stack.rs
///# Stack Module
///##   Handles commiting data to the terminal
use crate::sprite::{Pixel, Sprite};
use crossterm::{
    cursor, execute, queue,
    style::{self, Color, Stylize},
    terminal,
};
//use std::fs;
//use std::path::Path;
use std::{
    collections::HashMap,
    io::{self, Stdout, Write},
};
use std::{thread, time};
//██
//▆
static DEBUG: bool = false;

//static DEBUGFILE: File = File::Open()

/*macro_rules! debug {
    (msg) => {
        if DEBUG {
            let logfile: File = File::Open(Path::new("Renderstack.log"));
            logfile.wri
        }
    };
}*/

///string to represent each pixel by.
const PIXEL_ELEMENT: &str = "██";

///FrameBuffer type holds worldspace before commit
pub type SpriteVector = Vec<Pixel>;
//pub type Layer = Vec<Vec<Pixel>>;

pub struct Layer {
    pub buffer: Vec<Vec<Pixel>>,
    height: u16,
    width: u16,
    queue_pos: u16,
    isRendered: bool,
}
#[derive(Clone)]
pub struct FrameBuffer {
    pub buffer: Vec<Pixel>,
    color: Color,
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
pub fn init_layer(
    x_aspect: u16,
    y_aspect: u16,
    bg_color: Color,
    basebuffer: bool,
) -> Result<FrameBuffer, String> {
    //let mut framebuffer: FrameBuffer = vec![];
    let mut framebuffer: FrameBuffer = FrameBuffer {
        buffer: vec![],
        color: bg_color,
        height: y_aspect,
        width: x_aspect,
    };
    for y_framebuffer in 0..y_aspect {
        for x_framebuffer in 0..x_aspect {
            let working_pixel: Pixel = Pixel {
                x: x_framebuffer,
                y: y_framebuffer,
                //layer: 0,
                color: bg_color,
                isrendered: true,
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
                //layer: layer_world,
                color: pixel.color,
                isrendered: pixel.isrendered,
            };
            pixels.push(working_pixel);
        };
    }
    pixels
}
///# push_render
///## writes a frame to terminal
///takes in any vector of pixels and prints to the terminal.

fn get_raw_index(width: usize, x: usize, y: usize) -> usize {
    let raw_index: usize = width * y + x as usize;
    raw_index
}

pub fn framebuffer_write(
    x: u16,
    y: u16,
    layer: u16,
    sprite: &Sprite,
    framebuffer: &mut FrameBuffer,
) {
    let sprite_worldspace = to_worldspace(x, y, layer, sprite, framebuffer);
    for sprite_pixel in sprite_worldspace.iter() {
        let raw_index: usize = get_raw_index(
            framebuffer.width as usize,
            sprite_pixel.x as usize,
            sprite_pixel.y as usize,
        );
        let frame_pixel_opt = framebuffer.buffer.get_mut(raw_index);

        if let Some(frame_pixel) = frame_pixel_opt {
            frame_pixel.color = sprite_pixel.color;
            //frame_pixel.layer = sprite_pixel.layer;
        } else {
            println!("Framebuffer does not contain referenced pixel, ignoring...");
        }
    }
}

pub fn push_render(layer: Vec<Pixel>) {
    let mut stdout = io::stdout();
    //execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    for (i, pixel) in layer.iter().enumerate() {
        if pixel.isrendered {
            draw_pixel(pixel.x, pixel.y, pixel.color, &mut stdout, i as i16).unwrap();
        }
    }
    stdout.flush().unwrap();
}

///Renderer API
impl FrameBuffer {
    ///initializes framebuffer
    fn new(x: u16, y: u16, color: Color) -> Self {
        init_layer(x, y, color, true).unwrap()
    }
    ///write to framebuffer
    fn write(&mut self, x: u16, y: u16, layer: u16, sprite: &Sprite) {
        let sprite_worldspace = to_worldspace(x, y, layer, sprite, self);
        for sprite_pixel in sprite_worldspace.iter() {
            let raw_index: usize =
                ((self.width as usize) * sprite_pixel.y as usize) + sprite_pixel.x as usize;
            let frame_pixel_opt = self.buffer.get_mut(raw_index);

            if let Some(frame_pixel) = frame_pixel_opt {
                frame_pixel.color = sprite_pixel.color;
            } else {
                println!("Framebuffer does not contain referenced pixel, ignoring...");
            }
        }
    }
    ///Translates a SpriteVector into a worldspace position
    fn to_worldspace(
        &self,
        x_world: u16,
        y_world: u16,
        layer_world: u16,
        sprite: &Sprite,
    ) -> SpriteVector {
        let mut pixels: SpriteVector = vec![];

        for pixel in sprite.pixels.iter() {
            let x_pixel = pixel.x + x_world;
            let y_pixel = pixel.y + y_world;
            //if these are higher than the x y aspect of framebuffer then skip creation
            if (x_pixel < self.width) && (y_pixel < self.height) {
                let working_pixel: Pixel = Pixel {
                    x: pixel.x + x_world,
                    y: pixel.y + y_world,
                    //layer: layer_world,
                    color: pixel.color,
                    isrendered: pixel.isrendered,
                };
                pixels.push(working_pixel);
            };
        }
        pixels
    }
}

///# Renderer
///## RustTermRenderer Render Engine API
struct Renderer {
    ///buffer to write all screen changes to before committing to display
    buffer: FrameBuffer,
    ///hashmap of layers referenced by their ID, render order is determined by their render_pos value
    renderqueue: HashMap<u16, Layer>,
    ///stdout of the Renderer
    stdout: std::io::Stdout,
    ///framerate of Renderer, default value is 25fps (40ms)
    framerate: time::Duration,
    ///debug flag
    debug: bool,
}

impl Renderer {
    fn new(x: u16, y: u16, color: Color) -> Self {
        Renderer {
            buffer: FrameBuffer::new(x, y, color),
            renderqueue: HashMap::new(),
            stdout: std::io::stdout(),
            framerate: time::Duration::from_millis(40),
            debug: false,
        }
    }
    ///# Clear
    ///## Clears Terminal
    fn clear() {
        print!("\x1b[2J\x1b[H");
    }
    ///sets framerate interval in milliseconds,
    /// default is 25fps (40ms)
    fn set_framerate(&mut self, new_framerate: u64) {
        self.framerate = time::Duration::from_millis(new_framerate);
    }
    ///commits layers to the framebuffer to prepare for render push
    fn compose_frame(self) {}
    ///# Update
    ///## Pushes changes made to the buffer to the screen
    fn update(&mut self) {
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
        //push_render(self.buffer.buffer.clone());
        //pushes render
        for (i, pixel) in self.buffer.buffer.iter().enumerate() {
            if pixel.isrendered {
                draw_pixel(pixel.x, pixel.y, pixel.color, &mut self.stdout, i as i16).unwrap();
            }
        }
        self.stdout.flush().unwrap();
        thread::sleep(self.framerate);
    }
    fn push_render(&mut self) {}
    ///returns a mutable Layer from the renderqueue
    fn fetch_layer(&mut self, id: u16) -> &mut Layer {
        if let Some(layer) = self.renderqueue.get_mut(&id) {
            layer
        } else {
            panic!("[error] layer {} does not exist", id);
        }
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
    fn set_layer_visibility(&mut self, layer_id: u16, visible: bool) {
        let layer = self.fetch_layer(layer_id);
        layer.isRendered = visible;
    }
    ///# Move layer
    ///## Moves a layers position in the render pipeline
    fn move_layer(layer_id: u16, new_pos: u16) {
        println!("changes layer queue sequence");
    }

    ///# Direct Write
    ///## Directly writes to a pixel in the specified layer, bypassing sprite logic
    /// this function overrites the specified pixel, if another sprite later on in the Layer
    /// contains the same data for a pixel the direct write pixel will be overwritten as well
    fn direct_write(&mut self, x: u16, y: u16, color: Color, layer_id: u16) {
        let layer = self.fetch_layer(layer_id);
        let new_pixel: Pixel = Pixel {
            x: x,
            y: y,
            color: color,
            isrendered: true,
        };
        layer.buffer.push(vec![new_pixel]);
    }
    ///# Draw Sprite
    ///## Draws a Sprite to the layer at the specified location
    fn write_sprite(&mut self, x: u16, y: u16, sprite: Sprite, layer_id: u16) -> () {
        println!("will write the sprite vector to the specified layer");
        let worldspace_spritevector = self.buffer.to_worldspace(x, y, layer_id, &sprite);
        let layer = self.fetch_layer(layer_id);
        layer.buffer.push(worldspace_spritevector);
    }
    ///# Debug Mode
    ///## Toggles debug logging
    ///when enabled the rendering engine will write logs to renderer.log in the directory of the compiled executable
    fn debug_mode(&mut self, toggle: bool) {
        println!("toggles debug logging");
    }
}
