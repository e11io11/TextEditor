use sdl2::{rect::Rect, render::Canvas, video::Window};

use super::{RepositionFun, ResizeFun, VueComponent, SCROLL_BAR_COLOR, TEXT_COLOR};

pub(crate) struct ScrollBar {
    area: Rect,
    pub resize_fun: ResizeFun,
    pub reposition_fun: RepositionFun,
    vertical: bool,
    scroll_percent: f32,
    shown_percent: f32,
}

impl ScrollBar {
    pub fn new(resize_fun: ResizeFun, reposition_fun: RepositionFun, vertical: bool) -> Self {
        ScrollBar {
            area: Rect::new(0, 0, 0, 0),
            resize_fun,
            reposition_fun,
            vertical,
            scroll_percent: 0.0,
            shown_percent: 0.0,
        }
    }

    fn get_bar_area(&self) -> Rect {
        let (area_x, area_y) = (self.area.x, self.area.y);
        let (area_w, area_h) = self.area.size();
        let (w, h) = if self.vertical {
            (
                area_w,
                (((area_h as f32 * self.shown_percent) / 100.0) as u32).min(area_h),
            )
        } else {
            (
                (((area_w as f32 * self.shown_percent) / 100.0) as u32).min(area_w),
                area_h,
            )
        };
        let (x, y) = if self.vertical {
            (
                area_x,
                area_y + ((((area_h - h) as f32 * self.scroll_percent) / 100.0) as i32),
            )
        } else {
            (
                area_x + ((((area_w - w) as f32 * self.scroll_percent) / 100.0) as i32),
                area_y,
            )
        };

        Rect::new(x, y, w, h)
    }

    pub fn _debug_draw_rect(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(TEXT_COLOR);
        canvas.draw_rect(self.area).unwrap();
        let mut bar_area = self.get_bar_area();
        bar_area.resize((bar_area.w - 4) as u32, (bar_area.h - 4) as u32);
        bar_area.offset(2, 2);
        canvas.draw_rect(bar_area).unwrap();
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let bar_area = self.get_bar_area();
        let rect = {
            let mut rect = bar_area.clone();
            if self.vertical {
                rect.resize(rect.width() / 2, rect.height() - 10);
            } else {
                rect.resize(rect.width() - 10, rect.height() / 2);
            }
            rect.centered_on(bar_area.center())
        };
        canvas.set_draw_color(SCROLL_BAR_COLOR);
        canvas.fill_rect(rect)?;
        Ok(())
    }

    pub fn refresh(
        &mut self,
        scroll_percent: f32,
        shown_percent: f32,
        canvas: &mut Canvas<Window>,
    ) -> Result<(), String> {
        self.scroll_percent = scroll_percent;
        self.shown_percent = shown_percent;
        canvas.set_clip_rect(self.area);
        if shown_percent < 100.0 {
            self.draw(canvas)?;
        }
        Ok(())
    }

    pub fn click_scroll(&self, x: i32, y: i32) -> Option<f32> {
        let bar_area = self.get_bar_area();
        if !self.area.contains_point((x, y)) || bar_area.contains_point((x, y)) {
            None
        } else {
            let (bar_w, bar_h) = bar_area.size();
            let (area_w, area_h) = self.area.size();
            let (bar_x, bar_y) = self.bar_centered_on(x, y);
            if self.vertical {
                let percent = (bar_y as f32 / (area_h - bar_h) as f32) * 100.0;
                Some(percent)
            } else {
                let percent = (bar_x as f32 / (area_w - bar_w) as f32) * 100.0;
                Some(percent)
            }
        }
    }

    fn bar_centered_on(&self, x: i32, y: i32) -> (i32, i32) {
        let bar_area = self.get_bar_area();
        let (bar_w, bar_h) = bar_area.size();
        let (area_x, area_y) = (self.area.x, self.area.y);
        let (area_w, area_h) = self.area.size();
        let bar_y = (y - bar_h as i32 / 2).clamp(area_y, area_y + area_h as i32 - bar_h as i32);
        let bar_x = (x - bar_w as i32 / 2).clamp(area_x, area_x + area_w as i32 - bar_w as i32);
        (bar_x, bar_y)
    }

    pub fn hold_scroll(
        &self,
        origin: (i32, i32),
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32,
    ) -> Option<f32> {
        let bar_area = self.get_bar_area();
        let (old_mouse_x, old_mouse_y) = (x - xrel, y - yrel);
        if !self.area.contains_point(origin) {
            None
        } else {
            let (bar_w, bar_h) = bar_area.size();
            let (area_w, area_h) = self.area.size();
            let (area_x, area_y) = (self.area.x, self.area.y);
            let (dist_from_center_x, dist_from_center_y) = {
                let center = bar_area.center();
                (center.x - old_mouse_x, center.y - old_mouse_y)
            };
            let (bar_x, bar_y) =
                self.bar_centered_on(x + dist_from_center_x, y + dist_from_center_y);
            if self.vertical {
                let percent = ((bar_y - area_y) as f32 / (area_h - bar_h) as f32) * 100.0;
                Some(percent)
            } else {
                let percent = ((bar_x - area_x) as f32 / (area_w - bar_w) as f32) * 100.0;
                Some(percent)
            }
        }
    }
}

impl VueComponent for ScrollBar {
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
