use macroquad::prelude::*;
use macroquad::window::miniquad::{BlendFactor, BlendState, BlendValue, Equation};
use std::cell::RefCell;

// gpu resources built once on first use; macroquad is single-threaded
thread_local! {
    static BACKGROUND: RefCell<Option<Texture2D>> = RefCell::new(None);
    static SHADOW: RefCell<Option<Texture2D>> = RefCell::new(None);
    static AURA: RefCell<Option<Material>> = RefCell::new(None);
}

const BG_W: u16 = 480;
const BG_H: u16 = 270;

const SHADOW_TEX: f32 = 128.0;
const SHADOW_BLUR: f32 = 14.0;
const SHADOW_CORNER: f32 = 10.0;
const SHADOW_ALPHA: f32 = 0.5;
const SHADOW_DROP: f32 = 4.0;

pub fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Board backdrop: brightest at the divider, enemy half warm, own half cool, corner vignette.
pub fn draw_background(w: f32, h: f32) {
    let tex = BACKGROUND.with(|c| c.borrow_mut().get_or_insert_with(bake_background).clone());
    draw_texture_ex(&tex, 0.0, 0.0, WHITE, DrawTextureParams {
        dest_size: Some(vec2(w, h)),
        ..Default::default()
    });
}

fn bake_background() -> Texture2D {
    let mut img = Image::gen_image_color(BG_W, BG_H, BLACK);
    for py in 0..BG_H as u32 {
        let v = py as f32 / (BG_H - 1) as f32;
        let mid = 1.0 - (v - 0.5).abs() * 2.0;
        let warm = (0.5 - v).max(0.0) * 2.0;
        let cool = (v - 0.5).max(0.0) * 2.0;
        let base = 0.040 + 0.075 * smoothstep(0.0, 1.0, mid);
        for px in 0..BG_W as u32 {
            let u = px as f32 / (BG_W - 1) as f32;
            let dx = (u - 0.5) * 2.0;
            let dy = (v - 0.5) * 2.0;
            let vig = 1.0 - 0.38 * smoothstep(0.75, 1.5, (dx * dx + dy * dy).sqrt());
            let r = (base + 0.045 * warm) * vig;
            let g = (base + 0.010 * warm + 0.010 * cool) * vig;
            let b = (base + 0.025 + 0.055 * cool) * vig;
            img.set_pixel(px, py, Color::new(r, g, b, 1.0));
        }
    }
    let tex = Texture2D::from_image(&img);
    tex.set_filter(FilterMode::Linear);
    tex
}

/// Soft drop shadow under an (x, y, w, h) rect; extends past the rect and sits a few px low.
pub fn soft_shadow(x: f32, y: f32, w: f32, h: f32) {
    soft_shadow_tinted(x, y, w, h, WHITE);
}

/// The same shadow under a card that is being painted through a tint, so it fades with the card.
pub fn soft_shadow_tinted(x: f32, y: f32, w: f32, h: f32, tint: Color) {
    let tex = SHADOW.with(|c| c.borrow_mut().get_or_insert_with(bake_shadow).clone());
    // inner opaque region of the texture maps exactly onto the rect
    let grow = SHADOW_TEX / (SHADOW_TEX - 2.0 * SHADOW_BLUR);
    let sw = w * grow;
    let sh = h * grow;
    draw_texture_ex(&tex, x - (sw - w) / 2.0, y - (sh - h) / 2.0 + SHADOW_DROP, tint, DrawTextureParams {
        dest_size: Some(vec2(sw, sh)),
        ..Default::default()
    });
}

fn bake_shadow() -> Texture2D {
    let n = SHADOW_TEX as u16;
    let mut img = Image::gen_image_color(n, n, Color::new(0.0, 0.0, 0.0, 0.0));
    let c = SHADOW_TEX / 2.0;
    let he = c - SHADOW_BLUR - SHADOW_CORNER;
    for py in 0..n as u32 {
        for px in 0..n as u32 {
            // signed distance to the inner rounded rect, quadratic falloff over the blur band
            let qx = (px as f32 + 0.5 - c).abs() - he;
            let qy = (py as f32 + 0.5 - c).abs() - he;
            let d = vec2(qx.max(0.0), qy.max(0.0)).length() + qx.max(qy).min(0.0) - SHADOW_CORNER;
            let a = 1.0 - (d / SHADOW_BLUR).clamp(0.0, 1.0);
            img.set_pixel(px, py, Color::new(0.0, 0.0, 0.0, a * a * SHADOW_ALPHA));
        }
    }
    let tex = Texture2D::from_image(&img);
    tex.set_filter(FilterMode::Linear);
    tex
}

/// A band of light hugging a card rect and falling off outward.
pub struct Aura {
    /// gap between the card edge and where the band starts
    pub inset: f32,
    /// px the band fades out over
    pub spread: f32,
    /// alpha exponent across the band, higher fades to nothing sooner
    pub falloff: f32,
    /// corner radius in px, to follow the card frame art
    pub corner: f32,
    /// rgb at the inner edge of the band
    pub inner: Color,
    /// rgb at the outer edge, graded into across the band
    pub outer: Color,
    /// brightness at the inner edge
    pub alpha: f32,
}

