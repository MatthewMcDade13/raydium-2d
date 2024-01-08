use anyhow::anyhow;
use sdl2::{
    pixels::PixelFormatEnum,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use nalgebra_glm as glm;

use crate::gfx;

pub struct RaycastRenderer {
    canvas: Canvas<Window>,
    surface_size: glm::U32Vec2,
    target: Texture,
    texture_creator: TextureCreator<WindowContext>,

    pixels: Vec<glm::U8Vec3>,
}

impl RaycastRenderer {
    pub fn new(window: Window) -> anyhow::Result<Self> {
        let (width, height) = window.size();
        let canvas = window
            .into_canvas()
            .accelerated()
            .software()
            .target_texture()
            .build()?;

        let surface_size = glm::vec2(width, height);

        let texture_creator = canvas.texture_creator();
        let target =
            texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, width, height)?;

        let size = (width * height) as usize;
        let pixels = Vec::with_capacity(size);

        let s = Self {
            canvas,
            surface_size,
            texture_creator,
            target,
            pixels,
        };
        Ok(s)
    }

    pub fn draw_map(
        &mut self,
        map: &[u8],
        cam_pos: glm::Vec2,
        face_dir: glm::Vec2,
    ) -> anyhow::Result<()> {
        todo!()
    }

    pub fn clear(&mut self, clear_color: impl Into<Option<gfx::Color>>) {
        let color = match clear_color.into() {
            Some(c) => c,
            None => gfx::Color::black(),
        };
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }
}
