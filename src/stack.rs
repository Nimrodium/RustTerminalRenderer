//stack.rs
///# Stack Module
///##   Handles commiting data to the terminal
use crate::sprite::{Pixel, Sprite};
use core::unicode::conversions::to_lower;
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

///Unique Identifier for layers in the layerstack, used to reference layers when staging writes to the framebuffer
pub type LayerID = u16;
//pub type Layer = Vec<Vec<Pixel>>;
///Represents a distinct grouping of `SpriteVectors` in 3d space
pub struct Layer {
    pub buffer: Vec<SpriteVector>,
    height: u16,
    width: u16,
    stack_pos: u16,
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
    layer: LayerID,
    sprite: &Sprite,
    framebuffer: &mut FrameBuffer,
) {
    let sprite_worldspace = to_worldspace(x, y, sprite, framebuffer);
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
        //let mut framebuffer: FrameBuffer = vec![];
        let mut framebuffer: FrameBuffer = FrameBuffer {
            buffer: vec![],
            color: color,
            height: x,
            width: y,
        };
        for y_framebuffer in 0..y {
            for x_framebuffer in 0..x {
                let working_pixel: Pixel = Pixel {
                    x: x_framebuffer,
                    y: y_framebuffer,
                    //layer: 0,
                    color: color,
                    isrendered: true,
                };
                framebuffer.buffer.push(working_pixel);
            }
        }
        framebuffer
    }
    ///write to framebuffer
    ///this function should only be called by the layerstack rasterizer during rasterization
    fn write(&mut self, x: u16, y: u16, sprite: &Sprite) {
        let sprite_worldspace = self.to_worldspace(x, y, sprite);
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
    fn to_worldspace(&self, x_world: u16, y_world: u16, sprite: &Sprite) -> SpriteVector {
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
    ///data structure to organize the sequence in which to write layers to the framebuffer,
    layerstack: HashMap<u16, Layer>,
    ///stdout of the Renderer
    stdout: std::io::Stdout,
    ///framerate of Renderer, default value is 25fps (40ms)
    framerate: time::Duration,
    ///debug flag
    debug: bool,
}

enum ShiftDirection {
    Up,
    Down,
}

impl Renderer {
    fn new(x: u16, y: u16, color: Color) -> Self {
        Renderer {
            buffer: FrameBuffer::new(x, y, color),
            layerstack: HashMap::new(),
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
    ///rasterizes (flattens) layers into the 2d framebuffer
    fn layerstack_rasterize(self) {}

    /// Pushes changes made to the buffer to the screen
    fn render_update(&mut self) {
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
        //push_render(self.buffer.buffer.clone());
        //pushes render
        self.render_push();
        self.stdout.flush().unwrap();
        thread::sleep(self.framerate);
    }
    fn render_push(&mut self) {
        for (i, pixel) in self.buffer.buffer.iter().enumerate() {
            if pixel.isrendered {
                draw_pixel(pixel.x, pixel.y, pixel.color, &mut self.stdout, i as i16).unwrap();
            }
        }
    }

    ///returns a mutable Layer from the renderqueue
    ///# Parameters
    /// - `id` : requested layerID

    fn layer_fetch(&mut self, id: LayerID) -> &mut Layer {
        if let Some(layer) = self.layerstack.get_mut(&id) {
            layer
        } else {
            panic!("[error] layer {} does not exist", id);
        }
    }
    ///creates a new layer entry in the layerstack in the specified position
    ///# Parameters
    ///- `layer_id` : a new unique identification for the layer
    ///- `pos` : position to insert new layer
    /// Removes the specified layer from the layerstack.
    ///
    /// # Behavior
    /// **Shifting behavior**:
    /// - shifts all layers with positions higher than `pos` up by one to make room for the added layer
    /// - adds a new layer to the layerstack with id `layer_id` and position `pos`.
    ///
    /// # Example
    /// ```
    /// layer_add(1,0);
    /// // adds layer with ID 1.
    /// // This shifts all subsequent layers after `pos` in the layerstack by 1. then fills the void `pos` with the added layer
    /// ```

    fn layer_add(layer_id: LayerID, pos: u16) {
        println!("will create a new layer entry in the render queue");
    }
    /// Moves the specified layer to a new position in the layer stack.
    ///
    /// # Parameters
    /// - `layer_id`: The ID of the layer to move.
    /// - `new_pos`: The new position to which the layer should be moved.
    ///
    /// # Behavior
    /// **Shifting behavior**:
    /// - Shifts all layers with positions higher than `new_pos` up by one to make room for the layer.
    /// - The layer specified by `layer_id` is moved to `new_pos`.
    /// - All layers with positions higher than the original position of `layer_id` are shifted down by one.
    /// - The function ensures that layers are re-ordered in a way that maintains the correct hierarchy in the stack.
    ///
    /// # Example
    /// ```
    /// layer_move(1, 0);
    /// // Moves layer with ID 1 to position 0.
    /// // This makes layer 1 the first layer to write to the framebuffer during rasterization.
    /// ```
    fn layer_move(layer_id: LayerID, new_pos: u16) {
        println!("changes layer queue sequence");
    }
    /// Removes the specified layer from the layerstack.
    ///
    /// # Parameters
    /// - `layer_id`: The ID of the layer to remove.
    ///
    ///
    /// # Behavior
    /// **Shifting behavior**:
    /// - removes layer with `layer_id` ID
    /// - shifts all layers with positions higher than the removed layer down by one
    ///
    /// # Example
    /// ```
    /// layer_remove(1);
    /// // Removes layer with ID 1.
    /// // This shifts all subsequent layers in the layerstack down by 1.
    /// ```
    fn layer_remove(layer_id: LayerID) {
        println!("will remove the specified layer");
    }
    ///# Set Layer Visibility
    ///## Changes if the layer is included in the render pipeline
    fn layer_set_visibility(&mut self, layer_id: LayerID, visible: bool) {
        let layer = self.layer_fetch(layer_id);
        layer.isRendered = visible;
    }

    /// Moves layers relative to the starting position.
    ///
    /// # Parameters
    /// - `starting_pos`: The position from which shifting starts.
    /// - `direction`: Specifies the direction of the shift (Up or Down).
    ///
    /// # Behavior
    ///
    /// - **Up**: Shifts all values higher than the starting position up by one,
    ///   opening the starting position for a move or addition of a new layer.
    ///
    /// - **Down**: Shifts all values higher than the starting position down by one,
    ///   closing the starting position in the case of a layer removal or move.
    fn layer_shift(&mut self, starting_pos: LayerID, direction: ShiftDirection) {
        for layer in self.layerstack.values_mut() {
            //if greater than starting pos
            if layer.stack_pos > starting_pos {
                match direction {
                    //shift all values up
                    ShiftDirection::Up => layer.stack_pos += 1,
                    //shift all values down
                    ShiftDirection::Down => layer.stack_pos -= 1,
                }
            }
        }
    }

    ///# Draw Sprite
    ///## Draws a Sprite to the layer at the specified location
    fn layer_write_sprite(&mut self, x: u16, y: u16, sprite: Sprite, layer_id: LayerID) -> () {
        println!("will write the sprite vector to the specified layer");
        let worldspace_spritevector = self.buffer.to_worldspace(x, y, &sprite);
        let layer = self.layer_fetch(layer_id);
        layer.buffer.push(worldspace_spritevector);
    }

    ///# Direct Write
    ///## Directly writes to a pixel in the specified layer, bypassing sprite logic
    /// this function overrites the specified pixel, if another sprite later on in the Layer
    /// contains the same data for a pixel the direct write pixel will be overwritten as well
    fn layer_direct_write(&mut self, x: u16, y: u16, color: Color, layer_id: LayerID) {
        let layer = self.layer_fetch(layer_id);
        let new_pixel: Pixel = Pixel {
            x: x,
            y: y,
            color: color,
            isrendered: true,
        };
        layer.buffer.push(vec![new_pixel]);
    }

    ///# Debug Mode
    ///## Toggles debug logging
    ///when enabled the rendering engine will write logs to renderer.log in the directory of the compiled executable
    fn debug_mode(&mut self, toggle: bool) {
        println!("toggles debug logging");
    }
}
