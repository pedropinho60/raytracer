use crate::{background::Background, camera::Camera, object::Object};

pub struct Scene {
    pub background: Box<dyn Background>,
    pub camera: Box<dyn Camera>,
    pub objects: Vec<Box<dyn Object>>,
}
