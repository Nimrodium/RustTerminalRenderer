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
use crate::sprite::{compile_sprite, Metadata, Sprite, Spritearray};
use std::collections::HashMap;
use std::{thread, time};

use crossterm::{execute, style::Color, terminal};
use stack::{framebuffer_write, init_framebuffer, push_render, FrameBuffer};
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
    for i in 0..100 {
        let mut framebuffer: FrameBuffer = match init_framebuffer(50, 150, Color::White) {
            Err(why) => panic!("framebuffer init failed: {}", why),
            Ok(framebuffer) => framebuffer,
        };
        framebuffer_write(i, 4, 1, &compiled_sprite, &mut framebuffer);
        //framebuffer_write(i + 20, i, 1, &compiled_sprite, &mut framebuffer);

        //framerate
        let frame_duration = time::Duration::from_millis(10);
        thread::sleep(frame_duration);
        execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
        push_render(framebuffer.buffer);
    }
    Ok(())
}
