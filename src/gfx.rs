use nalgebra_glm as glm;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vert2D {
    pub pos: [f32; 4],
    pub color: [f32; 4],
    pub uv: [f32; 2],
}

impl Vert2D {
    pub fn zero() -> Self {
        Self::default()
    }

    pub fn with_pos(pos: glm::Vec2) -> Self {
        Self {
            pos: [pos.x, pos.y, 0., 0.],
            color: [0.0; 4],
            uv: [0.; 2],
        }
    }

    pub fn with_color(color: &glm::Vec4) -> Self {
        Self {
            pos: [0.; 4],
            color: [color.x, color.y, color.z, color.w],
            uv: [0.; 2],
        }
    }

    pub fn with_uv(uv: glm::Vec2) -> Self {
        Self {
            pos: [0.; 4],
            color: [0.0; 4],
            uv: [uv.x, uv.y],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Color(pub glm::Vec4);

impl Color {
    pub fn white() -> Self {
        Self(glm::vec4(1., 1., 1., 1.))
    }
    pub fn red() -> Self {
        Self(glm::vec4(1., 0., 0., 1.))
    }
    pub fn green() -> Self {
        Self(glm::vec4(0., 1., 0., 1.))
    }
    pub fn blue() -> Self {
        Self(glm::vec4(0., 0., 1., 1.))
    }
    pub fn black() -> Self {
        Self(glm::Vec4::zeros())
    }

    pub const WHITE_RAW: [f32; 4] = [1., 1., 1., 1.];
    pub const RED_RAW: [f32; 4] = [1., 0., 0., 1.];
    pub const GREEN_RAW: [f32; 4] = [0., 1., 0., 1.];
    pub const BLUE_RAW: [f32; 4] = [0., 0., 1., 1.];
}

impl Default for Color {
    fn default() -> Self {
        Self::white()
    }
}

impl From<Color> for sdl2::pixels::Color {
    fn from(value: Color) -> Self {
        let c = value.0;
        sdl2::pixels::Color {
            r: (c.x * 255.) as u8,
            g: (c.y * 255.) as u8,
            b: (c.z * 255.) as u8,
            a: (c.w * 255.) as u8,
        }
    }
}

impl From<glm::Vec4> for Color {
    fn from(value: glm::Vec4) -> Self {
        Self(value)
    }
}

impl From<Color> for glm::Vec4 {
    fn from(value: Color) -> Self {
        value.0
    }
}

pub const fn pack_rgb(r: u8, g: u8, b: u8) -> u32 {
    pack_rgba(r, g, b, 255)
}

pub const fn pack_rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    let red = (r as u32) << 24;
    let green = (g as u32) << 16;
    let blue = (b as u32) << 8;
    let alpha = a as u32;
    red | green | blue | alpha
}

pub const fn unpack_rgba(color: u32) -> (u8, u8, u8, u8) {
    let r = (color >> 24) as u8;
    let g = (color >> 16) as u8;
    let b = (color >> 8) as u8;
    let a = color as u8;
    (r, g, b, a)
}

pub const fn unpack_rgb(color: u32) -> (u8, u8, u8) {
    let (r, g, b, _) = unpack_rgba(color);
    (r, g, b)
}

use anyhow::*;
use image::{EncodableLayout, GenericImageView};
use sdl2::{
    pixels::PixelFormatEnum,
    rect::Rect,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};
use wgpu::Extent3d;

pub enum TextureType {
    Diffuse,
    Normal,
}
#[derive(Debug)]
pub struct Texture {
    pub handle: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub size: Extent3d,
}

impl Texture {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        ty: TextureType,
        label: Option<&str>,
    ) -> anyhow::Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, ty, label)
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        ty: TextureType,
        label: Option<&str>,
    ) -> anyhow::Result<Self> {
        let rgba = img.to_rgba8();
        let dims = img.dimensions();

        let size = wgpu::Extent3d {
            width: dims.0,
            height: dims.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: match ty {
                TextureType::Diffuse => wgpu::TextureFormat::Rgba8UnormSrgb,
                TextureType::Normal => wgpu::TextureFormat::Rgba8Unorm,
            },
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dims.0),
                rows_per_image: None,
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

        Ok(Self {
            handle: texture,
            view,
            sampler,
            size,
        })
    }
}

/// Software rendered Texture Surface with an underlying SDL_Texture that is
/// written to for drawing. all writing to texture happens when flush() is called
///
/// type: Streaming,
/// format: RGB888 (With alpha padding),
///
pub struct SDLTextureBuf {
    tex: sdl2::render::Texture,
    pixels: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
}

impl SDLTextureBuf {
    pub fn new(tc: &TextureCreator<WindowContext>, w: u32, h: u32) -> anyhow::Result<Self> {
        let tex = tc.create_texture_streaming(PixelFormatEnum::RGB888, w as u32, h as u32)?;
        // let pixels = vec![0; w * h * 4];
        let buf = image::DynamicImage::new_rgb8(w, h);
        let pixels = buf.to_rgb8();
        let s = Self { tex, pixels };
        Ok(s)
    }

    #[inline]
    pub fn pitch(&self) -> u32 {
        self.width()
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.pixels.width()
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.pixels.height()
    }

    // pub const fn index(&self, x: usize, y: usize) -> usize {
    //     let pitch = self.pitch() as usize;
    //     y * pitch + x * 4
    // }

    // #[inline]
    // pub fn byte_at(&self, x: usize, y: usize) -> u8 {
    //     self.pixels[self.index(x, y)]
    // }

    // #[inline]
    // pub fn pixel(&self, x: usize, y: usize) -> sdl2::pixels::Color {
    //     let i = self.index(x, y);
    //     let r = self.pixels[i];
    //     let g = self.pixels[i + 1];
    //     let b = self.pixels[i + 2];
    //     sdl2::pixels::Color::RGB(r, g, b)
    // }

    pub fn flush(&mut self) -> anyhow::Result<()> {
        // let bytes = self.pixels.as_flat_samples();
        self.tex
            .update(None, self.pixels.as_bytes(), self.pitch() as _)?;

        Ok(())
    }

    pub fn put(&mut self, x: u32, y: u32, color: image::Rgb<u8>) {
        self.pixels.put_pixel(x, y, color);
        // let i = self.index(x, y);
        //
        // self.pixels[i] = color.r;
        // self.pixels[i + 1] = color.g;
        // self.pixels[i + 2] = color.b;
    }

    pub fn clear_black(&mut self) {
        for p in self.pixels.iter_mut() {
            *p = 0;
        }

        for x in 0..self.width() {
            self.put(x, self.height() / 2, image::Rgb([255, 255, 255]));
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> anyhow::Result<()> {
        canvas
            .copy(
                &self.tex, None,
                None,
                // Rect::new(0, 0, self.w as u32 / 2, self.h as u32 / 2),
            )
            .map_err(|e| anyhow!(e))?;
        Ok(())
    }
}
