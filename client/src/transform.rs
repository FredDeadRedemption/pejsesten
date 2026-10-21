//! How a card is placed and painted for one frame.
//!
//! Layout decides where a card's slot is and nothing here moves it. An animation produces a
//! [`Transform`], the drawing applies it, and no effect has to reach into the drawing itself.

use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Transform {
    /// Pixels away from the slot the layout gave the card.
    pub offset: Vec2,
    /// Around the card's own centre; x alone is a card turning over.
    pub scale: Vec2,
    /// Radians, around the card's own centre.
    pub rotation: f32,
    /// Multiplied into every colour the card paints, alpha included.
    pub tint: Color,
    /// White laid over the finished card, the one thing a multiplied tint cannot do.
    pub flash: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Transform {
    pub const IDENTITY: Self = Self {
        offset: Vec2::ZERO,
        scale: Vec2::ONE,
        rotation: 0.0,
        tint: WHITE,
        flash: 0.0,
    };

    /// The rect the card covers once placed. A scale eats into the slot from both edges.
    pub fn place(&self, slot: Rect) -> Rect {
        let w = (slot.w * self.scale.x).max(1.0);
        let h = (slot.h * self.scale.y).max(1.0);
        Rect::new(
            slot.x + self.offset.x + (slot.w - w) / 2.0,
            slot.y + self.offset.y + (slot.h - h) / 2.0,
            w,
            h,
        )
    }

    /// Places the slot and holds the rotation while `draw` paints inside the rect it is handed.
    pub fn apply(&self, slot: Rect, draw: impl FnOnce(Rect)) {
        let r = self.place(slot);
        let pivot = Vec2::new(r.x + r.w / 2.0, r.y + r.h / 2.0);
        with_rotation(pivot, self.rotation, || draw(r));
    }
}

/// `color` as `tint` paints it.
pub fn shade(color: Color, tint: Color) -> Color {
    Color::new(color.r * tint.r, color.g * tint.g, color.b * tint.b, color.a * tint.a)
}

/// macroquad has no 2d transform of its own, so the gl model matrix is the way to tilt a card.
fn with_rotation(pivot: Vec2, angle: f32, draw: impl FnOnce()) {
    if angle == 0.0 {
        draw();
        return;
    }
    let m = Mat4::from_translation(pivot.extend(0.0))
        * Mat4::from_rotation_z(angle)
        * Mat4::from_translation(-pivot.extend(0.0));
    unsafe { get_internal_gl() }.quad_gl.push_model_matrix(m);
    draw();
    unsafe { get_internal_gl() }.quad_gl.pop_model_matrix();
}
