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
use stack::{init_FrameBuffer, push_render, FrameBuffer, FrameBuffer_write};
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
        color_map: HashMap::from([('░', Color::Black), ('▓', Color::Magenta)]),
        height: 5,
        width: 5,
        tag: None,
    };
    //stdout.flush().unwrap();
    let compiled_sprite: Sprite = match compile_sprite(smiley_sprite, smily_metadata, 1) {
        Err(why) => panic!("aspect ratio error {}", why),
        Ok(sprite) => sprite,
    };

    let mut framebuffer: FrameBuffer = match init_FrameBuffer(50, 150, Color::White) {
        Err(why) => panic!("framebuffer init failed: {}", why),
        Ok(framebuffer) => framebuffer,
    };
    FrameBuffer_write(10, 15, 1, compiled_sprite.pixels, &mut framebuffer);
    push_render(framebuffer.framebuffer);
    Ok(())
}
