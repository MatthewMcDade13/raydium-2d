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

    pub const fn with_pos(pos: glm::Vec2) -> Self {
        Self {
            pos: [pos.x, pos.y, 0., 0.],
            color: [0.0; 4],
            uv: [0.; 2],
        }
    }

    pub const fn with_color(color: &glm::Vec4) -> Self {
        Self {
            pos: [0.; 4],
            color: [color.x, color.y, color.z, color.w],
            uv: [0.; 2],
        }
    }

    pub const fn with_uv(uv: glm::Vec2) -> Self {
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
    pub const WHITE: Self = Self(glm::vec4(1., 1., 1., 1.));
    pub const RED: Self = Self(glm::vec4(1., 0., 0., 1.));
    pub const GREEN: Self = Self(glm::vec4(0., 1., 0., 1.));
    pub const BLUE: Self = Self(glm::vec4(0., 0., 1., 1.));

    pub const WHITE_RAW: [f32; 4] = [1., 1., 1., 1.];
    pub const RED_RAW: [f32; 4] = [1., 0., 0., 1.];
    pub const GREEN_RAW: [f32; 4] = [0., 1., 0., 1.];
    pub const BLUE_RAW: [f32; 4] = [0., 0., 1., 1.];
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
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

use anyhow::*;
use image::GenericImageView;
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
