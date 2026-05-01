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
    origin: Point3,
    dimensions: WindowSize,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl PerspectiveCamera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        up: Vec3,
        fovy: u16,
        width: u16,
        height: u16,
    ) -> Self {
        let h = (fovy as f64 / 2.0).to_radians().tan();

        let aspect_ratio = width as f64 / height as f64;

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

    pub fn generate_ray(&self, point: Point2, width: u16, height: u16) -> Ray {
        let width = width as f64;
        let height = height as f64;

        let left = self.dimensions.left;
        let right = self.dimensions.right;
        let bottom = self.dimensions.bottom;
        let top = self.dimensions.top;

        let u = left + (right - left) * (point.col as f64 + 0.5) / width;
        let v = bottom + (top - bottom) * (height - 1.0 - point.row as f64 + 0.5) / height;

        let direction = self.w + u * self.u + v * self.v;

        Ray::new(self.origin, direction.normalize())
    }
}

pub struct OrthographicCamera {
    origin: Point3,
    dimensions: WindowSize,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl OrthographicCamera {
    pub fn new(look_from: Point3, look_at: Point3, up: Vec3, dimensions: WindowSize) -> Self {
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

    pub fn generate_ray(&self, point: Point2, width: u16, height: u16) -> Ray {
        let width = width as f64;
        let height = height as f64;

        let left = self.dimensions.left;
        let right = self.dimensions.right;
        let bottom = self.dimensions.bottom;
        let top = self.dimensions.top;

        let u = left + (right - left) * (point.col as f64 + 0.5) / width;
        let v = bottom + (top - bottom) * (height - 1.0 - point.row as f64 + 0.5) / height;

        let origin = self.origin + u * self.u + v * self.v;

        Ray::new(origin, self.w)
    }
}
