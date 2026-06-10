mod bvh;

use derive_more::From;

use crate::{
    core::ray::Ray,
    geometry::{
        bounding_box::BoundingBox,
        hittable::{Hit, Hittable},
        surfel::Surfel,
    },
    parse::dto::AggregatorDTO,
};

pub use bvh::Bvh;

#[derive(Debug, Clone, From)]
pub enum PrimitiveAggregator {
    List(PrimitiveList),
    Bvh(Bvh),
}

impl PrimitiveAggregator {
    pub fn build(aggregator_dto: AggregatorDTO, list: Vec<Hittable>) -> Self {
        match aggregator_dto {
            AggregatorDTO::List => PrimitiveList::new(list).into(),
            AggregatorDTO::Bvh { max_prims_per_node } => {
                Bvh::build(&list, max_prims_per_node, bvh::SplitMethod::Sah).into()
            }
        }
    }
}

impl Hit for PrimitiveAggregator {
    fn bounding_box(&self) -> BoundingBox {
        match self {
            PrimitiveAggregator::List(inner) => inner.bounding_box(),
            PrimitiveAggregator::Bvh(inner) => inner.bounding_box(),
        }
    }

    fn intersect(&self, ray: &mut Ray) -> Option<Surfel> {
        match self {
            PrimitiveAggregator::List(inner) => inner.intersect(ray),
            PrimitiveAggregator::Bvh(inner) => inner.intersect(ray),
        }
    }

    fn intersect_any(&self, ray: &mut Ray) -> bool {
        match self {
            PrimitiveAggregator::List(inner) => inner.intersect_any(ray),
            PrimitiveAggregator::Bvh(inner) => inner.intersect_any(ray),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrimitiveList {
    primitives: Vec<Hittable>,
    bbox: BoundingBox,
}

impl PrimitiveList {
    pub fn new(list: Vec<Hittable>) -> Self {
        let bbox = list.iter().fold(BoundingBox::EMPTY, |a, b| {
            BoundingBox::join(a, b.bounding_box())
        });

        Self {
            primitives: list,
            bbox,
        }
    }
}

impl Hit for PrimitiveList {
    fn bounding_box(&self) -> BoundingBox {
        self.bbox
    }

    fn intersect(&self, ray: &mut Ray) -> Option<Surfel> {
        if !self.bbox.hit(ray) {
            return None;
        }

        let mut closest_hit = None;

        for primitive in &self.primitives {
            if let Some(surfel) = primitive.intersect(ray) {
                ray.t_max = surfel.t;
                closest_hit = Some(surfel);
            }
        }

        closest_hit
    }

    fn intersect_any(&self, ray: &mut Ray) -> bool {
        self.primitives.iter().any(|p| p.intersect_any(ray))
    }
}
