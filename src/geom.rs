use nalgebra_glm as glm;

use crate::{
    gfx::{self, Vert2D},
    math::Transform,
};

#[derive(Debug, Default, Clone)]
pub struct Quad {
    pub xform: Transform,
    pub color: glm::Vec4,

    pub z_index: i32,
}

impl Quad {
    pub const VERTS: [Vert2D; 4] = [
        Vert2D {
            pos: [-1., -1., 0., 0.],
            color: gfx::Color::WHITE_RAW,
            uv: [0., 1.],
        },
        Vert2D {
            pos: [1., -1., 0., 0.],
            color: gfx::Color::WHITE_RAW,
            uv: [1., 1.],
        },
        Vert2D {
            pos: [1., 1., 0., 0.],
            color: gfx::Color::WHITE_RAW,
            uv: [1., 0.],
        },
        Vert2D {
            pos: [-1., 1., 0., 0.],
            color: gfx::Color::WHITE_RAW,
            uv: [0., 0.],
        },
    ];

    /// Calculates/Applies self.xform and self.color to Quad verts
    pub fn calc_into_verts(&self) -> [Vert2D; 4] {
        todo!()
    }
}
//
// #[derive(Debug, Clone)]
// pub struct QuadBuffer {
//     vert_buf: Vec<Vert2D>,
//     index_buf: Vec<u32>,
//     current: u32,
// }
//
// impl QuadBuffer {
//     pub fn empty() -> Self {
//         Self {
//             vert_buf: Vec::with_capacity(16),
//             index_buf: Vec::with_capacity(32),
//             current: 0,
//         }
//     }
//
//     pub fn push_quad(&mut self, quad: &Quad) {
//         let verts = quad.as_verts();
//         self.vert_buf.extend(verts);
//
//         self.index_buf.extend(&[
//             self.current * 4 + 0,
//             self.current * 4 + 1,
//             self.current * 4 + 2,
//             self.current * 4 + 0,
//             self.current * 4 + 2,
//             self.current * 4 + 3,
//         ]);
//         self.current += 1;
//     }
//
//     pub fn push_with_xform(&mut self, quad: &Quad, xform: &nal::Matrix4<f32>) {
//         let verts = translate_verts(quad.as_verts().as_slice(), xform);
//         self.vert_buf.extend(verts);
//
//         self.index_buf.extend(&[
//             self.current * 4 + 0,
//             self.current * 4 + 1,
//             self.current * 4 + 2,
//             self.current * 4 + 0,
//             self.current * 4 + 2,
//             self.current * 4 + 3,
//         ]);
//         self.current += 1;
//     }
//
//     pub fn vertex_buffer(&self) -> &[Vert2D] {
//         self.vert_buf.as_slice()
//     }
//
//     pub fn index_buffer(&self) -> &[u32] {
//         self.index_buf.as_slice()
//     }
// }
//
// fn translate_verts(verts: &[Vert2D], xform: &nal::Matrix4<f32>) -> Vec<Vert2D> {
//     let mut result = Vec::with_capacity(verts.len());
//     for v in verts.iter() {
//         let pos = nal::Vector4::new(v.pos[0], v.pos[1], v.pos[2], 1.);
//         let new_pos = xform * pos;
//         let new_pos = [new_pos.x, new_pos.y, new_pos.z];
//         result.push(Vert2D::new(&new_pos, &v.uv))
//     }
//     result
// }
//
// fn normalize_texture_coords(
//     verts: &mut [Vert2D],
//     texture_rect: &Rect,
//     texture_size: &nal::Vector2<f32>,
// ) {
//     let texture_rect_size = nal::Vector2::new(texture_rect.w, texture_rect.h);
//     let uv_offset = nal::Vector2::new(texture_rect.x, texture_rect.y);
//
//     for v in verts.iter_mut() {
//         let flipped = nal::Vector2::new(v.uv[0], 1. - v.uv[1]);
//
//         let texture_dim = nal::Vector2::new(
//             texture_rect_size.x * flipped.x,
//             texture_rect_size.y * flipped.y,
//         );
//         let texture_coord = texture_dim + uv_offset;
//         let normalized = nal::Vector2::new(
//             texture_coord.x / texture_size.x,
//             texture_coord.y / texture_size.y,
//         );
//         v.uv = [normalized.x, normalized.y];
//     }
// }
