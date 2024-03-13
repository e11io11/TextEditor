extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::ttf::{self};
use std::path::Path;
use text_zone::TextContent;
use timer::Timer;
use vue::TextBox;

mod save_load;
mod text_zone;
mod timer;
mod vue;

fn text_editing(keycode: Keycode, keymod: Mod, text_content: &mut TextContent) -> bool {
    match keycode {
        Keycode::Return if keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD) => {
            text_content.new_line()
        }
        Keycode::Return => text_content.break_line(),
        Keycode::Up => text_content.move_cursor_up(1),
        Keycode::Down => text_content.move_cursor_down(1),
        Keycode::Left => text_content.move_cursor_left(1),
        Keycode::Right => text_content.move_cursor_right(1),
        Keycode::Backspace => text_content.remove(),
        _ => return false,
    }
    return true;
}

fn command(keycode: Keycode, keymod: Mod, text_content: &mut TextContent) -> bool {
    match keycode {
        Keycode::S if keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD) => {
            save_load::save(&&text_content.get_string()).unwrap()
        }
        _ => return false,
    }
    return true;
}

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
        .load_font(Path::new("assets/droid-sans-mono.regular.ttf"), 18)
        .unwrap();
    let mut text_box = TextBox::new(canvas, font);
    let mut text_content = TextContent::new();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    video_subsystem.text_input().start();
    let timer = Timer::new();
    let mut refresh_switch = true;
    'running: loop {
        let events = event_pump.poll_iter();
        for event in events {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    keymod,
                    ..
                } => {
                    if text_editing(keycode, keymod, &mut text_content) {
                        text_box.reset_cursor_timer()
                    } else if command(keycode, keymod, &mut text_content) {
                    }
                }
                Event::TextInput { text, .. } => {
                    text_content.append(text);
                    text_box.reset_cursor_timer();
                }
                Event::MouseButtonDown { x, y, .. } => {
                    text_content.set_cursor(text_box.index_of_position(x, y));
                    text_box.reset_cursor_timer();
                }
                _ => {}
            }
            println!("{:?}", text_content.get_cursor());
            println!("{:?}", text_content.get_text());
        }
        let refresh = refresh_switch != timer.switch_every_n_millis(10);
        if refresh {
            text_box.refresh(text_content.get_text(), text_content.get_cursor());
            refresh_switch = !refresh_switch;
        }
    }
}
