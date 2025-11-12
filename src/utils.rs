use raylib::ffi::ColorFromHSV;
use raylib::prelude::Color;

pub fn color_from_hsv(h: f32, s: f32, v: f32) -> Color {
    unsafe {
        let color_raw = ColorFromHSV(h, s, v);
        let color = Color {
            r: color_raw.r,
            g: color_raw.g,
            b: color_raw.b,
            a: color_raw.a,
        };
        return color;
    }
}
