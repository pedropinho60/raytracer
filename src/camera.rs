use crate::{
    WindowSize,
    film::Film,
    math::{Point2, Point3, Vec3},
    ray::Ray,
};

pub trait Camera {
    fn generate_ray(&self, point: Point2) -> Ray;
    fn film(&mut self) -> &mut Film;
}

pub struct PerspectiveCamera {
    look_at: Point3,
    look_from: Point3,
    up: Vec3,
    fovy: u16,
    film: Film,
}

impl PerspectiveCamera {
    pub fn new(look_at: Point3, look_from: Point3, up: Vec3, fovy: u16, film: Film) -> Self {
        Self {
            look_at,
            look_from,
            up,
            fovy,
            film,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn generate_ray(&self, point: Point2) -> Ray {
        let width = self.film.width() as f64;
        let height = self.film.height() as f64;

        let h = (self.fovy as f64 / 2.0).to_radians().tan();

        let aspect_ratio = width / height;

        let left = -aspect_ratio * h;
        let right = aspect_ratio * h;
        let bottom = -h;
        let top = h;

        let u = left + (right - left) * (point.col as f64 + 0.5) / width;
        let v = bottom + (top - bottom) * (height - 1.0 - point.row as f64 + 0.5) / height;

        let gaze = self.look_at - self.look_from;
        let vec_w = gaze.normalize();
        let vec_u = self.up.cross(vec_w).normalize();
        let vec_v = vec_w.cross(vec_u).normalize();

        let direction = vec_w + u * vec_u + v * vec_v;

        Ray {
            origin: self.look_from,
            direction,
        }
    }

    fn film(&mut self) -> &mut Film {
        &mut self.film
    }
}

pub struct OrthographicCamera {
    look_at: Point3,
    look_from: Point3,
    up: Vec3,
    dimensions: WindowSize,
    film: Film,
}

impl OrthographicCamera {
    pub fn new(
        look_at: Point3,
        look_from: Point3,
        up: Vec3,
        dimensions: WindowSize,
        film: Film,
    ) -> Self {
        Self {
            look_at,
            look_from,
            up,
            film,
            dimensions,
        }
    }
}

impl Camera for OrthographicCamera {
    fn generate_ray(&self, point: Point2) -> Ray {
        let width = self.film.width() as f64;
        let height = self.film.height() as f64;

        let left = self.dimensions.left;
        let right = self.dimensions.right;
        let bottom = self.dimensions.bottom;
        let top = self.dimensions.top;

        let u = left + (right - left) * (point.col as f64 + 0.5) / width;
        let v = bottom + (top - bottom) * (height - 1.0 - point.row as f64 + 0.5) / height;

        let gaze = self.look_at - self.look_from;
        let vec_w = gaze.normalize();
        let vec_u = self.up.cross(vec_w).normalize();
        let vec_v = vec_w.cross(vec_u).normalize();

        let origin = self.look_from + u * vec_u + v * vec_v;

        Ray {
            origin,
            direction: vec_w,
        }
    }

    fn film(&mut self) -> &mut Film {
        &mut self.film
    }
}
