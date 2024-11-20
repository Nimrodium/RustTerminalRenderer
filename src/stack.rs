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

///FrameBuffer type holds worldspace before commit
pub type SpriteVector = Vec<Pixel>;

///Unique Identifier for layers in the layerstack, used to reference layers when staging writes to the framebuffer
pub type LayerID = u16;
//pub type Layer = Vec<Vec<Pixel>>;
///Represents a distinct grouping of `SpriteVectors` in 3d space
pub struct Layer {
    pub buffer: Vec<SpriteVector>,
    stack_pos: u16,
    is_rendered: bool,
}

///Used to draw a pixel to the screen NOT framebuffer, this is for final commit

///renders blank framebuffer
/*
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
///Translates sprite's positional data into worldspace
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
*/
///returns the raw (flattened) index of a value based on its x and y position.
///returns `usize`.
///# Parameters
///- `width` : width aka maximum x value of a row of the structure.
///- `x` : target position x coordinate.
///- `y` : target position y coordinate.
///# Example
///```
///let get_raw_index(5,4,2);
fn get_raw_index(width: usize, x: usize, y: usize) -> usize {
    let raw_index: usize = width * y + x as usize;
    raw_index
}
/*
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
}*/

///RustTermRenderer Rendering Engine API
///used to interface with the rendering engine
#[derive(Clone)]
pub struct FrameBuffer {
    pub buffer: Vec<Pixel>,
    color: Color,
    height: u16,
    width: u16,
}
/// collection of sprites to draw at a depth
struct Layerstack {
    stack: HashMap<LayerID, Layer>,
    framebuffer: FrameBuffer,
    sequence: Vec<LayerID>,
    sequence_rebuild: bool,
}

pub struct Renderer {
    ///data structure to organize the sequence in which to write layers to the framebuffer,
    layerstack: Layerstack,
    /////cache of layer position sequence by id
    //layerstack_sequence: Vec<LayerID>,
    /////rebuild sequence flag
    //layerstack_sequence_rebuild: bool,
    pixel_element: String,
    ///stdout of the Renderer
    stdout: std::io::Stdout,
    ///framerate of Renderer, default value is 25fps (40ms)
    framerate: time::Duration,
    ///debug flag
    debug: bool,
}

//TODO might move to render_api.rs
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
    fn write_layer(&mut self, layer: &Layer) {
        for sprite_vector in layer.buffer.iter() {
            for sprite_pixel in sprite_vector.iter() {
                let raw_index: usize = get_raw_index(
                    self.width as usize,
                    sprite_pixel.x as usize,
                    sprite_pixel.y as usize,
                );
                if let Some(buffer_pixel) = self.buffer.get_mut(raw_index) {
                    buffer_pixel.color = sprite_pixel.color;
                } else {
                    println!("FrameBuffer does not contain referenced pixel");
                }
            }
        }
    }
    ///returns a transformed SpriteVector of a Sprite in a worldspace position
    ///# Example
    ///```
    ///to_worldspace(10,15,dino);
    ///```
    ///returns SpriteVector of dino at worldspace position (10,15) (of sprite center).
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
impl Layerstack {
    fn new(width: u16, height: u16, bg_color: Color) -> Self {
        Layerstack {
            stack: HashMap::new(),
            framebuffer: FrameBuffer::new(width, height, bg_color),
            sequence: vec![],
            sequence_rebuild: true,
            //this will cause the sequence to be initially built for the first frame
        }
    }
    ///rasterizes (flattens) layers into the 2d framebuffer
    ///often used before `push_render()`
    ///# Example
    ///```
    ///layerstack_rasterize()
    ///```
    ///adds layer data to framebuffer

