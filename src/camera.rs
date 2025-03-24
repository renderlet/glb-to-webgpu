use crate::winit::event::MouseButton;
use glm::{Quat, Mat4, Vec2, Vec3, Vec4};
use itertools::Itertools;
use nalgebra_glm::{self as glm, quat_to_mat4};

use crate::model::GPUVertex;

#[derive(Copy, Clone, Debug)]
enum MouseState {
    Unknown,
    Free(Vec2),
    Rotate(Vec2),
    Pan(Vec2, Vec3),
}

pub struct Camera {
    /// Aspect ratio of the window
    width: f32,
    height: f32,

    /// Model scale
    scale: f32,

    /// Center of view volume
    center: Vec3,

    mouse: MouseState,

    pub quat: Quat
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Camera {
            width,
            height,
            quat: Default::default(),
            scale: 1.0,
            center: Vec3::zeros(),
            mouse: MouseState::Unknown,
        }
    }

    fn mat(&self) -> Mat4 {
        self.view_matrix() * self.model_matrix()
    }

    fn mat_i(&self) -> Mat4 {
        (self.view_matrix() * self.model_matrix())
            .try_inverse()
            .expect("Failed to invert mouse matrix")
    }

    pub fn fit_verts(&mut self, verts: &[GPUVertex]) {
        let xb = verts
            .iter()
            .map(|v| v.pos[0])
            .minmax()
            .into_option()
            .unwrap();
        let yb = verts
            .iter()
            .map(|v| v.pos[1])
            .minmax()
            .into_option()
            .unwrap();
        let zb = verts
            .iter()
            .map(|v| v.pos[2])
            .minmax()
            .into_option()
            .unwrap();
        let dx = xb.1 - xb.0;
        let dy = yb.1 - yb.0;
        let dz = zb.1 - zb.0;
        self.scale = (0.8/ dx.max(dy).max(dz)) as f32;
        self.center = Vec3::new(
            (xb.0 + xb.1) as f32 / 2.0,
            (yb.0 + yb.1) as f32 / 2.0,
            (zb.0 + zb.1) as f32 / 2.0,
        );
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn model_matrix(&self) -> Mat4 {
        let i = Mat4::identity();
        // The transforms below are applied bottom-to-top when thinking about
        // the model, i.e. it's translated, then scaled, then rotated, etc.

        let mut lh_to_rh = Mat4::identity();
        lh_to_rh.m33 = -1.0;

        let mat_quat = quat_to_mat4(&self.quat);

        // Convert from left handed to right handed rotations
        lh_to_rh *

        // Scale to compensate for model size
        glm::scale(&i, &Vec3::new(self.scale, self.scale, self.scale)) *

        mat_quat * 

        // Recenter model
        glm::translate(&i, &-self.center)
    }

    /// Returns a matrix which compensates for window aspect ratio and clipping
    pub fn view_matrix(&self) -> Mat4 {
        let i = Mat4::identity();
        // The Z clipping range is 0-1, so push forward
        glm::translate(&i, &Vec3::new(0.0, 0.0, 0.5)) *

        // Scale to compensate for aspect ratio and reduce Z scale to improve
        // clipping
        glm::scale(&i, &Vec3::new(1.0, self.width / self.height, 0.1))
    }

}
