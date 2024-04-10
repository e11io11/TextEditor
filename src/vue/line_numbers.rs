use sdl2::{rect::Rect, render::Canvas, ttf::Font, video::Window};

use super::{
    str_rect_at_line, text_area_container::TOP_MARGIN, RepositionFun, ResizeFun, VueComponent,
    VueError, BACKGROUND_COLOR, GREY_TEXT_COLOR,
};

pub(crate) struct LineNumbers {
    area: Rect,
    pub resize_fun: ResizeFun,
    pub reposition_fun: RepositionFun,
}

impl LineNumbers {
    pub fn new(resize_fun: ResizeFun, reposition_fun: RepositionFun) -> Self {
        LineNumbers {
            area: Rect::new(0, 0, 0, 0),
            resize_fun,
            reposition_fun,
        }
    }

    fn draw(
        &mut self,
        line_n: usize,
        scroll_offset: f32,
        canvas: &mut Canvas<Window>,
        font: &Font,
        content_font: &Font,
    ) -> Result<(), VueError> {
        let creator = canvas.texture_creator();
        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.fill_rect(self.area)?;
        for n in 0..line_n {
            let text = (n + 1).to_string();
            let surface = font.render(&text).blended(GREY_TEXT_COLOR)?;
            let texture = surface.as_texture(&creator)?;
            let rect = {
                let mut rect = str_rect_at_line(font, &text, n)?;
                let container = str_rect_at_line(content_font, &text, n)?;
                rect.center_on(container.center());
                rect.offset(
                    self.area.right() - rect.w - 10,
                    self.area.y() + TOP_MARGIN as i32 + scroll_offset as i32,
                );
                rect
            };
            canvas.copy(&texture, None, rect)?;
        }
        Ok(())
    }

    pub fn refresh(
        &mut self,
        line_n: usize,
        scroll_offset: f32,
        canvas: &mut Canvas<Window>,
        font: &Font,
        content_font: &Font,
    ) -> Result<(), VueError> {
        canvas.set_clip_rect(self.area);
        self.draw(line_n, scroll_offset, canvas, font, content_font)?;
        Ok(())
    }
}

impl VueComponent for LineNumbers {
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
