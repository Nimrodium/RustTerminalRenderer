//sprite.rs
use crossterm::style::Color;
use regex::Regex;
use std::collections::HashMap;

pub type x_pos = i16;
pub type y_pos = i16;

///struct to represent one pixel
#[derive(Clone)]
pub struct Pixel {
    ///x coordinate
    pub x: x_pos,
    /// y coordinate
    pub y: y_pos,
    ///// might be deprecated
    //pub layer: u16,
    /// color of pixel
    pub color: Color,
    ///flag stating wether to render the pixel
    pub isrendered: bool,
}
///A human readable string represenation of a sprite
pub type SpriteSource = &'static str;

//░
//▒
//▓
///defines a compiled sprite
pub struct Sprite {
    /// Vector of all pixels held within the sprite in local spritespace
    pub pixels: Vec<Pixel>,
    /// height of sprite in local spritespace
    pub height: u16,
    /// width of sprite in local spritespace
    pub width: u16,
    /// calculated center of sprite as a tuple (x,y)
    pub center: (u16, u16),
    ///tag is for grouping together sprites
    pub tag: Option<String>,
}
/// metadata used in tandem with a SpriteSource to properly compile into a Sprite
pub struct Metadata {
    ///mapping of characters in sprite_source into associated colors
    pub color_map: HashMap<char, Color>,
    /// character for transparent pixels
    pub transparent: char,
    /// height of sprite (y)
    pub height: u16,
    /// width of sprite (x)
    pub width: u16,
    /// tag for sprite
    pub tag: Option<String>,
}
/// Compiles a Sprite from Metadata and SpriteSource
/// # Parameters
/// - `sprite_source` : human readable string visualization of sprite
/// - `metadata` : compiliation information
/// # Example
/// ```
/// let compiled_sprite : Sprite = compile_sprite(source,metadata);
/// ```
/// a new Sprite instance is created from source and metadata
pub fn compile_sprite(sprite_source: SpriteSource, metadata: Metadata) -> Result<Sprite, String> {
    //sanitizes char array
    let mut valid_characters: String = metadata.color_map.keys().collect();
    valid_characters.push(metadata.transparent);
    let pattern = Regex::new(&format!("[^{}]", valid_characters)).unwrap();
    let sprite_array = pattern.replace_all(sprite_source, "").to_string();

    //verifies aspect ratio is valid
    let expected_length = (metadata.height * metadata.width) as usize;
    let real_length = sprite_array.chars().count();
    if expected_length != real_length {
        let why = format!(
            "defined aspect ratio did not align with whitelisted array size\nexpected {}\nreal :{}\nSprite may be missing a color_map definition",
            expected_length, real_length
        );

        return Err(why.to_string());
    }

    //whats being returned
    let mut compiled_sprite: Sprite = Sprite {
        pixels: vec![],
        height: metadata.height,
        width: metadata.width,
        center: (metadata.width / 2, metadata.height / 2),
        tag: None,
    };
    let mut y = 0;
    let mut x = 0;
    for pixel in sprite_array.chars() {
        x += 1;
        print!("{}", pixel);
        let isrendered_bool;
        let pixel_color;
        if pixel == metadata.transparent {
            isrendered_bool = false;
            pixel_color = Color::Black;
        } else {
            isrendered_bool = true;
            pixel_color = *metadata.color_map.get(&pixel).unwrap();
        };

        //define new pixel object
        let generated_pixel: Pixel = Pixel {
            x: x as x_pos,
            y: y as y_pos,
            //layer: 1,
            color: pixel_color,
            isrendered: isrendered_bool,
        };

        compiled_sprite.pixels.push(generated_pixel);

        if x == metadata.width {
            //at end of width
            y += 1;
            x = 0;
        }
    }
    Ok(compiled_sprite)
}