    fn layerstack_rasterize(&mut self, buffer: &mut FrameBuffer) {
        for id in self.sequence.clone() {
            let layer = self.layer_fetch_mut(&id);
            if layer.is_rendered {
                buffer.write_layer(&layer);
            }
        }
    }
    ///returns a mutable Layer from the layerstack
    ///# Parameters
    /// - `id` : requested layerID
    ///
    /// # Example
    /// ```
    /// foreground : Layer = layer_fetch(2);
    /// ```
    ///foreground becomes a mutable reference to the layer with id 2

    fn layer_fetch_mut(&mut self, id: &LayerID) -> &mut Layer {
        if let Some(layer) = self.stack.get_mut(id) {
            layer
        } else {
            panic!("[error] layer {} does not exist", id);
        }
    }
    /// Returns an immutable Layer from the layerstack
    fn layer_fetch(&self, id: &LayerID) -> &Layer {
        if let Some(layer) = self.stack.get(id) {
            layer
        } else {
            panic!("[error] layer {} does not exist", id);
        }
    }
    fn rebuild_sequence(&mut self) {
        self.sequence = vec![];
        for position in 0..self.stack.len() {
            for (id, layer) in self.stack.iter() {
                if layer.stack_pos as usize == position {
                    self.sequence.push(*id);
                }
            }
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

    pub fn layer_add(&mut self, layer_id: LayerID, pos: u16) {
        println!("will create a new layer entry in the render queue");
        if self.stack.contains_key(&layer_id) {
            println!("error! layer already exists");
        }

        let new_layer = Layer {
            buffer: vec![],
            stack_pos: pos,
            is_rendered: true,
        };

        self.layer_shift(pos, ShiftDirection::Up);
        self.stack.insert(layer_id, new_layer);
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
    /// ```
    /// Moves layer with ID 1 to position 0.

    pub fn layer_move(&mut self, layer_id: LayerID, new_pos: u16) {
        println!("changes layer queue sequence");
        let old_pos = (self.layer_fetch(&layer_id)).stack_pos;
        self.layer_shift(new_pos, ShiftDirection::Up);
        let target_layer = self.layer_fetch_mut(&layer_id);
        target_layer.stack_pos = new_pos;
        self.layer_shift(old_pos, ShiftDirection::Down);
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
    pub fn layer_remove(&mut self, layer_id: LayerID) {
        println!("will remove the specified layer");
        let void_pos = (self.layer_fetch(&layer_id)).stack_pos;
        self.stack.remove(&layer_id);
        self.layer_shift(void_pos, ShiftDirection::Down);
    }
    ///Toggles layer visibilty
    ///# Parameters
    ///- `layer_id` : target layer
    ///- `isvisible` : boolean to decide whether to include layer in rasterization
    ///# Example
    ///```
    ///layer_set_visibility(1,false);
    ///```
    ///`layer_id` 1 is not included in rasterization
    pub fn layer_set_visibility(&mut self, layer_id: LayerID, isvisible: bool) {
        let layer = self.layer_fetch_mut(&layer_id);
        layer.is_rendered = isvisible;
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
        self.sequence_rebuild = true;
        for layer in self.stack.values_mut() {
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

    ///writes a Sprite's SpriteVector to the target layer
    ///# Parameters
    ///- `x` : target worldspace x position of center of Sprite
    ///- `y` : target worldspace y position of center of Sprite
    ///- `sprite` : sprite template to write to screen
    ///- `layer-id` : target layer
    ///# Example
    ///```
    ///layer_write_sprite(10,15,Dino,1);
    ///```
    ///writes the `dino` Sprite to (10,15) on layer 1.

    pub fn layer_write_sprite(&mut self, x: u16, y: u16, sprite: Sprite, layer_id: LayerID) -> () {
        println!("will write the sprite vector to the specified layer");
        let worldspace_spritevector = self.framebuffer.to_worldspace(x, y, &sprite);
        let layer = self.layer_fetch_mut(&layer_id);
        layer.buffer.push(worldspace_spritevector);
    }

    ///directly writes a pixel to the target layer
    ///# Parameters
    ///- `x` : target worldspace x position.
    ///- `y` : target worldspace y position.
    ///- `color` : color of pixel
    ///- `layer_id` : target layer
    ///# Example
    ///```
    ///layer_direct_write(10,15,Color::Green,1);
    ///```
    ///directly writes a green pixel to (10,15) of layer 1.
    pub fn layer_direct_write(&mut self, x: u16, y: u16, color: Color, layer_id: LayerID) {
        let layer = self.layer_fetch_mut(&layer_id);
        let new_pixel: Pixel = Pixel {
            x: x,
            y: y,
            color: color,
            isrendered: true,
        };
        layer.buffer.push(vec![new_pixel]);
    }
}
enum ShiftDirection {
    Up,
    Down,
}
///Renderer API
impl Renderer {
    ///returns a new instance of the Renderer
    ///# Parameters
    ///- `width` : length of row (x aspect)
    ///- `height` : number of rows (y aspect)
    ///- `bg_color` : color of background
    ///# Example
    ///```
    ///let engine : Renderer = Renderer::new(50,50,Color::Black);
    ///```
    ///engine is now an instance of Renderer with a size of 50x50px
    pub fn new(width: u16, height: u16, bg_color: Color) -> Self {
        Renderer {
            layerstack: Layerstack::new(width, height, bg_color),
            //layerstack_sequence: vec![],
            //layerstack_sequence_rebuild: true,
            stdout: std::io::stdout(),
            framerate: time::Duration::from_millis(40),
            debug: false,
        }
    }
    /// Clears terminal display
    /// analogous to POSIX `clear` and DOS `cls`
    fn clear() {
        print!("\x1b[2J\x1b[H");
    }
    ///sets framerate interval in milliseconds,
    ///default is 25fps (40ms)
    ///# Parameters
    ///- `new_framerate` : frame display duration in milliseconds
    ///# Examples
    ///```
    /// set_framerate(100);
    ///```
    ///sets framerate to 10fps (100ms)
    ///
    ///```
    ///set_framerate(40);
    ///```
    ///sets framerate to 25fps (40ms)
    ///
    ///```
    ///set_framerate(16);
    ///```
    ///sets framerate to 60fps (16ms)

    pub fn set_framerate(&mut self, new_framerate: u64) {
        self.framerate = time::Duration::from_millis(new_framerate);
    }

    /// updates display by rasterizing layers then pushes framebuffer to the display,
    /// then sleeps for the framerate interval
    /// # Example:
    /// ```
    /// render_update();
    /// ```

    pub fn render_update(&mut self) {
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
        //push_render(self.buffer.buffer.clone());
        //pushes render
        self.render_push();
        self.stdout.flush().unwrap();
        thread::sleep(self.framerate);
    }
    /// pushes framebuffer to Display
    /// by drawing all pixels in the framebuffer to the Display
    /// # Example
    /// ```
    /// render_push();
    /// ```
    /// displays framebuffer
    fn render_push(&mut self) {
        for (i, pixel) in self.layerstack.framebuffer.buffer.iter().enumerate() {
            if pixel.isrendered {
                Renderer::draw_pixel(pixel.x, pixel.y, pixel.color, &mut self.stdout, i as i16)
                    .unwrap();
            }
        }
    }
    fn draw_pixel(x: u16, y: u16, color: Color, stdout: &mut Stdout, i: i16) -> io::Result<()> {
        //let element = format!("|{} ", i);
        queue!(
            stdout,
            cursor::MoveTo(x * 2, y),
            style::PrintStyledContent(PIXEL_ELEMENT.with(color))
        )?;

        Ok(())
    }
    ///Enables debug logging
    ///writes status updates to renderer.log
    ///# Parameters
    ///- `toggle` : boolean to turn on/off logging
    ///# Example
    ///```
    /// debug_mode(true);
    ///```
    ///enables debug logging
    pub fn debug_mode(&mut self, toggle: bool) {
        println!("toggles debug logging");
    }
}