/// One additive quad over the card and its margin, so the band is a true falloff and not a stack
/// of rings. The sdf is in the quad's own space, so a rotated card carries its aura round with it.
pub fn draw_aura(r: Rect, aura: &Aura) {
    let mat = AURA.with(|c| c.borrow_mut().get_or_insert_with(bake_aura).clone());
    let pad = aura.inset + aura.spread;
    let half = Vec2::new(r.w, r.h) / 2.0;
    mat.set_uniform("card_half", half);
    mat.set_uniform("quad_half", half + Vec2::splat(pad));
    mat.set_uniform("inset", aura.inset);
    // a zero spread divides by zero in the shader, a zero falloff is pow(0, 0)
    mat.set_uniform("spread", aura.spread.max(0.01));
    mat.set_uniform("falloff", aura.falloff.max(0.01));
    mat.set_uniform("corner", aura.corner);
    mat.set_uniform("inner_color", Vec3::new(aura.inner.r, aura.inner.g, aura.inner.b));
    mat.set_uniform("outer_color", Vec3::new(aura.outer.r, aura.outer.g, aura.outer.b));
    mat.set_uniform("alpha", aura.alpha);

    gl_use_material(&mat);
    draw_rectangle(r.x - pad, r.y - pad, r.w + 2.0 * pad, r.h + 2.0 * pad, WHITE);
    gl_use_default_material();
}

fn bake_aura() -> Material {
    load_material(
        ShaderSource::Glsl { vertex: AURA_VERTEX, fragment: AURA_FRAGMENT },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("card_half", UniformType::Float2),
                UniformDesc::new("quad_half", UniformType::Float2),
                UniformDesc::new("inset", UniformType::Float1),
                UniformDesc::new("spread", UniformType::Float1),
                UniformDesc::new("falloff", UniformType::Float1),
                UniformDesc::new("corner", UniformType::Float1),
                UniformDesc::new("inner_color", UniformType::Float3),
                UniformDesc::new("outer_color", UniformType::Float3),
                UniformDesc::new("alpha", UniformType::Float1),
            ],
            // additive, so the band reads as light thrown on the board rather than paint over it
            pipeline_params: PipelineParams {
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::One,
                )),
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .expect("aura shader")
}

// glsl es 100 throughout: the web build runs on webgl1, which takes nothing newer
const AURA_VERTEX: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;

varying mediump vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
}"#;

const AURA_FRAGMENT: &str = r#"#version 100
precision mediump float;

varying vec2 uv;

uniform vec2 card_half;
uniform vec2 quad_half;
uniform float inset;
uniform float spread;
uniform float falloff;
uniform float corner;
uniform vec3 inner_color;
uniform vec3 outer_color;
uniform float alpha;

void main() {
    vec2 p = (uv - 0.5) * 2.0 * quad_half;
    vec2 q = abs(p) - card_half + corner;
    float d = length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - corner;
    float band = clamp((d - inset) / spread, 0.0, 1.0);
    // feathered, else the brightest edge of the band aliases hard along the card
    float a = pow(1.0 - band, falloff) * smoothstep(0.0, 1.5, d - inset);
    // graded on sqrt, so the inner color does not win the whole visible part of the band
    gl_FragColor = vec4(mix(inner_color, outer_color, sqrt(band)), alpha * a);
}"#;

/// Non-overlapping pieces, so translucent colors blend once everywhere.
pub fn fill_round_rect(x: f32, y: f32, w: f32, h: f32, r: f32, color: Color) {
    let r = r.min(w / 2.0).min(h / 2.0);
    draw_rectangle(x + r, y, w - 2.0 * r, h, color);
    draw_rectangle(x, y + r, r, h - 2.0 * r, color);
    draw_rectangle(x + w - r, y + r, r, h - 2.0 * r, color);
    draw_arc(x + r, y + r, 24, 0.0, 180.0, r, 90.0, color);
    draw_arc(x + w - r, y + r, 24, 0.0, 270.0, r, 90.0, color);
    draw_arc(x + w - r, y + h - r, 24, 0.0, 0.0, r, 90.0, color);
    draw_arc(x + r, y + h - r, 24, 0.0, 90.0, r, 90.0, color);
}

pub fn stroke_round_rect(x: f32, y: f32, w: f32, h: f32, r: f32, t: f32, color: Color) {
    let r = r.min(w / 2.0).min(h / 2.0);
    draw_line(x + r, y, x + w - r, y, t, color);
    draw_line(x + r, y + h, x + w - r, y + h, t, color);
    draw_line(x, y + r, x, y + h - r, t, color);
    draw_line(x + w, y + r, x + w, y + h - r, t, color);
    // draw_arc bands outward from its radius, lines center on theirs
    let ar = (r - t / 2.0).max(0.0);
    draw_arc(x + r, y + r, 24, ar, 180.0, t, 90.0, color);
    draw_arc(x + w - r, y + r, 24, ar, 270.0, t, 90.0, color);
    draw_arc(x + w - r, y + h - r, 24, ar, 0.0, t, 90.0, color);
    draw_arc(x + r, y + h - r, 24, ar, 90.0, t, 90.0, color);
}
