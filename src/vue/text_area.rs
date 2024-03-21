use sdl2::{rect::Rect, render::Canvas, ttf::Font, video::Window};

use crate::{timer, vue::percent_length};

use super::{char_size, percent_position, str_rect, BACKGROUND_COLOR, GREY_TEXT_COLOR, TEXT_COLOR};
const X_MARGIN: i32 = 40;
const Y_MARGIN: i32 = 10;

pub(crate) struct TextArea {
    area: Rect,
    pub resize_fun: fn((u32, u32)) -> (u32, u32),
    pub reposition_fun: fn((u32, u32)) -> (i32, i32),
    content_size: (u32, u32),
    scroll_offset: (i32, i32),
    cursor_update: bool,
    cursor_timer: timer::Timer,
}

impl TextArea {
    pub fn new(
        resize_fun: fn((u32, u32)) -> (u32, u32),
        reposition_fun: fn((u32, u32)) -> (i32, i32),
    ) -> Self {
        TextArea {
            area: Rect::new(0, 0, 0, 0),
            resize_fun,
            reposition_fun,
            content_size: (0, 0),
            scroll_offset: (0, 0),
            cursor_update: false,
            cursor_timer: timer::Timer::new(),
        }
    }

    pub fn set_position(&mut self, pos: (i32, i32)) {
        let (x, y) = pos;
        self.area = Rect::new(x, y, self.area.width(), self.area.height());
    }

    pub fn set_size(&mut self, size: (u32, u32)) {
        let (w, h) = size;
        self.area = Rect::new(self.area.x(), self.area.y(), w, h);
    }

    fn get_content_area(&self) -> Rect {
        let (w, h) = self.area.size();
        let (x, y) = (self.area.x(), self.area.y());
        let (x_scroll, y_scroll) = self.scroll_offset;
        Rect::new(
            x + X_MARGIN + x_scroll,
            y + Y_MARGIN + y_scroll,
            (w as i32 - X_MARGIN - x_scroll) as u32,
            (h as i32 - Y_MARGIN - y_scroll) as u32,
        )
    }

    fn get_line_number_area(&self) -> Rect {
        let (_, h) = self.area.size();
        let (x, y) = (self.area.x(), self.area.y());
        let (_, y_scroll) = self.scroll_offset;
        Rect::new(
            x,
            y + Y_MARGIN + y_scroll,
            X_MARGIN as u32,
            (h as i32 - Y_MARGIN - y_scroll) as u32,
        )
    }

    pub fn scroll_x(&mut self, x: i32) {
        let bounds = self.cursor_bounds();
        let min = bounds.w - self.content_size.0 as i32;
        if min > 0 {
            return;
        }
        self.scroll_offset.0 = (x + self.scroll_offset.0).clamp(min, 0);
    }

    pub fn scroll_y(&mut self, y: i32) {
        let bounds = self.cursor_bounds();
        let min = bounds.h - self.content_size.1 as i32;
        if min > 0 {
            return;
        }
        self.scroll_offset.1 = (y + self.scroll_offset.1).clamp(min, 0);
    }

    pub fn cursor_scroll(&mut self, cursor: (usize, usize), font: &Font) {
        let bounds = self.cursor_bounds();
        let (x, y) = self.cursor_position(cursor, font);
        match x {
            x if x < bounds.x => {
                self.scroll_offset.0 = (self.scroll_offset.0 + bounds.x - x).min(0)
            }
            x if x > bounds.x + bounds.w => self.scroll_offset.0 += bounds.x + bounds.w - x,
            _ => {}
        };
        match y {
            y if y < bounds.y => {
                self.scroll_offset.1 = (self.scroll_offset.1 + bounds.y - y).min(0)
            }
            y if y > bounds.y + bounds.h => self.scroll_offset.1 += bounds.y + bounds.h - y,
            _ => {}
        };
    }

    fn cursor_bounds(&self) -> Rect {
        let bounds = {
            let rec = self
                .get_content_area()
                .intersection(self.area)
                .expect("Should always intersect");
            let new_w = percent_length(rec.w as u32, 80);
            let new_h = percent_length(rec.h as u32, 60);
            let new_x = rec.x + percent_position(rec.w as u32, 10);
            let new_y = rec.y + percent_position(rec.h as u32, 20);
            Rect::new(new_x, new_y, new_w, new_h)
        };
        bounds
    }

