use derive_more::From;

use crate::{
    aggregator::{PrimitiveAggregator, PrimitiveBVH, PrimitiveList},
    bounding_box::BoundingBox,
    primitive::Primitive,
    ray::Ray,
    surfel::Surfel,
};

pub trait Hit {
    fn bounding_box(&self) -> BoundingBox;
    fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<(f32, Surfel)>;
    fn intersect_any(&self, ray: Ray, t_min: f32, t_max: f32) -> bool;
}

#[derive(Debug, Clone, From)]
pub enum Hittable {
    Primitive(Primitive),
    Aggregate(PrimitiveAggregator),
}

impl From<PrimitiveList> for Hittable {
    fn from(value: PrimitiveList) -> Self {
        Self::Aggregate(PrimitiveAggregator::from(value))
    }
}

impl From<PrimitiveBVH> for Hittable {
    fn from(value: PrimitiveBVH) -> Self {
        Self::Aggregate(PrimitiveAggregator::from(value))
    }
}

impl Hit for Hittable {
    fn bounding_box(&self) -> BoundingBox {
        match self {
            Hittable::Primitive(inner) => inner.bounding_box(),
            Hittable::Aggregate(inner) => inner.bounding_box(),
        }
    }

    fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<(f32, Surfel)> {
        match self {
            Hittable::Primitive(inner) => inner.intersect(ray, t_min, t_max),
            Hittable::Aggregate(inner) => inner.intersect(ray, t_min, t_max),
        }
    }

    fn intersect_any(&self, ray: Ray, t_min: f32, t_max: f32) -> bool {
        match self {
            Hittable::Primitive(inner) => inner.intersect_any(ray, t_min, t_max),
            Hittable::Aggregate(inner) => inner.intersect_any(ray, t_min, t_max),
        }
    }
}
