use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};

use glam::Vec3A;
use static_assertions::assert_eq_size;

use crate::{
    core::ray::Ray,
    geometry::{
        bounding_box::BoundingBox,
        hittable::{Hit, Hittable},
        surfel::Surfel,
    },
};

#[derive(Debug, Clone, Copy)]
pub enum SplitMethod {
    Sah,
}

#[derive(Clone, Copy)]
struct BucketInfo {
    count: usize,
    bounds: BoundingBox,
}

const NUM_BUCKETS: usize = 12;

#[derive(Debug, Clone)]
pub struct PrimitiveInfo {
    index: usize,
    bounds: BoundingBox,
    centroid: Vec3A,
}

impl PrimitiveInfo {
    pub fn new(index: usize, hittable: &Hittable) -> Self {
        let bounds = hittable.bounding_box();
        Self {
            index,
            bounds,
            centroid: (bounds.min + bounds.max) * 0.5,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BvhNodeKind {
    Leaf {
        first_prim_offset: usize,
        num_prims: usize,
    },
    Interior {
        split_axis: u8,
        left: Box<BvhBuildNode>,
        right: Box<BvhBuildNode>,
    },
}

#[derive(Debug, Clone)]
pub struct BvhBuildNode {
    pub bounds: BoundingBox,
    pub kind: BvhNodeKind,
}

impl BvhBuildNode {
    pub fn new_leaf(first_prim_offset: usize, num_prims: usize, bounds: BoundingBox) -> Self {
        Self {
            bounds,
            kind: BvhNodeKind::Leaf {
                first_prim_offset,
                num_prims,
            },
        }
    }

    pub fn new_interior(axis: u8, left: Box<BvhBuildNode>, right: Box<BvhBuildNode>) -> Self {
        let bounds = BoundingBox::join(left.bounds, right.bounds);
        Self {
            bounds,
            kind: BvhNodeKind::Interior {
                split_axis: axis,
                left,
                right,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LinearBvhNode {
    pub bounds_min: [f32; 3],
    pub bounds_max: [f32; 3],
    pub offset: u32,
    pub num_prims: u16,
    pub axis: u8,
    pub _pad: u8,
}

assert_eq_size!(LinearBvhNode, [u8; 32]);

impl LinearBvhNode {
    pub fn new(bounds: BoundingBox, offset: u32, num_prims: u16, axis: u8) -> Self {
        Self {
            bounds_min: bounds.min.to_array(),
            bounds_max: bounds.max.to_array(),
            offset,
            num_prims,
            axis,
            _pad: 0,
        }
    }

    pub fn bounds(&self) -> BoundingBox {
        BoundingBox::new(self.min(), self.max())
    }

    #[inline]
    pub fn min(&self) -> Vec3A {
        Vec3A::from_array(self.bounds_min)
    }

    #[inline]
    pub fn max(&self) -> Vec3A {
        Vec3A::from_array(self.bounds_max)
    }
}

#[derive(Debug, Clone)]
pub struct Bvh {
    pub nodes: Vec<LinearBvhNode>,
    pub primitives: Vec<Hittable>,
}

impl Bvh {
    #[allow(unused)]
    pub fn build(
        primitives: &[Hittable],
        max_prims_per_node: usize,
        split_method: SplitMethod,
    ) -> Self {
        let start = Instant::now();
        if primitives.is_empty() {
            return Self {
                nodes: vec![],
                primitives: vec![],
            };
        }

        let mut primitive_info: Vec<PrimitiveInfo> = primitives
            .iter()
            .enumerate()
            .map(|(i, p)| PrimitiveInfo::new(i, p))
            .collect();

        let mut total_nodes = AtomicUsize::new(0);
        let temp_root = match split_method {
            SplitMethod::Sah => {
                Self::build_recursive(&mut primitive_info, 0, &total_nodes, max_prims_per_node)
            }
        };

        let num_nodes = total_nodes.load(Ordering::Relaxed);
        let mut linear_nodes = Vec::with_capacity(num_nodes);
        let mut offset = 0;

        Self::flatten_bvh_tree(&temp_root, &mut offset, &mut linear_nodes);

        let mut ordered_prims = Vec::with_capacity(primitives.len());
        for info in primitive_info {
            ordered_prims.push(primitives[info.index].clone());
        }

        let elapsed = start.elapsed();

        println!("Time to build BVH: {elapsed:?}");

        Self {
            nodes: linear_nodes,
            primitives: ordered_prims,
        }
    }

    fn build_recursive(
        primitive_info: &mut [PrimitiveInfo],
        start_offset: usize,
        total_nodes: &AtomicUsize,
        max_prims_per_node: usize,
    ) -> BvhBuildNode {
        total_nodes.fetch_add(1, Ordering::Relaxed);

        let mut bounds = BoundingBox::EMPTY;
        let mut centroid_bounds = BoundingBox::EMPTY;
        for info in primitive_info.iter() {
            bounds.expand_by_box(&info.bounds);
            centroid_bounds.expand_by_point(info.centroid);
        }

        let num_prims = primitive_info.len();
        if num_prims <= 1 || centroid_bounds.longest_axis_len() < 1e-6 {
            return BvhBuildNode::new_leaf(start_offset, num_prims, bounds);
        }

        let mut mid = primitive_info.len() / 2;
        let split_axis = centroid_bounds.longest_axis();

        if num_prims <= 4 {
            primitive_info.select_nth_unstable_by(mid, |a, b| {
                a.centroid[split_axis as usize].total_cmp(&b.centroid[split_axis as usize])
            });
        } else {
            let mut buckets = [BucketInfo {
                count: 0,
                bounds: BoundingBox::EMPTY,
            }; NUM_BUCKETS];

            for info in primitive_info.iter() {
                let min = centroid_bounds.min[split_axis as usize];
                let max = centroid_bounds.max[split_axis as usize];

                let offset = (info.centroid[split_axis as usize] - min) / (max - min);

                let mut b = (offset * NUM_BUCKETS as f32) as usize;

                if b == NUM_BUCKETS {
                    b = NUM_BUCKETS - 1;
                }

                buckets[b].count += 1;
                buckets[b].bounds.expand_by_box(&info.bounds);
            }

            let costs = Self::compute_sah_costs(&buckets, &bounds);

            let (min_cost_split_bucket, &min_cost) = costs
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.total_cmp(b))
                .unwrap();

            let leaf_cost = num_prims as f32;

            if num_prims > max_prims_per_node || min_cost < leaf_cost {
                mid = Self::partition_primitives(
                    primitive_info,
                    &centroid_bounds,
                    split_axis,
                    min_cost_split_bucket,
                );

                if mid == 0 || mid == num_prims {
                    mid = num_prims / 2;
                    primitive_info.select_nth_unstable_by(mid, |a, b| {
                        a.centroid[split_axis as usize]
                            .partial_cmp(&b.centroid[split_axis as usize])
                            .unwrap()
                    });
                }
            } else {
                return BvhBuildNode::new_leaf(start_offset, num_prims, bounds);
            }
        }

        let (left_info, right_info) = primitive_info.split_at_mut(mid);

        let (left_child, right_child) = rayon::join(
            || {
                Box::new(Self::build_recursive(
                    left_info,
                    start_offset,
                    total_nodes,
                    max_prims_per_node,
                ))
            },
            || {
                Box::new(Self::build_recursive(
                    right_info,
                    start_offset + mid,
                    total_nodes,
                    max_prims_per_node,
                ))
            },
        );

        BvhBuildNode::new_interior(split_axis, left_child, right_child)
    }

    fn compute_sah_costs(
        buckets: &[BucketInfo; NUM_BUCKETS],
        bounds: &BoundingBox,
    ) -> [f32; NUM_BUCKETS - 1] {
        let mut costs = [0f32; NUM_BUCKETS - 1];
        for (i, cost) in costs.iter_mut().enumerate() {
            let mut b0 = BoundingBox::EMPTY;
            let mut b1 = BoundingBox::EMPTY;

            let mut count0 = 0;
            let mut count1 = 0;

            for bucket in &buckets[..=i] {
                b0.expand_by_box(&bucket.bounds);
                count0 += bucket.count;
            }

            for bucket in &buckets[i + 1..] {
                b1.expand_by_box(&bucket.bounds);
                count1 += bucket.count;
            }
            // TODO: Change the cost calculation since static dispatch is used.
            let denom = bounds.surface_area().max(1e-6);
            *cost = 0.125
                + (count0 as f32 * b0.surface_area() + count1 as f32 * b1.surface_area()) / denom;
        }

        costs
    }

    fn partition_primitives(
        primitive_info: &mut [PrimitiveInfo],
        centroid_bounds: &BoundingBox,
        split_axis: u8,
        min_cost_split_bucket: usize,
    ) -> usize {
        let min_bound = centroid_bounds.min[split_axis as usize];
        let max_bound = centroid_bounds.max[split_axis as usize];

        let mut mid = 0;

        for i in 0..primitive_info.len() {
            let offset = (primitive_info[i].centroid[split_axis as usize] - min_bound)
                / (max_bound - min_bound);
            let mut b = (offset * NUM_BUCKETS as f32) as usize;
            if b == NUM_BUCKETS {
                b = NUM_BUCKETS - 1;
            }

            if b <= min_cost_split_bucket {
                primitive_info.swap(mid, i);
                mid += 1;
            }
        }

        mid
    }

    fn flatten_bvh_tree(
        node: &BvhBuildNode,
        offset: &mut u32,
        linear_nodes: &mut Vec<LinearBvhNode>,
    ) -> u32 {
        let my_offset = *offset;
        *offset += 1;

        match &node.kind {
            BvhNodeKind::Leaf {
                first_prim_offset,
                num_prims,
            } => {
                linear_nodes.push(LinearBvhNode::new(
                    node.bounds,
                    *first_prim_offset as u32,
                    *num_prims as u16,
                    0,
                ));
            }
            BvhNodeKind::Interior {
                split_axis,
                left,
                right,
            } => {
                linear_nodes.push(LinearBvhNode::new(node.bounds, 0, 0, *split_axis));

                Self::flatten_bvh_tree(left, offset, linear_nodes);

                let right_child_offset = Self::flatten_bvh_tree(right, offset, linear_nodes);

                linear_nodes[my_offset as usize].offset = right_child_offset;
            }
        }

        my_offset
    }
}

impl Hit for Bvh {
    fn bounding_box(&self) -> BoundingBox {
        if self.nodes.is_empty() {
            return BoundingBox::EMPTY;
        }

        let root = &self.nodes[0];

        BoundingBox {
            min: root.min(),
            max: root.max(),
        }
    }

    fn intersect(&self, ray: &mut Ray) -> Option<Surfel> {
        let mut final_hit = None;

        let mut current_node_index = 0;
        let mut to_visit_offset = 0;

        let mut nodes_to_visit = [0_usize; 64];

        let dir_is_neg = [
            ray.inv_dir.x < 0.0,
            ray.inv_dir.y < 0.0,
            ray.inv_dir.z < 0.0,
        ];

        loop {
            let node = &self.nodes[current_node_index];
            if node.bounds().hit(ray) {
                if node.num_prims > 0 {
                    let start = node.offset as usize;
                    let end = start + node.num_prims as usize;

                    for i in start..end {
                        if let Some(isect) = self.primitives[i].intersect(ray) {
                            ray.t_max = isect.t;
                            final_hit = Some(isect);
                        }
                    }

                    if to_visit_offset == 0 {
                        break;
                    }
                    to_visit_offset -= 1;
                    current_node_index = nodes_to_visit[to_visit_offset];
                } else {
                    let axis = node.axis as usize;

                    if dir_is_neg[axis] {
                        nodes_to_visit[to_visit_offset] = current_node_index + 1;
                        to_visit_offset += 1;

                        current_node_index = node.offset as usize;
                    } else {
                        nodes_to_visit[to_visit_offset] = node.offset as usize;
                        to_visit_offset += 1;

                        current_node_index += 1;
                    }
                }
            } else {
                if to_visit_offset == 0 {
                    break;
                }
                to_visit_offset -= 1;
                current_node_index = nodes_to_visit[to_visit_offset];
            }
        }

        final_hit
    }

    fn intersect_any(&self, ray: &mut Ray) -> bool {
        let mut current_node_index = 0;
        let mut to_visit_offset = 0;

        let mut nodes_to_visit = [0_usize; 64];

        let dir_is_neg = [
            ray.inv_dir.x < 0.0,
            ray.inv_dir.y < 0.0,
            ray.inv_dir.z < 0.0,
        ];

        loop {
            let node = &self.nodes[current_node_index];
            if node.bounds().hit(ray) {
                if node.num_prims > 0 {
                    let start = node.offset as usize;
                    let end = start + node.num_prims as usize;

                    for i in start..end {
                        if self.primitives[i].intersect_any(ray) {
                            return true;
                        }
                    }

                    if to_visit_offset == 0 {
                        return false;
                    }
                    to_visit_offset -= 1;
                    current_node_index = nodes_to_visit[to_visit_offset];
                } else {
                    let axis = node.axis as usize;

                    if dir_is_neg[axis] {
                        nodes_to_visit[to_visit_offset] = current_node_index + 1;
                        to_visit_offset += 1;

                        current_node_index = node.offset as usize;
                    } else {
                        nodes_to_visit[to_visit_offset] = node.offset as usize;
                        to_visit_offset += 1;

                        current_node_index += 1;
                    }
                }
            } else {
                if to_visit_offset == 0 {
                    return false;
                }
                to_visit_offset -= 1;
                current_node_index = nodes_to_visit[to_visit_offset];
            }
        }
    }
}
