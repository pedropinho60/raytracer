use std::cmp::Ordering;

use derive_more::From;

use crate::{
    bounding_box::{BoundingBox, Interval},
    hittable::{Hit, Hittable},
    ray::Ray,
    surfel::Surfel,
};

#[derive(Debug, Clone, From)]
pub enum PrimitiveAggregator {
    List(PrimitiveList),
    Bvh(PrimitiveBVH),
}

impl Hit for PrimitiveAggregator {
    fn bounding_box(&self) -> BoundingBox {
        match self {
            PrimitiveAggregator::List(inner) => inner.bounding_box(),
            PrimitiveAggregator::Bvh(inner) => inner.bounding_box(),
        }
    }

    fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<(f32, Surfel)> {
        match self {
            PrimitiveAggregator::List(inner) => inner.intersect(ray, t_min, t_max),
            PrimitiveAggregator::Bvh(inner) => inner.intersect(ray, t_min, t_max),
        }
    }

    fn intersect_any(&self, ray: Ray, t_min: f32, t_max: f32) -> bool {
        match self {
            PrimitiveAggregator::List(inner) => inner.intersect_any(ray, t_min, t_max),
            PrimitiveAggregator::Bvh(inner) => inner.intersect_any(ray, t_min, t_max),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrimitiveList {
    primitives: Vec<Hittable>,
    bbox: BoundingBox,
}

impl PrimitiveList {
    pub fn new(list: &[Hittable]) -> Self {
        let bbox = list.iter().fold(BoundingBox::EMPTY, |a, b| {
            BoundingBox::join(a, b.bounding_box())
        });

        Self {
            primitives: list.to_vec(),
            bbox,
        }
    }
}

impl Hit for PrimitiveList {
    fn bounding_box(&self) -> BoundingBox {
        self.bbox
    }

    fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<(f32, Surfel)> {
        let mut closest_hit = None;

        let mut t_closest = t_max;

        for primitive in &self.primitives {
            if let Some((t, surfel)) = primitive.intersect(ray, t_min, t_closest)
                && !surfel.from_behind
            {
                t_closest = t;
                closest_hit = Some(surfel);
            }
        }

        Some((t_closest, closest_hit?))
    }

    fn intersect_any(&self, ray: Ray, t_min: f32, t_max: f32) -> bool {
        self.primitives
            .iter()
            .any(|p| p.intersect_any(ray, t_min, t_max))
    }
}

#[derive(Debug, Clone)]
pub struct PrimitiveBVH {
    root: BVHNode,
}

impl PrimitiveBVH {
    pub fn new(list: &[Hittable]) -> Self {
        Self {
            root: BVHNode::new(list),
        }
    }
}

impl Hit for PrimitiveBVH {
    fn bounding_box(&self) -> BoundingBox {
        self.root.bounding_box()
    }

    fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<(f32, Surfel)> {
        self.root.intersect(ray, t_min, t_max)
    }

    fn intersect_any(&self, ray: Ray, t_min: f32, t_max: f32) -> bool {
        self.root.intersect_any(ray, t_min, t_max)
    }
}

#[derive(Debug, Clone)]
struct BVHNode {
    left: Box<Hittable>,
    right: Box<Hittable>,
    bbox: BoundingBox,
}

impl BVHNode {
    pub fn new(list: &[Hittable]) -> Self {
        let mut bbox = BoundingBox::EMPTY;

        for object in list {
            bbox = BoundingBox::join(bbox, object.bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            2 => Self::box_z_compare,
            _ => unreachable!(),
        };

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
            _ => {
                let mut vec = list.to_vec();
                vec.sort_unstable_by(comparator);

                let mid = list.len() / 2;

                left = Box::new(Self::new(&list[..mid]).into());
                right = Box::new(Self::new(&list[mid..]).into());
            }
        }

        Self { left, right, bbox }
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

    fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<(f32, Surfel)> {
        if !self.bbox.hit(ray, Interval::new(t_min, t_max)) {
            return None;
        }

        let hit_left = self.left.intersect(ray, t_min, t_max);
        let hit_right = self
            .right
            .intersect(ray, t_min, hit_left.map(|(t, _)| t).unwrap_or(t_max));

        match (hit_left, hit_right) {
            (Some(l), Some(r)) => {
                if l.0 < r.0 {
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
        Self::Aggregate(PrimitiveAggregator::Bvh(PrimitiveBVH { root: value }))
    }
}
