use macroquad::prelude::*;

#[cfg(target_arch = "wasm32")]
const MEDIA_ROOT: &str = "/media/";
#[cfg(not(target_arch = "wasm32"))]
const MEDIA_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/media/");

pub struct Card3D {
    texture_front: Texture2D,
    texture_back: Texture2D,
    position: Vec3,
    rotation_y: f32,
    target_rotation_y: f32,
    size: Vec2,
    rotation_speed: f32,
}

impl Card3D {
    pub async fn load() -> Self {
        let front_path = format!("{}{}", MEDIA_ROOT, "Assassin.png");
        let back_path = format!("{}{}", MEDIA_ROOT, "cards/card-bg-purple.png");
        let texture_front = load_texture(&front_path).await.unwrap();
        let texture_back = load_texture(&back_path).await.unwrap();
        texture_front.set_filter(FilterMode::Linear);
        texture_back.set_filter(FilterMode::Linear);

        Self {
            texture_front,
            texture_back,
            position: Vec3::ZERO,
            rotation_y: 0.0,
            target_rotation_y: 0.0,
            size: Vec2::new(180.0, 250.0),
            rotation_speed: 8.0,
        }
    }

    pub fn flip(&mut self) {
        self.target_rotation_y += std::f32::consts::PI;
    }

    pub fn update(&mut self, dt: f32) {
        let diff = self.target_rotation_y - self.rotation_y;
        self.rotation_y += diff * self.rotation_speed * dt;
    }

    pub fn draw(&self) {
        set_camera(&Camera3D {
            position: vec3(0.0, 0.0, 500.0),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0.0, 0.0, 0.0),
            ..Default::default()
        });

        let cos_y = self.rotation_y.cos();
        let sin_y = self.rotation_y.sin();
        let show_front = cos_y > 0.0;

        let texture = if show_front {
            &self.texture_front
        } else {
            &self.texture_back
        };

        let e1 = Vec3::new(self.size.x * cos_y, 0.0, -self.size.x * sin_y);
        let e2 = Vec3::new(0.0, self.size.y, 0.0);
        let offset = self.position - e1 / 2.0 - e2 / 2.0;

        if show_front {
            draw_affine_parallelogram(offset, e1, e2, Some(texture), WHITE);
        } else {
            draw_affine_parallelogram(offset + e1, -e1, e2, Some(texture), WHITE);
        }

        set_default_camera();
    }
}
