use sdl2::{
    rect::{Point, Rect},
    render::Canvas,
    ttf::Font,
    video::Window,
};

use crate::{
    timer::{self, Timer},
    vue::percent_length,
};

use super::{
    char_size, percent_position, str_rect_at_line, text_area_container::TOP_MARGIN, RepositionFun,
    ResizeFun, VueComponent, VueError, TEXT_COLOR,
};

pub(crate) struct TextArea {
    area: Rect,
    pub resize_fun: ResizeFun,
    pub reposition_fun: RepositionFun,
    content_size: (usize, usize),
    content_font_size: (u32, u32),
    scroll_offset: (f32, f32),
    cursor_update: bool,
    cursor_timer: Timer,
}

impl TextArea {
    pub fn new(resize_fun: ResizeFun, reposition_fun: RepositionFun) -> Self {
        TextArea {
            area: Rect::new(0, 0, 0, 0),
            resize_fun,
            reposition_fun,
            content_size: (0, 0),
            content_font_size: (0, 0),
            scroll_offset: (0.0, 0.0),
            cursor_update: false,
            cursor_timer: Timer::new(),
        }
    }

    fn get_content_area(&self) -> Rect {
        let (w, h) = {
            let (l, c) = self.content_size;
            let (w, h) = self.content_font_size;
            (w * c as u32, h * l as u32)
        };
        let (x, y) = {
            let (x, y) = (self.area.x(), self.area.y());
            let (x_scroll, y_scroll) = self.scroll_offset;
            (x + x_scroll as i32, y + y_scroll as i32 + TOP_MARGIN as i32)
        };
        Rect::new(x, y, w, h)
    }

    pub fn get_scroll_offset(&self) -> (f32, f32) {
        self.scroll_offset
    }

    pub fn get_scroll_percent(&self) -> (f32, f32) {
        let (w, h) = {
            let (w1, h1) = self.get_scrollable_area();
            let (w2, h2) = self.area.size();
            (w1 - w2.min(w1), h1 - h2.min(h1))
        };
        let (x, y) = self.scroll_offset;
        (
            ((-x * 100.0) / w as f32).max(0.0),
            ((-y * 100.0) / h as f32).max(0.0),
        )
    }

    pub fn set_y_scroll_percent(&mut self, y: f32) {
        let (_, h) = {
            let (w1, h1) = self.get_scrollable_area();
            let (w2, h2) = self.area.size();
            (w1 - w2.min(w1), h1 - h2.min(h1))
        };
        self.scroll_offset.1 = (h as f32 * -y / 100.0).clamp(-(h as f32), 0.0);
    }

    pub fn set_x_scroll_percent(&mut self, x: f32) {
        let (w, _) = {
            let (w1, h1) = self.get_scrollable_area();
            let (w2, h2) = self.area.size();
            (w1 - w2.min(w1), h1 - h2.min(h1))
        };
        self.scroll_offset.0 = (w as f32 * -x / 100.0).clamp(-(w as f32), 0.0);
    }

    pub fn get_shown_percent(&self) -> (f32, f32) {
        let (w, h) = self.get_scrollable_area();
        let (w1, h1) = self.area.size();
        (w1 as f32 * 100.0 / w as f32, h1 as f32 * 100.0 / h as f32)
    }

    pub fn get_scrollable_area(&self) -> (u32, u32) {
        let (w1, h1) = self.get_content_area().size();
        let (w2, h2) = self.area.size();
        (
            w1 + percent_length(w2, 90),
            h1 + TOP_MARGIN + percent_length(h2, 80),
        )
    }

    pub fn scroll_x(&mut self, x: f32) {
        let (w, _) = self.get_scrollable_area();
        let min = -(w as i32) + self.area.w as i32;
        if min > 0 {
            return;
        }
        self.scroll_offset.0 = (x + self.scroll_offset.0).clamp(min as f32, 0.0);
    }

    pub fn scroll_y(&mut self, y: f32) {
        let (_, h) = self.get_scrollable_area();
        let min = -(h as i32) + self.area.h as i32;
        if min > 0 {
            return;
        }
        self.scroll_offset.1 = (y + self.scroll_offset.1).clamp(min as f32, 0.0);
    }

