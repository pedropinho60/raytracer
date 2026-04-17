use crate::RGBColor;

#[derive(Clone)]
pub enum Material {
    Flat { kd: RGBColor },
}
