extern crate sdl2;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::{Keycode, Mod};
use sdl2::ttf::{self};
use text_zone::TextContent;
use timer::Timer;
use vue::Vue;

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
        Keycode::Up if keymod.intersects(Mod::LALTMOD | Mod::RALTMOD) => {
            text_content.move_line_up()
        }
        Keycode::Down if keymod.intersects(Mod::LALTMOD | Mod::RALTMOD) => {
            text_content.move_line_down()
        }
        Keycode::Up => text_content.move_cursor_up(1),
        Keycode::Down => text_content.move_cursor_down(1),
        Keycode::Left => text_content.move_cursor_left(1),
        Keycode::Right => text_content.move_cursor_right(1),
        Keycode::Backspace => text_content.remove(),
        Keycode::Tab => text_content.append("    ".to_string()),
        _ => return false,
    }
    return true;
}

fn command(keycode: Keycode, keymod: Mod, text_content: &mut TextContent) -> bool {
    match keycode {
        Keycode::S if keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD) => {
            save_load::save(&text_content.get_string()).unwrap()
        }
        _ => return false,
    }
    return true;
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = {
        let w = video_subsystem
            .window("Text Editor", 800, 600)
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        w
    };

    let canvas = window.into_canvas().build().unwrap();
    let ttf_context = ttf::init().unwrap();
    let mut vue = Vue::new(canvas, &ttf_context);
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
                        vue.text_area.send_cursor_update()
                    } else if command(keycode, keymod, &mut text_content) {
                    }
                }
                Event::TextInput { text, .. } => {
                    text_content.append(text);
                    vue.text_area.send_cursor_update();
                }
                Event::MouseButtonDown { x, y, .. } => {
                    text_content.set_cursor(vue.cursor_position(x, y));
                    vue.text_area.send_cursor_update();
                }
                Event::Window {
                    win_event: WindowEvent::Resized(..),
                    ..
                } => vue.resize(),
                Event::MouseWheel {
                    precise_x,
                    precise_y,
                    ..
                } => {
                    vue.scroll_text_area(precise_x as i32, precise_y as i32);
                }
                _ => {}
            }
        }
        let refresh = refresh_switch != timer.switch_n_times_per_second(60);
        if refresh {
            vue.refresh(
                text_content.get_text(),
                text_content.size(),
                text_content.get_cursor(),
            );
            refresh_switch = !refresh_switch;
        }
    }
}
