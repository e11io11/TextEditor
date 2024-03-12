extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::ttf::{self};
use std::path::Path;
use text_zone::TextContent;
use vue::TextBox;

mod save_load;
mod text_zone;
mod vue;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Text Editor", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let ttf_context = ttf::init().unwrap();
    let canvas = window.into_canvas().build().unwrap();
    let font = ttf_context
        .load_font(Path::new("assets/arial_monospaced_mt.ttf"), 24)
        .unwrap();
    let mut text_box = TextBox::new(canvas, font);
    let mut text_content = TextContent::new();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    video_subsystem.text_input().start();
    'running: loop {
        let events = event_pump.poll_iter();
        for event in events {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => text_content.new_line(),
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => text_content.move_cursor_up(1),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => text_content.move_cursor_down(1),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => text_content.move_cursor_left(1),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => text_content.move_cursor_right(1),
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => text_content.remove(),
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    keymod: Mod::LCTRLMOD | Mod::RCTRLMOD,
                    ..
                } => {
                    save_load::save(&&text_content.get_string()).unwrap();
                }
                Event::TextInput { text, .. } => text_content.append(text),
                Event::TextEditing {
                    text,
                    start,
                    length,
                    ..
                } => {
                    println!("Text editing: {}, {}, {}", text, start, length);
                    text_content.append(text);
                }
                _ => {}
            }
            println!("{:?}", text_content.get_cursor());
            println!("{:?}", text_content.get_text());
            text_box.refresh(text_content.get_text(), text_content.get_cursor());
        }
    }
}
