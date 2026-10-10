use macroquad::prelude::*;

// virtual canvas height, matches window_conf so the 720p design keeps its proportions
pub const REF_H: f32 = 720.0;

// a phone with its rotation locked still gets a landscape board; turn the phone clockwise to read it
const PORTRAIT_SPIN: f32 = -90.0;

fn portrait() -> bool {
    screen_height() > screen_width()
}

/// Canvas size: height is fixed, width follows the window's long edge so nothing stretches.
pub fn size() -> Vec2 {
    let (long, short) = match portrait() {
        true => (screen_height(), screen_width()),
        false => (screen_width(), screen_height()),
    };
    vec2(REF_H * long / short, REF_H)
}

// macroquad flips y on the screen pass, so a positive zoom.y is what gives y-down screen coords
fn camera_for(canvas: Vec2, portrait: bool) -> Camera2D {
    // the spin sends the canvas x axis down the screen, so the zoom axes swap with it
    let (zoom, rotation) = match portrait {
        true => (vec2(2.0 / canvas.y, 2.0 / canvas.x), PORTRAIT_SPIN),
        false => (vec2(2.0 / canvas.x, 2.0 / canvas.y), 0.0),
    };
    Camera2D { target: canvas / 2.0, zoom, rotation, ..Default::default() }
}

fn camera() -> Camera2D {
    camera_for(size(), portrait())
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

#[cfg(test)]
mod tests {
    use super::*;

    const CANVAS: Vec2 = Vec2::new(1280.0, 720.0);

    fn ndc(cam: &Camera2D, point: Vec2) -> Vec2 {
        let v = cam.matrix().transform_point3(vec3(point.x, point.y, 0.0));
        vec2(v.x, v.y)
    }

    fn at(cam: &Camera2D, point: Vec2, expected: Vec2) {
        let got = ndc(cam, point);
        assert!((got - expected).length() < 1e-4, "{point} landed at {got}, expected {expected}");
    }

    #[test]
    fn landscape_keeps_the_canvas_origin_top_left() {
        let cam = camera_for(CANVAS, false);
        at(&cam, Vec2::ZERO, vec2(-1.0, 1.0));
        at(&cam, CANVAS, vec2(1.0, -1.0));
    }

    /// The canvas top edge must run up the left of the screen, so turning the phone clockwise reads it.
    #[test]
    fn portrait_lays_the_canvas_along_the_long_edge() {
        let cam = camera_for(CANVAS, true);
        at(&cam, Vec2::ZERO, vec2(-1.0, -1.0));
        at(&cam, vec2(CANVAS.x, 0.0), vec2(-1.0, 1.0));
        at(&cam, CANVAS, vec2(1.0, 1.0));
        at(&cam, vec2(0.0, CANVAS.y), vec2(1.0, -1.0));
    }
}
