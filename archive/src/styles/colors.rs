use palette::{IntoColor, Okhsv, Srgb};
use ratatui::{prelude::*, widgets::*};

pub struct RgbSwatch;

impl Widget for RgbSwatch {
    fn render(self, area: Rect, buf: &mut Buffer) {
       for(yi, y) in (area.top()..area.bottom()).enumerate() {
            let value = area.height as f32 - yi as f32;
            let value_fg = value / (area.height as f32);
            let value_bg = (value - 0.5) / (area.height as f32);
            for (xi, x) in (area.left()..area.right()).enumerate() {
                let hue = xi as f32 * 360.0 / area.width as f32;
                let fg = color_from_oklab(hue, Okhsv::max_saturation(), value_fg);
                let bg = color_from_oklab(hue, Okhsv::max_saturation(), value_bg);
                buf.get_mut(x, y).set_char(' ').set_fg(fg).set_bg(bg);
            }
       } 
    }
}

pub fn color_from_oklab(hue: f32, saturation: f32, value: f32) -> Color {
    let color: Srgb = Okhsv::new(hue, saturation, value).into_color();
    let color = color.into_format();
    Color::Rgb(color.red, color.green, color.blue)
}
