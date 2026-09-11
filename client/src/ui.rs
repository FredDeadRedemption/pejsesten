use macroquad::prelude::*;

// virtual canvas height, matches window_conf so the 720p design keeps its proportions
pub const REF_H: f32 = 720.0;

/// Canvas size: height is fixed, width follows the window aspect so nothing stretches.
pub fn size() -> Vec2 {
    vec2(REF_H * screen_width() / screen_height(), REF_H)
}

// macroquad flips y on the screen pass, so a positive zoom.y is what gives y-down screen coords
fn camera() -> Camera2D {
    let s = size();
    Camera2D {
        target: s / 2.0,
        zoom: vec2(2.0 / s.x, 2.0 / s.y),
        ..Default::default()
    }
}

pub fn set_ui_camera() {
    set_camera(&camera());
}

pub fn mouse_ui() -> Vec2 {
    let (mx, my) = mouse_position();
    camera().screen_to_world(vec2(mx, my))
}

pub fn text(s: &str, x: f32, y: f32, size: f32, color: Color) -> TextDimensions {
    let (font_size, font_scale, _) = camera_font_scale(size);
    draw_text_ex(
        s,
        x,
        y,
        TextParams { font_size, font_scale, color, ..Default::default() },
    )
}

// aspect term from camera_font_scale is always 1 here since the canvas keeps square pixels
pub fn measure(s: &str, size: f32) -> TextDimensions {
    let (font_size, font_scale, _) = camera_font_scale(size);
    measure_text(s, None, font_size, font_scale)
}
