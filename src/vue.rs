use std::{collections::HashMap, ffi::NulError, path::Path};

use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{Canvas, TextureValueError},
    ttf::{Font, FontError, Sdl2TtfContext},
    video::Window,
};

use self::{info_bar::InfoBar, text_area::TextArea};

mod info_bar;
mod text_area;

const BAR_COLOR: Color = Color::RGB(24, 24, 24);
const OUTLINE_COLOR: Color = Color::RGB(43, 43, 43);
const BACKGROUND_COLOR: Color = Color::RGB(31, 31, 31);
const TEXT_COLOR: Color = Color::RGB(204, 204, 204);
const GREY_TEXT_COLOR: Color = Color::RGB(110, 118, 129);

const TEXT_FONT: &str = "__TEXT_FONT__";
const UI_FONT: &str = "__UI_FONT__";

#[derive(Debug)]
pub(crate) enum VueError {
    InvalidLatin1Text(NulError),
    WidthOverflows(u32),
    HeightOverflows(u32),
    WidthMustBeMultipleOfTwoForFormat(u32, PixelFormatEnum),
    SdlError(String),
}

impl From<FontError> for VueError {
    fn from(e: FontError) -> Self {
        match e {
            FontError::InvalidLatin1Text(e) => VueError::InvalidLatin1Text(e),
            FontError::SdlError(s) => VueError::SdlError(s),
        }
    }
}

impl From<TextureValueError> for VueError {
    fn from(e: TextureValueError) -> Self {
        match e {
            TextureValueError::WidthOverflows(w) => VueError::WidthOverflows(w),
            TextureValueError::HeightOverflows(h) => VueError::HeightOverflows(h),
            TextureValueError::WidthMustBeMultipleOfTwoForFormat(w, f) => {
                VueError::WidthMustBeMultipleOfTwoForFormat(w, f)
            }
            TextureValueError::SdlError(s) => VueError::SdlError(s),
        }
    }
}

impl From<String> for VueError {
    fn from(s: String) -> Self {
        VueError::SdlError(s)
    }
}

pub(crate) struct Fonts<'a> {
    map: HashMap<String, Font<'a, 'a>>,
}

impl<'a> Fonts<'a> {
    pub fn new() -> Self {
        Fonts {
            map: HashMap::new(),
        }
    }

    pub fn _add(&mut self, font: Font<'a, 'a>) {
        self.map.insert(font.face_family_name().unwrap(), font);
    }

    pub fn add_with_name(&mut self, font: Font<'a, 'a>, name: String) {
        self.map.insert(name, font);
    }

    pub fn get(&self, name: &str) -> Option<&Font> {
        self.map.get(name)
    }

    pub fn _get_names(&self) -> Vec<String> {
        self.map.keys().map(|s| s.clone()).collect()
    }
}

pub(crate) struct Vue<'a> {
    canvas: Canvas<Window>,
    fonts: Fonts<'a>,
    pub text_area: TextArea,
    info_bar: InfoBar,
}

impl<'a> Vue<'a> {
    pub fn new(canvas: Canvas<Window>, ttf_context: &'a Sdl2TtfContext) -> Self {
        let fonts = {
            let mut f = Fonts::new();
            let font_path = Path::new("assets/droid-sans-mono.regular.ttf");
            f.add_with_name(
                (&ttf_context).load_font(font_path, 16).unwrap(),
                TEXT_FONT.to_string(),
            );
            f.add_with_name(
                (&ttf_context).load_font(font_path, 14).unwrap(),
                UI_FONT.to_string(),
            );
            f
        };
        let text_area = TextArea::new(|(w, h)| (w, h - 30), |(_, _)| (0, 0));
        let info_bar = info_bar::InfoBar::new(|(w, _)| (w, 30), |(_, h)| (0, (h - 30) as i32));
        let mut v = Vue {
            canvas,
            fonts,
            text_area,
            info_bar,
        };
        v.resize();
        v
    }

    pub fn resize(&mut self) {
        self.canvas.set_draw_color(BACKGROUND_COLOR);
        self.canvas.clear();
        let (w, h) = self.canvas.window().size();
        self.text_area.set_size((self.text_area.resize_fun)((w, h)));
        self.text_area
            .set_position((self.text_area.reposition_fun)((w, h)));
        self.info_bar.set_size((self.info_bar.resize_fun)((w, h)));
        self.info_bar
            .set_position((self.info_bar.reposition_fun)((w, h)));
    }

    pub fn cursor_position(&self, x: i32, y: i32) -> (usize, usize) {
        let (l, c) = self
            .text_area
            .index_of_position(x, y, &self.fonts.get(TEXT_FONT).unwrap());
        (l, c)
    }

    pub fn scroll_text_area(&mut self, x: i32, y: i32) {
        self.text_area.scroll_y(20 * y);
        self.text_area.scroll_x(20 * -x);
    }

    pub fn refresh(
        &mut self,
        content: Vec<String>,
        content_size: (usize, usize),
        cursor: (usize, usize),
    ) {
        self.canvas.set_draw_color(BACKGROUND_COLOR);
        self.canvas.clear();
        self.text_area
            .refresh(
                content,
                content_size,
                cursor,
                &mut self.canvas,
                self.fonts.get(TEXT_FONT).unwrap(),
                self.fonts.get(UI_FONT).unwrap(),
            )
            .unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
            });
        self.info_bar
            .refresh(cursor, &mut self.canvas, self.fonts.get(UI_FONT).unwrap())
            .unwrap_or_else(|e| eprintln!("Error: {:?}", e));
        self.canvas.present();
    }
}

pub fn _percent_as_size(size: (u32, u32), percent: (u32, u32)) -> (u32, u32) {
    ((size.0 * percent.0) / 100, (size.1 * percent.1) / 100)
}

pub fn percent_length(length: u32, percent: u32) -> u32 {
    (length * percent) / 100
}

pub fn percent_position(length: u32, percent: u32) -> i32 {
    ((length * percent) / 100) as i32
}

pub fn _percent_as_position(size: (u32, u32), percent: (u32, u32)) -> (i32, i32) {
    (
        ((size.0 * percent.0) / 100) as i32,
        ((size.1 * percent.1) / 100) as i32,
    )
}

pub fn char_size(font: &Font) -> (u32, u32) {
    assert!(
        font.face_is_fixed_width(),
        "This function should only be used with mono fonts"
    );
    font.size_of_char('a').unwrap()
}

pub fn str_rect(font: &Font, text: &str) -> Result<Rect, FontError> {
    let (width, height) = font.size_of(text)?;
    Ok(Rect::new(0, 0, width, height))
}

pub fn _center(x1: u32, x2: u32) -> u32 {
    (x1 + x2) / 2
}
