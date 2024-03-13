use sdl2::{pixels::Color, rect::Rect, render::Canvas, ttf::Font, video::Window};

use crate::timer;

const BACKGROUND_COLOR: Color = Color::RGB(31, 31, 31);
const TEXT_COLOR: Color = Color::RGB(204, 204, 204);

pub(crate) struct TextBox<'a> {
    canvas: Canvas<Window>,
    font: Font<'a, 'a>,
    char_size: (u32, u32),
    cursor_timer: timer::Timer,
}

impl<'a> TextBox<'a> {
    pub fn new(canvas: Canvas<Window>, font: Font<'a, 'a>) -> Self {
        let char_size = font.size_of_char('a').unwrap();
        TextBox {
            canvas,
            font,
            char_size,
            cursor_timer: timer::Timer::new(),
        }
    }

    pub fn draw_content(&mut self, content: &Vec<String>) {
        for (line, text) in content.iter().enumerate() {
            if text.is_empty() {
                continue;
            }
            let surface = self.font.render(text).blended(TEXT_COLOR).unwrap();
            let binding = self.canvas.texture_creator();
            let texture = surface.as_texture(&binding).unwrap();
            let rect = text_rect(&self.font, text, line.try_into().unwrap());
            self.canvas.copy(&texture, None, rect).unwrap()
        }
    }

    pub fn draw_cursor(&mut self, cursor: (usize, usize)) {
        let blink = self.cursor_timer.switch_every_n_millis(2000);
        if blink {
            return;
        }
        let (l, c) = cursor;
        let (w, h) = self.char_size;
        let (x, y) = {
            let x = (w * c as u32) as i32;
            let y = (h * l as u32) as i32;
            (x, y)
        };
        let rect = {
            let w = w / 4;
            Rect::new(x - (w / 2) as i32, y, w, h)
        };
        self.canvas.set_draw_color(TEXT_COLOR);
        self.canvas.draw_rect(rect).unwrap();
    }

    pub fn reset_cursor_timer(&mut self) {
        self.cursor_timer = timer::Timer::new();
    }

    pub fn refresh(&mut self, content: Vec<String>, cursor: (usize, usize)) {
        self.canvas.set_draw_color(BACKGROUND_COLOR);
        self.canvas.clear();
        self.draw_content(&content);
        self.draw_cursor(cursor);
        self.canvas.present();
    }

    pub fn index_of_position(&self, x: i32, y: i32) -> (usize, usize) {
        let (w, h) = self.char_size;
        let l = (y / h as i32) as usize;
        let c = (x / w as i32) as usize;
        (l, c)
    }
}

fn text_rect(font: &Font, text: &str, line: usize) -> Rect {
    let (width, height) = font.size_of(text).unwrap();
    Rect::new(0, (height * line as u32) as i32, width, height)
}
