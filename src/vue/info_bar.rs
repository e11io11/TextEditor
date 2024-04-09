use sdl2::{rect::Rect, render::Canvas, ttf::Font, video::Window};

use super::{
    char_size, str_rect, RepositionFun, ResizeFun, VueComponent, VueError, BAR_COLOR,
    OUTLINE_COLOR, TEXT_COLOR,
};

pub(crate) struct InfoBar {
    area: Rect,
    pub resize_fun: ResizeFun,
    pub reposition_fun: RepositionFun,
}

impl InfoBar {
    pub fn new(resize_fun: ResizeFun, reposition_fun: RepositionFun) -> Self {
        InfoBar {
            area: Rect::new(0, 0, 0, 0),
            resize_fun,
            reposition_fun,
        }
    }

    pub fn draw_bar(
        &self,
        cursor: (usize, usize),
        canvas: &mut Canvas<Window>,
        font: &Font,
    ) -> Result<(), VueError> {
        canvas.set_draw_color(BAR_COLOR);
        canvas.fill_rect(self.area)?;
        canvas.set_draw_color(OUTLINE_COLOR);
        canvas.draw_rect(self.area)?;
        let (l, c) = {
            let (l, c) = cursor;
            (l + 1, c + 1)
        };
        let cursor_str = format!("Ln {}, Col {}", l, c);
        let surface = font.render(&cursor_str).blended(TEXT_COLOR)?;
        let creator = canvas.texture_creator();
        let texture = surface.as_texture(&creator)?;
        let rect = {
            let area = self.area;
            let str_rect = str_rect(font, &cursor_str)?;
            let mut rect = str_rect.centered_on(area.center());
            rect.set_x(area.width() as i32 - str_rect.width() as i32 - char_size(font).0 as i32);
            rect
        };
        canvas.copy(&texture, None, rect)?;
        Ok(())
    }

    pub fn refresh(
        &self,
        cursor: (usize, usize),
        canvas: &mut Canvas<Window>,
        font: &Font,
    ) -> Result<(), VueError> {
        canvas.set_clip_rect(self.area);
        self.draw_bar(cursor, canvas, font)?;
        Ok(())
    }
}

impl VueComponent for InfoBar {
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
