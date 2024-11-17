//main.rs

//rust rewrite of terminal 2d rendering engine in python.
//also going to try to do multiple files for this one

//main rs -- init game
//renderpipe.rs -- graphics stack logic
//pixel.rs -- pixel logic?
//game.rs -- chrome dino game or really anything, doesnt matter ig.
//input.rs -- handling real input that seems to be nearly impossible in python+wayland
//https://stackoverflow.com/questions/35671985/how-do-i-get-keyboard-input-without-the-user-pressing-the-enter-key

//possible libs to use?
//crossterm
mod sprite;
mod stack;
use crate::sprite::{compile_sprite, Metadata, Sprite, Spritearray};
use std::collections::HashMap;

use crossterm::{execute, style::Color, terminal};
use stack::push_render;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    println!("main");
    let mut stdout = io::stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    let smiley_sprite: Spritearray = "
        ░░▓▓░
        ▓░░░▓
        ░░░▓▓
        ▓░░░▓
        ░░▓▓░
    ";

    let smily_metadata: Metadata = Metadata {
        color_map: HashMap::from([('░', Color::Blue), ('▓', Color::White)]),
        height: 5,
        width: 5,
        tag: None,
    };
    //stdout.flush().unwrap();
    let compiled_sprite: Sprite = match compile_sprite(smiley_sprite, smily_metadata, 1) {
        Err(why) => panic!("aspect ratio error {}", why),
        Ok(sprite) => sprite,
    };
    push_render(compiled_sprite.pixels);
    Ok(())
}
