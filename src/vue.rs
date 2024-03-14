use sdl2::{pixels::Color, render::Canvas, ttf::Font, video::Window};

use self::text_area::TextArea;

mod text_area;

const BACKGROUND_COLOR: Color = Color::RGB(31, 31, 31);
const TEXT_COLOR: Color = Color::RGB(204, 204, 204);

pub(crate) struct Vue<'a> {
    canvas: Canvas<Window>,
    pub text_area: TextArea<'a>,
}

impl<'a> Vue<'a> {
    pub fn new(canvas: Canvas<Window>, font: Font<'a, 'a>) -> Self {
        let text_area = TextArea::new(font);
        let mut v = Vue { canvas, text_area };
        v.resize();
        v
    }

    pub fn resize(&mut self) {
        self.canvas.set_draw_color(BACKGROUND_COLOR);
        self.canvas.clear();
        let (w, h) = percent_as_size(self.canvas.window().size(), (80, 90));
        self.text_area.set_size(w, h);
        let (x, y) = percent_as_position(self.canvas.window().size(), (10, 5));
        self.text_area.set_position(x, y);
    }

    pub fn refresh(&mut self, content: Vec<String>, cursor: (usize, usize)) {
        self.text_area.refresh(content, cursor, &mut self.canvas);
        self.canvas.present();
    }
}
pub fn percent_as_size(size: (u32, u32), percent: (u32, u32)) -> (u32, u32) {
    ((size.0 * percent.0) / 100, (size.1 * percent.1) / 100)
}

pub fn percent_as_position(size: (u32, u32), percent: (u32, u32)) -> (i32, i32) {
    (
        ((size.0 * percent.0) / 100) as i32,
        ((size.1 * percent.1) / 100) as i32,
    )
}
