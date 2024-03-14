use sdl2::{rect::Rect, render::Canvas, ttf::Font, video::Window};

use crate::timer;

use super::{BACKGROUND_COLOR, TEXT_COLOR};
const X_MARGIN: i32 = 10;
const Y_MARGIN: i32 = 10;

pub(crate) struct TextArea<'a> {
    /* canvas: Canvas<Window>, */
    area: Rect,
    font: Font<'a, 'a>,
    char_size: (u32, u32),
    cursor_timer: timer::Timer,
}

impl<'a> TextArea<'a> {
    pub fn new(font: Font<'a, 'a>) -> Self {
        let char_size = font.size_of_char('a').unwrap();
        TextArea {
            area: Rect::new(0, 0, 0, 0),
            font,
            char_size,
            cursor_timer: timer::Timer::new(),
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.area = Rect::new(x, y, self.area.width(), self.area.height());
    }

    pub fn set_size(&mut self, w: u32, h: u32) {
        self.area = Rect::new(self.area.x(), self.area.y(), w, h);
    }

    pub fn get_draw_area(&self) -> Rect {
        let (w, h) = self.area.size();
        let (x, y) = (self.area.x(), self.area.y());
        Rect::new(
            x + X_MARGIN,
            y + Y_MARGIN,
            w - X_MARGIN as u32,
            h - Y_MARGIN as u32,
        )
    }

    pub fn _debug_draw_rect(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(TEXT_COLOR);
        canvas.draw_rect(self.area).unwrap();
        canvas.draw_rect(self.get_draw_area()).unwrap();
    }

    pub fn draw_content(&mut self, content: &Vec<String>, canvas: &mut Canvas<Window>) {
        let creator = canvas.texture_creator();
        for (line, text) in content.iter().enumerate() {
            if text.is_empty() {
                continue;
            }
            let surface = self.font.render(text).blended(TEXT_COLOR).unwrap();
            let texture = surface.as_texture(&creator).unwrap();
            let rect = {
                let area = self.get_draw_area();
                let mut rect = text_rect(&self.font, text, line);
                rect.offset(area.x(), area.y());
                rect
            };
            canvas.copy(&texture, None, rect).unwrap()
        }
    }

    pub fn draw_cursor(&mut self, cursor: (usize, usize), canvas: &mut Canvas<Window>) {
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
            let mut r = Rect::new(x - (w / 2) as i32, y, w, h);
            let area = self.get_draw_area();
            r.offset(area.x(), area.y());
            r
        };
        canvas.set_draw_color(TEXT_COLOR);
        canvas.draw_rect(rect).unwrap();
    }

    pub fn reset_cursor_timer(&mut self) {
        self.cursor_timer = timer::Timer::new();
    }

    pub fn refresh(
        &mut self,
        content: Vec<String>,
        cursor: (usize, usize),
        canvas: &mut Canvas<Window>,
    ) {
        canvas.set_clip_rect(self.area);
        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();
        self.draw_content(&content, canvas);
        self.draw_cursor(cursor, canvas);
    }

    pub fn index_of_position(&self, x: i32, y: i32) -> (usize, usize) {
        let (w, h) = self.char_size;
        let area = self.get_draw_area();
        let l = ((y - area.y) / h as i32) as usize;
        let c = ((x - area.x) / w as i32) as usize;
        (l, c)
    }
}

fn text_rect(font: &Font, text: &str, line: usize) -> Rect {
    let (width, height) = font.size_of(text).unwrap();
    Rect::new(0, (height * line as u32) as i32, width, height)
}
