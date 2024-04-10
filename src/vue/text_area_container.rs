use sdl2::{rect::Rect, render::Canvas, ttf::Font, video::Window};

use super::{
    line_numbers::LineNumbers, scroll_bar::ScrollBar, text_area::TextArea, RepositionFun,
    ResizeFun, VueComponent, VueError,
};

pub(super) const TOP_MARGIN: u32 = 10;

pub(crate) struct TextAreaContainer {
    area: Rect,
    pub resize_fun: ResizeFun,
    pub reposition_fun: RepositionFun,
    text_area: TextArea,
    line_numbers: LineNumbers,
    vertical_scroll_bar: ScrollBar,
    horizontal_scroll_bar: ScrollBar,
}

impl TextAreaContainer {
    pub fn new(resize_fun: ResizeFun, reposition_fun: RepositionFun) -> Self {
        let text_area = TextArea::new(|(w, h)| (w - 40, h - 30), |_, (x, y)| (x + 40, y));
        let line_numbers = LineNumbers::new(|(_, h)| (40, h), |_, (x, y)| (x, y));
        let vertical_scroll_bar = ScrollBar::new(
            |(_, h)| (20, h - TOP_MARGIN),
            |(w, _), (x, y)| (x + (w as i32 - 20), y),
            true,
        );
        let horizontal_scroll_bar = ScrollBar::new(
            |(w, _)| (w - 20, 20),
            |(_, h), (x, y)| (x, y + (h - 20) as i32),
            false,
        );
        TextAreaContainer {
            area: Rect::new(0, 0, 0, 0),
            resize_fun,
            reposition_fun,
            text_area,
            line_numbers,
            vertical_scroll_bar,
            horizontal_scroll_bar,
        }
    }

    pub fn scroll(&mut self, x: f32, y: f32) {
        self.text_area.scroll_y(20.0 * y);
        self.text_area.scroll_x(20.0 * -x);
    }

    pub fn cursor_index(&self, x: i32, y: i32) -> Option<(usize, usize)> {
        self.text_area.index_of_position(x, y)
    }

    pub fn send_cursor_update(&mut self) {
        self.text_area.send_cursor_update();
    }

    pub fn click_scroll_bar(&mut self, x: i32, y: i32) -> bool {
        let b1 = match self.vertical_scroll_bar.click_scroll(x, y) {
            Some(p) => {
                self.text_area.set_y_scroll_percent(p);
                true
            }
            None => false,
        };
        let b2 = match self.horizontal_scroll_bar.click_scroll(x, y) {
            Some(p) => {
                self.text_area.set_x_scroll_percent(p);
                true
            }
            None => false,
        };
        b1 || b2
    }

    pub fn hold_scroll_bar(
        &mut self,
        origin: (i32, i32),
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32,
    ) -> bool {
        let b1 = match self
            .vertical_scroll_bar
            .hold_scroll(origin, x, y, xrel, yrel)
        {
            Some(p) => {
                self.text_area.set_y_scroll_percent(p);
                true
            }
            None => false,
        };
        let b2 = match self
            .horizontal_scroll_bar
            .hold_scroll(origin, x, y, xrel, yrel)
        {
            Some(p) => {
                self.text_area.set_x_scroll_percent(p);
                true
            }
            None => false,
        };
        b1 || b2
    }

    pub fn refresh(
        &mut self,
        content: Vec<String>,
        content_size: (usize, usize),
        cursor: (usize, usize),
        canvas: &mut Canvas<Window>,
        content_font: &Font,
        line_number_font: &Font,
    ) -> Result<(), VueError> {
        canvas.set_clip_rect(self.area);
        self.text_area
            .refresh(content, content_size, cursor, canvas, content_font)?;
        self.line_numbers.refresh(
            content_size.0,
            self.text_area.get_scroll_offset().1,
            canvas,
            line_number_font,
            content_font,
        )?;
        self.vertical_scroll_bar.refresh(
            self.text_area.get_scroll_percent().1,
            self.text_area.get_shown_percent().1,
            canvas,
        )?;
        self.horizontal_scroll_bar.refresh(
            self.text_area.get_scroll_percent().0,
            self.text_area.get_shown_percent().0,
            canvas,
        )?;
        Ok(())
    }
}

impl VueComponent for TextAreaContainer {
    fn set_position(&mut self, pos: (i32, i32)) {
        let (x, y) = pos;
        self.area = Rect::new(x, y, self.area.width(), self.area.height());
    }

    fn set_size(&mut self, size: (u32, u32)) {
        let (w, h) = size;
        self.area = Rect::new(self.area.x(), self.area.y(), w, h);
    }

    fn get_resize_fun(&self) -> ResizeFun {
        self.resize_fun
    }

    fn get_reposition_fun(&self) -> RepositionFun {
        self.reposition_fun
    }

    fn affect_children(&mut self) {
        let size = self.area.size();
        let pos = (self.area.x, self.area.y);
        self.text_area.on_resize(size, pos);
        self.line_numbers.on_resize(size, pos);
        self.vertical_scroll_bar.on_resize(size, pos);
        self.horizontal_scroll_bar.on_resize(size, pos);
    }
}
