use derive_more::From;
use glam::Vec3A;
use serde::Deserialize;

use crate::{
    core::ray::Ray,
    parse::dto::{CameraArgsDTO, CameraDTO},
};

#[derive(Debug, Clone, Copy)]
pub struct ViewPlane {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}

impl<'de> Deserialize<'de> for ViewPlane {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let parts: Vec<_> = s.split_whitespace().collect();

        if parts.len() != 4 {
            return Err(serde::de::Error::custom(
                "Expected exactly 4 size components",
            ));
        }

        let left = parts[0].parse().map_err(serde::de::Error::custom)?;
        let right = parts[1].parse().map_err(serde::de::Error::custom)?;
        let bottom = parts[2].parse().map_err(serde::de::Error::custom)?;
        let top = parts[3].parse().map_err(serde::de::Error::custom)?;

        Ok(ViewPlane {
            left,
            right,
            bottom,
            top,
        })
    }
}

#[derive(From)]
pub enum Camera {
    Perspective(PerspectiveCamera),
    Orthographic(OrthographicCamera),
}

impl Camera {
    pub fn build(
        camera_dto: CameraDTO,
        camera_args: CameraArgsDTO,
        width: u16,
        height: u16,
    ) -> Camera {
        let CameraArgsDTO {
            look_from,
            look_at,
            up,
        } = camera_args;

        match camera_dto {
            CameraDTO::Orthographic { screen_window } => {
                OrthographicCamera::new(look_from.into(), look_at.into(), up.into(), screen_window)
                    .into()
            }
            CameraDTO::Perspective { fovy } => PerspectiveCamera::new(
                look_from.into(),
                look_at.into(),
                up.into(),
                fovy,
                width,
                height,
            )
            .into(),
        }
    }

    pub fn generate_ray(&self, row: usize, col: usize, width: usize, height: usize) -> Ray {
        match self {
            Camera::Perspective(inner) => inner.generate_ray(row, col, width, height),
            Camera::Orthographic(inner) => inner.generate_ray(row, col, width, height),
        }
    }
}

pub struct PerspectiveCamera {
    origin: Vec3A,
    dimensions: ViewPlane,
    u: Vec3A,
    v: Vec3A,
    w: Vec3A,
}

impl PerspectiveCamera {
    pub fn new(
        look_from: Vec3A,
        look_at: Vec3A,
        up: Vec3A,
        fovy: u16,
        width: u16,
        height: u16,
    ) -> Self {
        let h = (f32::from(fovy) / 2.0).to_radians().tan();

        let aspect_ratio = f32::from(width) / f32::from(height);

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
            dimensions: ViewPlane {
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

    pub fn generate_ray(&self, row: usize, col: usize, width: usize, height: usize) -> Ray {
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
    origin: Vec3A,
    dimensions: ViewPlane,
    u: Vec3A,
    v: Vec3A,
    w: Vec3A,
}

impl OrthographicCamera {
    pub fn new(look_from: Vec3A, look_at: Vec3A, up: Vec3A, dimensions: ViewPlane) -> Self {
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

    pub fn generate_ray(&self, row: usize, col: usize, width: usize, height: usize) -> Ray {
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
