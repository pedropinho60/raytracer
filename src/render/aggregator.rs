use std::cmp::Ordering;

use derive_more::From;

use crate::{
    core::ray::Ray,
    geometry::{
        bounding_box::{BoundingBox, Interval},
        hittable::{Hit, Hittable},
        surfel::Surfel,
    },
    parse::dto::AggregatorDTO,
};

#[derive(Debug, Clone, From)]
pub enum PrimitiveAggregator {
    List(PrimitiveList),
    Bvh(PrimitiveBVH),
    BvhNode(BVHNode),
}

impl PrimitiveAggregator {
    pub fn build(aggregator_dto: AggregatorDTO, list: Vec<Hittable>) -> Self {
        match aggregator_dto {
            AggregatorDTO::List => PrimitiveList::new(list).into(),
            AggregatorDTO::Bvh { max_prims_per_node } => {
                PrimitiveBVH::new(list, max_prims_per_node).into()
            }
        }
    }
}

impl Hit for PrimitiveAggregator {
    fn bounding_box(&self) -> BoundingBox {
        match self {
            PrimitiveAggregator::List(inner) => inner.bounding_box(),
            PrimitiveAggregator::Bvh(inner) => inner.bounding_box(),
            PrimitiveAggregator::BvhNode(inner) => inner.bounding_box(),
        }
    }

    fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Surfel> {
        match self {
            PrimitiveAggregator::List(inner) => inner.intersect(ray, t_min, t_max),
            PrimitiveAggregator::Bvh(inner) => inner.intersect(ray, t_min, t_max),
            PrimitiveAggregator::BvhNode(inner) => inner.intersect(ray, t_min, t_max),
        }
    }

    fn intersect_any(&self, ray: Ray, t_min: f32, t_max: f32) -> bool {
        match self {
            PrimitiveAggregator::List(inner) => inner.intersect_any(ray, t_min, t_max),
            PrimitiveAggregator::Bvh(inner) => inner.intersect_any(ray, t_min, t_max),
            PrimitiveAggregator::BvhNode(inner) => inner.intersect_any(ray, t_min, t_max),
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

    fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Surfel> {
        let mut closest_hit = None;

        let mut t_closest = t_max;

        for primitive in &self.primitives {
            if let Some(surfel) = primitive.intersect(ray, t_min, t_closest) {
                t_closest = surfel.t;
                closest_hit = Some(surfel);
            }
        }

        closest_hit
    }

    fn intersect_any(&self, ray: Ray, t_min: f32, t_max: f32) -> bool {
        self.primitives
            .iter()
            .any(|p| p.intersect_any(ray, t_min, t_max))
    }
}

#[derive(Debug, Clone)]
pub struct PrimitiveBVH {
    root: Box<Hittable>,
}

impl PrimitiveBVH {
    pub fn new(list: Vec<Hittable>, max_prims_per_node: usize) -> Self {
        Self {
            root: Box::new(BVHNode::build(list, max_prims_per_node)),
        }
    }
}

impl Hit for PrimitiveBVH {
    fn bounding_box(&self) -> BoundingBox {
        self.root.bounding_box()
    }

    fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Surfel> {
        self.root.intersect(ray, t_min, t_max)
    }

    fn intersect_any(&self, ray: Ray, t_min: f32, t_max: f32) -> bool {
        self.root.intersect_any(ray, t_min, t_max)
    }
}

#[derive(Debug, Clone)]
pub struct BVHNode {
    left: Box<Hittable>,
    right: Box<Hittable>,
    bbox: BoundingBox,
}

impl BVHNode {
    pub fn build(mut list: Vec<Hittable>, max_prims_per_node: usize) -> Hittable {
        let mut bbox = BoundingBox::EMPTY;

        for object in &list {
            bbox = BoundingBox::join(bbox, object.bounding_box());
        }

        if list.len() <= max_prims_per_node {
            return PrimitiveList::new(list).into();
        }

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            2 => Self::box_z_compare,
            _ => unreachable!(),
        };

        list.sort_unstable_by(comparator);

        let left;
        let right;

        match list.len() {
            1 => {
                left = Box::new(list[0].clone());
                right = Box::new(list[0].clone());
            }
            2 => {
                left = Box::new(list[0].clone());
                right = Box::new(list[1].clone());
            }
            n => {
                let mid = n / 2;
                let right_list = list.split_off(mid);

                left = Box::new(Self::build(list, max_prims_per_node));
                right = Box::new(Self::build(right_list, max_prims_per_node));
            }
        }

        Self { left, right, bbox }.into()
    }

    fn box_compare(a: &Hittable, b: &Hittable, axis_index: usize) -> Ordering {
        let a_axis_interval = a.bounding_box()[axis_index];
        let b_axis_interval = b.bounding_box()[axis_index];
        a_axis_interval.min.total_cmp(&b_axis_interval.min)
    }

    fn box_x_compare(a: &Hittable, b: &Hittable) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Hittable, b: &Hittable) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Hittable, b: &Hittable) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hit for BVHNode {
    fn bounding_box(&self) -> BoundingBox {
        self.bbox
    }

    fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Surfel> {
        if !self.bbox.hit(ray, Interval::new(t_min, t_max)) {
            return None;
        }

        let hit_left = self.left.intersect(ray, t_min, t_max);
        let hit_right = self
            .right
            .intersect(ray, t_min, hit_left.map_or(t_max, |s| s.t));

        match (hit_left, hit_right) {
            (Some(l), Some(r)) => {
                if l.t < r.t {
                    Some(l)
                } else {
                    Some(r)
                }
            }
            (l, r) => l.or(r),
        }
    }

    fn intersect_any(&self, ray: Ray, t_min: f32, t_max: f32) -> bool {
        if !self.bbox.hit(ray, Interval::new(t_min, t_max)) {
            return false;
        }

        let hit_left = self.left.intersect_any(ray, t_min, t_max);
        let hit_right = self.right.intersect_any(ray, t_min, t_max);

        hit_left || hit_right
    }
}

impl From<BVHNode> for Hittable {
    fn from(value: BVHNode) -> Self {
        Self::Aggregate(PrimitiveAggregator::Bvh(PrimitiveBVH {
            root: Box::new(Hittable::Aggregate(PrimitiveAggregator::BvhNode(value))),
        }))
    }
}
