use derive_more::From;
use glam::Vec3;

use crate::{WindowSize, ray::Ray};

#[derive(From)]
pub enum Camera {
    Perspective(PerspectiveCamera),
    Orthographic(OrthographicCamera),
}

impl Camera {
    pub fn generate_ray(&self, row: u16, col: u16, width: u16, height: u16) -> Ray {
        match self {
            Camera::Perspective(inner) => inner.generate_ray(row, col, width, height),
            Camera::Orthographic(inner) => inner.generate_ray(row, col, width, height),
        }
    }
}

pub struct PerspectiveCamera {
    origin: Vec3,
    dimensions: WindowSize,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl PerspectiveCamera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        up: Vec3,
        fovy: u16,
        width: u16,
        height: u16,
    ) -> Self {
        let h = (fovy as f32 / 2.0).to_radians().tan();

        let aspect_ratio = width as f32 / height as f32;

        let left = -aspect_ratio * h;
        let right = aspect_ratio * h;
        let bottom = -h;
        let top = h;

        let gaze = look_at - look_from;
        let vec_w = gaze.normalize();
        let vec_u = up.cross(vec_w).normalize();
        let vec_v = vec_w.cross(vec_u);

        Self {
            origin: look_from,
            dimensions: WindowSize {
                left,
                right,
                bottom,
                top,
            },
            u: vec_u,
            v: vec_v,
            w: vec_w,
        }
    }

    pub fn generate_ray(&self, row: u16, col: u16, width: u16, height: u16) -> Ray {
        let width = width as f32;
        let height = height as f32;

        let left = self.dimensions.left;
        let right = self.dimensions.right;
        let bottom = self.dimensions.bottom;
        let top = self.dimensions.top;

        let u = left + (right - left) * (col as f32 + 0.5) / width;
        let v = bottom + (top - bottom) * (height - 1.0 - row as f32 + 0.5) / height;

        let direction = self.w + u * self.u + v * self.v;

        Ray::new(self.origin, direction.normalize())
    }
}

pub struct OrthographicCamera {
    origin: Vec3,
    dimensions: WindowSize,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl OrthographicCamera {
    pub fn new(look_from: Vec3, look_at: Vec3, up: Vec3, dimensions: WindowSize) -> Self {
        let gaze = look_at - look_from;
        let vec_w = gaze.normalize();
        let vec_u = up.cross(vec_w).normalize();
        let vec_v = vec_w.cross(vec_u);

        Self {
            origin: look_from,
            dimensions,
            u: vec_u,
            v: vec_v,
            w: vec_w,
        }
    }

    pub fn generate_ray(&self, row: u16, col: u16, width: u16, height: u16) -> Ray {
        let width = width as f32;
        let height = height as f32;

        let left = self.dimensions.left;
        let right = self.dimensions.right;
        let bottom = self.dimensions.bottom;
        let top = self.dimensions.top;

        let u = left + (right - left) * (col as f32 + 0.5) / width;
        let v = bottom + (top - bottom) * (height - 1.0 - row as f32 + 0.5) / height;

        let origin = self.origin + u * self.u + v * self.v;

        Ray::new(origin, self.w)
    }
}
