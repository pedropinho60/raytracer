use derive_more::From;

use crate::{
    WindowSize,
    math::{Point2, Point3, Vec3},
    ray::Ray,
};

#[derive(From)]
pub enum Camera {
    Perspective(PerspectiveCamera),
    Orthographic(OrthographicCamera),
}

impl Camera {
    pub fn generate_ray(&self, point: Point2, width: u16, height: u16) -> Ray {
        match self {
            Camera::Perspective(inner) => inner.generate_ray(point, width, height),
            Camera::Orthographic(inner) => inner.generate_ray(point, width, height),
        }
    }
}

pub struct PerspectiveCamera {
    look_from: Point3,
    look_at: Point3,
    up: Vec3,
    dimensions: WindowSize,
}

impl PerspectiveCamera {
    pub fn new(look_from: Point3, look_at: Point3, up: Vec3, dimensions: WindowSize) -> Self {
        Self {
            look_from,
            look_at,
            up,
            dimensions,
        }
    }

    pub fn generate_ray(&self, point: Point2, width: u16, height: u16) -> Ray {
        let width = width as f64;
        let height = height as f64;

        let left = self.dimensions.left;
        let right = self.dimensions.right;
        let bottom = self.dimensions.bottom;
        let top = self.dimensions.top;

        let u = left + (right - left) * (point.col as f64 + 0.5) / width;
        let v = bottom + (top - bottom) * (height - 1.0 - point.row as f64 + 0.5) / height;

        let gaze = self.look_at - self.look_from;
        let vec_w = gaze.normalize();
        let vec_u = self.up.cross(vec_w).normalize();
        let vec_v = vec_w.cross(vec_u);

        let direction = vec_w + u * vec_u + v * vec_v;

        Ray::new(self.look_from, direction.normalize())
    }
}

pub struct OrthographicCamera {
    look_from: Point3,
    look_at: Point3,
    up: Vec3,
    dimensions: WindowSize,
}

impl OrthographicCamera {
    pub fn new(look_from: Point3, look_at: Point3, up: Vec3, dimensions: WindowSize) -> Self {
        Self {
            look_from,
            look_at,
            up,
            dimensions,
        }
    }

    pub fn generate_ray(&self, point: Point2, width: u16, height: u16) -> Ray {
        let width = width as f64;
        let height = height as f64;

        let left = self.dimensions.left;
        let right = self.dimensions.right;
        let bottom = self.dimensions.bottom;
        let top = self.dimensions.top;

        let u = left + (right - left) * (point.col as f64 + 0.5) / width;
        let v = bottom + (top - bottom) * (height - 1.0 - point.row as f64 + 0.5) / height;

        let gaze = self.look_at - self.look_from;
        let vec_w = gaze.normalize();
        let vec_u = self.up.cross(vec_w).normalize();
        let vec_v = vec_w.cross(vec_u);

        let origin = self.look_from + u * vec_u + v * vec_v;

        Ray::new(origin, vec_w)
    }
}