    pub fn cursor_scroll(&mut self, cursor: (usize, usize)) {
        let bounds = self.cursor_bounds();
        let (x, y) = self.cursor_position(cursor);
        match x {
            x if x < bounds.x => {
                self.scroll_offset.0 = (self.scroll_offset.0 + bounds.x as f32 - x as f32).min(0.0)
            }
            x if x > bounds.x + bounds.w => {
                self.scroll_offset.0 += (bounds.x + bounds.w - x) as f32
            }
            _ => {}
        };
        match y {
            y if y < bounds.y => {
                self.scroll_offset.1 = (self.scroll_offset.1 + bounds.y as f32 - y as f32).min(0.0)
            }
            y if y > bounds.y + bounds.h => {
                self.scroll_offset.1 += (bounds.y + bounds.h - y) as f32
            }
            _ => {}
        };
    }

    fn cursor_bounds(&self) -> Rect {
        let bounds = {
            let rec = self.area;
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

    fn draw_content(
        &mut self,
        content: &Vec<String>,
        canvas: &mut Canvas<Window>,
        font: &Font,
    ) -> Result<(), VueError> {
        let creator = canvas.texture_creator();
        for (line, text) in content.iter().enumerate() {
            if text.is_empty() {
                continue;
            }
            let surface = font.render(text).blended(TEXT_COLOR)?;
            let texture = surface.as_texture(&creator)?;
            let rect = {
                let area = self.get_content_area();
                let mut rect = str_rect_at_line(font, text, line)?;
                rect.offset(area.x(), area.y());
                rect
            };
            canvas.copy(&texture, None, rect)?;
        }
        Ok(())
    }

    fn cursor_position(&self, cursor: (usize, usize)) -> (i32, i32) {
        let (l, c) = cursor;
        let (w, h) = self.content_font_size;
        let draw_area = self.get_content_area();
        let (x, y) = {
            let x = (w * c as u32) as i32;
            let y = (h * l as u32) as i32;
            (x + draw_area.x, y + draw_area.y)
        };
        (x, y)
    }

    fn draw_cursor(
        &mut self,
        cursor: (usize, usize),
        canvas: &mut Canvas<Window>,
    ) -> Result<(), String> {
        let blink = self.cursor_timer.switch_every_n_millis(2000);
        if blink {
            return Ok(());
        }
        let (x, y) = self.cursor_position(cursor);
        let rect = {
            let (w, h) = self.content_font_size;
            let w = w / 4;
            Rect::new(x - (w / 2) as i32, y, w, h)
        };
        canvas.set_draw_color(TEXT_COLOR);
        canvas.draw_rect(rect)?;
        Ok(())
    }

    fn reset_cursor_timer(&mut self) {
        self.cursor_timer = timer::Timer::new();
    }

    pub fn send_cursor_update(&mut self) {
        self.cursor_update = true;
    }

    fn on_cursor_update(&mut self, cursor: (usize, usize)) {
        self.reset_cursor_timer();
        self.cursor_scroll(cursor);
        self.cursor_update = false;
    }

    pub fn refresh(
        &mut self,
        content: Vec<String>,
        content_size: (usize, usize),
        cursor: (usize, usize),
        canvas: &mut Canvas<Window>,
        content_font: &Font,
    ) -> Result<(), VueError> {
        self.content_font_size = char_size(content_font);
        self.content_size = content_size;
        if self.cursor_update {
            self.on_cursor_update(cursor);
        }
        self.draw_content(&content, canvas, content_font)?;
        self.draw_cursor(cursor, canvas)?;
        Ok(())
    }

    pub fn index_of_position(&self, x: i32, y: i32) -> Option<(usize, usize)> {
        if self.area.contains_point(Point::new(x, y)) {
            let (w, h) = self.content_font_size;
            let area = self.get_content_area();
            let l = ((y - area.y) / h as i32) as usize;
            let c = ((x - area.x) / w as i32) as usize;
            Some((l, c))
        } else {
            None
        }
    }
}

impl VueComponent for TextArea {
    fn set_position(&mut self, pos: (i32, i32)) {
        let (x, y) = pos;
        self.area = Rect::new(x, y, self.area.width(), self.area.height());
    }

    fn set_size(&mut self, size: (u32, u32)) {
        let (w, h) = size;
        self.area = Rect::new(self.area.x(), self.area.y(), w, h);
    }

    fn get_reposition_fun(&self) -> RepositionFun {
        self.reposition_fun
    }

    fn get_resize_fun(&self) -> ResizeFun {
        self.resize_fun
    }
}
