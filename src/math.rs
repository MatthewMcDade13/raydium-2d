use nalgebra_glm as glm;

pub const PI: f32 = 3.1415926535897932384626433832795;
pub const HALF_PI: f32 = 1.5707963267948966192313216916398;
pub const TWO_PI: f32 = 6.283185307179586476925286766559;
pub const DEG_TO_RAD: f32 = 0.017453292519943295769236907684886;
pub const RAD_TO_DEG: f32 = 57.295779513082320876798154814105;
pub const EULER: f32 = 2.718281828459045235360287471352;

#[inline]
pub fn radians(degrees: f32) -> f32 {
    degrees * DEG_TO_RAD
}

#[inline]
pub fn degrees(radians: f32) -> f32 {
    radians * RAD_TO_DEG
}

#[derive(Debug, Default, Clone)]
pub struct TransformDesc {
    pub position: Option<glm::Vec2>,
    pub scale: Option<glm::Vec2>,
    pub rotation: Option<glm::Vec2>,
}

#[derive(Debug, Default, Clone)]
pub struct Transform {
    position: glm::Vec2,
    scale: glm::Vec2,
    pub origin_offset: glm::Vec2,
    rotation: f32,

    model: glm::Mat4,

    needs_update: bool,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            model: glm::identity(),
            ..Default::default()
        }
    }

    pub fn set_position(&mut self, pos: glm::Vec2) {
        self.position = pos;
        self.needs_update = true;
    }

    pub fn set_scale(&mut self, scale: glm::Vec2) {
        self.scale = scale;
        self.needs_update = true;
    }

    pub fn set_rot(&mut self, rot: f32) {
        self.rotation = rot;
        self.needs_update = true;
    }

    pub fn translate(&mut self, offset: glm::Vec2) {
        let pos = self.position + offset;
        self.set_position(pos);
    }

    pub fn scale(&mut self, offset: glm::Vec2) {
        let scale = self.scale + offset;
        self.set_scale(scale);
    }

    pub fn rotate(&mut self, deg: f32) {
        let rot = self.rotation + deg;
        self.set_rot(rot);
    }

    pub fn model_mat(&mut self) -> &glm::Mat4 {
        if self.needs_update {
            let pos = glm::vec2_to_vec3(&self.position);
            let origin_offset = glm::vec2_to_vec3(&self.origin_offset);
            let scale = glm::vec2_to_vec3(&self.scale);

            let model = glm::translate(&glm::identity(), &pos);
            let model = glm::translate(&model, &origin_offset);
            let rad = radians(self.rotation);
            let d = glm::vec3(0., 0., 1.);
            let model = glm::rotate(&model, rad, &d);
            let model = glm::translate(&model, &-origin_offset);
            let model = glm::scale(&model, &scale);
            self.model = model;
            self.needs_update = false;
            &self.model
        } else {
            &self.model
        }
    }
}
