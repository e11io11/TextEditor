use sdl2::{pixels::Color, rect::Rect, render::Canvas, ttf::Font, video::Window};

pub(crate) struct TextBox<'a> {
    canvas: Canvas<Window>,
    font: Font<'a, 'a>,
}

impl<'a> TextBox<'a> {
    pub fn new(canvas: Canvas<Window>, font: Font<'a, 'a>) -> Self {
        TextBox { canvas, font }
    }

    pub fn draw_content(&mut self, content: &Vec<String>) {
        for (line, text) in content.iter().enumerate() {
            if text.is_empty() {
                continue;
            }
            let surface = self
                .font
                .render(text)
                .solid(Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                })
                .unwrap();
            let binding = self.canvas.texture_creator();
            let texture = surface.as_texture(&binding).unwrap();
            let rect = text_rect(&self.font, text, line.try_into().unwrap());
            self.canvas.copy(&texture, None, rect).unwrap()
        }
    }

    pub fn draw_cursor(&mut self, cursor: (usize, usize)) {
        let (l, c) = cursor;
        let (w, h) = self.font.size_of_char('a').unwrap();
        let (x, y) = {
            let x = (w * c as u32) as i32;
            let y = (h * l as u32) as i32;
            (x, y)
        };
        let rect = Rect::new(x, y, w, h);
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.draw_rect(rect).unwrap();
    }

    pub fn refresh(&mut self, content: Vec<String>, cursor: (usize, usize)) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.draw_content(&content);
        self.draw_cursor(cursor);
        self.canvas.present();
    }
}

fn text_rect(font: &Font, text: &str, line: usize) -> Rect {
    let (width, height) = font.size_of(text).unwrap();
    Rect::new(0, (height * line as u32) as i32, width, height)
}