    pub fn _debug_draw_rect(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(TEXT_COLOR);
        canvas.draw_rect(self.area).unwrap();
        canvas.draw_rect(self.get_content_area()).unwrap();
    }

    fn draw_content(&mut self, content: &Vec<String>, canvas: &mut Canvas<Window>, font: &Font) {
        let creator = canvas.texture_creator();
        for (line, text) in content.iter().enumerate() {
            if text.is_empty() {
                continue;
            }
            let surface = font.render(text).blended(TEXT_COLOR).unwrap();
            let texture = surface.as_texture(&creator).unwrap();
            let rect = {
                let area = self.get_content_area();
                let mut rect = text_rect(font, text, line);
                rect.offset(area.x(), area.y());
                rect
            };
            canvas.copy(&texture, None, rect).unwrap()
        }
    }

    fn draw_line_numbers(
        &mut self,
        line_n: usize,
        canvas: &mut Canvas<Window>,
        font: &Font,
        content_font: &Font,
    ) {
        let creator = canvas.texture_creator();
        let area = self.get_line_number_area();
        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.fill_rect(area).unwrap();
        for n in 0..line_n {
            let text = (n + 1).to_string();
            let surface = font.render(&text).blended(GREY_TEXT_COLOR).unwrap();
            let texture = surface.as_texture(&creator).unwrap();
            let rect = {
                let mut rect = text_rect(font, &text, n);
                let container = text_rect(content_font, &text, n);
                rect.center_on(container.center());
                rect.offset(area.right() - rect.w - 10, area.y());
                rect
            };
            canvas.copy(&texture, None, rect).unwrap()
        }
    }

    fn cursor_position(&self, cursor: (usize, usize), font: &Font) -> (i32, i32) {
        let (l, c) = cursor;
        let (w, h) = char_size(font);
        let draw_area = self.get_content_area();
        let (x, y) = {
            let x = (w * c as u32) as i32;
            let y = (h * l as u32) as i32;
            (x + draw_area.x, y + draw_area.y)
        };
        (x, y)
    }

    fn draw_cursor(&mut self, cursor: (usize, usize), canvas: &mut Canvas<Window>, font: &Font) {
        let blink = self.cursor_timer.switch_every_n_millis(2000);
        if blink {
            return;
        }
        let (x, y) = self.cursor_position(cursor, font);
        let rect = {
            let (w, h) = char_size(font);
            let w = w / 4;
            Rect::new(x - (w / 2) as i32, y, w, h)
        };
        canvas.set_draw_color(TEXT_COLOR);
        canvas.draw_rect(rect).unwrap();
    }

    fn reset_cursor_timer(&mut self) {
        self.cursor_timer = timer::Timer::new();
    }

    pub fn send_cursor_update(&mut self) {
        self.cursor_update = true;
    }

    fn on_cursor_update(&mut self, cursor: (usize, usize), font: &Font) {
        self.reset_cursor_timer();
        self.cursor_scroll(cursor, font);
        self.cursor_update = false;
    }

    fn update_content_size(&mut self, content_size: (usize, usize), font: &Font) {
        self.content_size = {
            let (l, c) = content_size;
            let (w, h) = char_size(font);
            (c as u32 * w, l as u32 * h)
        };
    }

    pub fn refresh(
        &mut self,
        content: Vec<String>,
        content_size: (usize, usize),
        cursor: (usize, usize),
        canvas: &mut Canvas<Window>,
        content_font: &Font,
        line_number_font: &Font,
    ) {
        self.update_content_size(content_size, content_font);
        if self.cursor_update {
            self.on_cursor_update(cursor, content_font);
        }
        canvas.set_clip_rect(self.area);
        self.draw_content(&content, canvas, content_font);
        self.draw_line_numbers(content.len(), canvas, line_number_font, content_font);
        self.draw_cursor(cursor, canvas, content_font);
    }

    pub fn index_of_position(&self, x: i32, y: i32, font: &Font) -> (usize, usize) {
        let (w, h) = char_size(font);
        let area = self.get_content_area();
        let l = ((y - area.y) / h as i32) as usize;
        let c = ((x - area.x) / w as i32) as usize;
        (l, c)
    }
}

fn text_rect(font: &Font, text: &str, line: usize) -> Rect {
    let (_, height) = font.size_of(text).unwrap();
    let rect = str_rect(font, text);
    rect.bottom_shifted(height as i32 * line as i32)
}
