//main.rs

//rust rewrite of terminal 2d rendering engine in python.
//also going to try to do multiple files for this one

//main -- entrypoint
//stack -- renderstack logic for actually writing to frames
//sprite -- handling of sprite actions
//game -- loadable game module
//https://stackoverflow.com/questions/35671985/how-do-i-get-keyboard-input-without-the-user-pressing-the-enter-key

//possible libs to use?
//crossterm
mod sprite;
mod stack;
use crate::sprite::{compile_sprite, Metadata, Sprite, SpriteSource};
use std::collections::HashMap;
use std::{thread, time};

use crossterm::{execute, style::Color, terminal};
use stack::Renderer;
use std::io::{self, Write};

fn main() {
    println!("main");

    let smiley_sprite: SpriteSource = "
        ░░▓▓░
        ▓░░░▓
        ░░░▓▓
        ▓░░░▓
        ░░▓▓░
    ";

    let smily_metadata: Metadata = Metadata {
        color_map: HashMap::from([('░', Color::Black), ('▓', Color::Magenta)]),
        transparent: 'T',
        height: 5,
        width: 5,
        tag: None,
    };

    let second_sprite: SpriteSource = "

        ░▓▓▓░
        ░░░▓░
        ░░░▓░
        ░░░▓░

        ";

    let second_sprite_metadata: Metadata = Metadata {
        color_map: HashMap::from([('░', Color::Green), ('▓', Color::Blue)]),
        transparent: 'T',
        height: 4,
        width: 5,
        tag: None,
    };
    //stdout.flush().unwrap();
    let compiled_sprite: Sprite = match compile_sprite(smiley_sprite, smily_metadata, 1) {
        Err(why) => panic!("aspect ratio error {}", why),
        Ok(sprite) => sprite,
    };

    let second_comp_sprite: Sprite = match compile_sprite(second_sprite, second_sprite_metadata, 1)
    {
        Err(why) => panic!("aspect ratio error {}", why),
        Ok(sprite) => sprite,
    };

    let mut renderer = Renderer::new(50, 50, Color::White);
    let background = renderer.layerstack.add(0, 0);
    let foreground = renderer.layerstack.add(1, 1);
    renderer.set_framerate(40);

    for i in 0..=1000 {
        //renderer.layerstack.wipe_buffers();
        renderer.layerstack.write_sprite(i + 1, i + 1, &compiled_sprite, background);
        renderer.layerstack.write_sprite(
            (30 as u16).saturating_sub(i),
            (30 as u16).saturating_sub(i),
            &second_comp_sprite,
            foreground,
        );
        renderer.render_update();
        renderer.clear();
    }
}
