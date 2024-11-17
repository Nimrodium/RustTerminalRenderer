//sprite.rs
use crossterm::style::Color;
use regex::Regex;
use std::collections::HashMap;

///Sprite Metadata struct needs to define what colors are present in the sprite and the string representation of those colors

///smallest element to which the frame is defined by.
pub struct Pixel {
    pub x: u16,
    pub y: u16,
    pub layer: u16,
    pub color: Color,
    pub object_id: i32,
}

pub type Spritearray = &'static str;

//░
//▒
//▓

pub struct Sprite {
    pub pixels: Vec<Pixel>,
    pub height: u16,
    pub width: u16,
    pub center: (u16, u16),
    ///tag is for grouping together sprites
    pub tag: Option<String>,
}

pub struct Metadata {
    pub color_map: HashMap<char, Color>,
    pub height: u16,
    pub width: u16,
    pub tag: Option<String>,
}
///# compile_sprite
///takes in a Spritearray, Metadata and a sprite ID to compile into a sprite object
pub fn compile_sprite(
    sprite_array: Spritearray,
    metadata: Metadata,
    id: i32,
) -> Result<Sprite, String> {
    //sanitizes char array
    let valid_characters: String = metadata.color_map.keys().collect();
    let pattern = Regex::new(&format!("[^{}]", valid_characters)).unwrap();
    let sprite_array = pattern.replace_all(sprite_array, "").to_string();

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

        //define new pixel object
        let generated_pixel: Pixel = Pixel {
            x: x as u16,
            y: y as u16,
            layer: 1,
            color: *metadata.color_map.get(&pixel).unwrap(),
            object_id: id,
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
