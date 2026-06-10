use std::{path::Path, sync::Arc};

use derive_more::From;
use glam::Vec2;

use crate::{
    core::ray::Ray,
    error::Result,
    geometry::{
        bounding_box::BoundingBox,
        primitive::{Plane, Primitive, Sphere, Triangle, TriangleMesh},
        surfel::Surfel,
    },
    parse::dto::{ObjectDTO, TriangleMeshDTO},
    render::aggregator::{Bvh, PrimitiveAggregator, PrimitiveList},
};

pub trait Hit {
    fn bounding_box(&self) -> BoundingBox;
    fn intersect(&self, ray: &mut Ray) -> Option<Surfel>;
    fn intersect_any(&self, ray: &mut Ray) -> bool;
}

#[derive(Debug, Clone, From)]
pub enum Hittable {
    Primitive(Primitive),
    Aggregate(PrimitiveAggregator),
}

impl Hittable {
    pub fn from_object(
        object_dto: ObjectDTO,
        material_id: usize,
        xml_file_path: &Path,
    ) -> Result<Vec<Hittable>> {
        match object_dto {
            ObjectDTO::Sphere { center, radius } => Ok(vec![
                Primitive::new(
                    Sphere {
                        center: center.into(),
                        radius,
                    }
                    .into(),
                    material_id,
                )
                .into(),
            ]),
            ObjectDTO::Plane { point, normal } => Ok(vec![
                Primitive::new(Plane::new(point.into(), normal.into()).into(), material_id).into(),
            ]),
            ObjectDTO::TriangleMesh(TriangleMeshDTO::Inline {
                ntriangles,
                vertices,
                vertex_indices,
                normals,
                normal_indices,
                uvs,
                uv_indices,
                reverse_vertex_order: _reverse_vertex_order,
                compute_normals: _compute_normals,
                backface_cull,
            }) => {
                let mesh = Arc::new(TriangleMesh {
                    vertex_indices: vertex_indices.0,
                    normal_indices: normal_indices.0,
                    uvcoord_indices: uv_indices.map_or_else(|| vec![0; ntriangles * 3], |a| a.0),
                    vertices: vertices.0,
                    normals: normals.0,
                    uvcoords: uvs.map_or_else(|| vec![Vec2::ZERO], |a| a.0),
                });

                let triangles = (0..ntriangles)
                    .map(|i| {
                        Primitive::new(
                            Triangle::new(mesh.clone(), i, backface_cull.unwrap_or(true)).into(),
                            material_id,
                        )
                        .into()
                    })
                    .collect();

                Ok(triangles)
            }
            ObjectDTO::TriangleMesh(TriangleMeshDTO::File {
                filename,
                reverse_vertex_order,
                backface_cull,
            }) => {
                let load_options = tobj::LoadOptions {
                    triangulate: true,
                    ..Default::default()
                };

                let current_dir = xml_file_path.parent().unwrap_or_else(|| Path::new(""));
                let resolved_path = current_dir.join(filename);

                let (models, _) = tobj::load_obj(resolved_path, &load_options)?;

                let triangles = TriangleMesh::from_obj(
                    &models,
                    reverse_vertex_order,
                    backface_cull.unwrap_or(true),
                    material_id,
                );

                Ok(triangles)
            }
        }
    }
}

impl From<PrimitiveList> for Hittable {
    fn from(value: PrimitiveList) -> Self {
        Self::Aggregate(PrimitiveAggregator::from(value))
    }
}

impl From<Bvh> for Hittable {
    fn from(value: Bvh) -> Self {
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

    fn intersect(&self, ray: &mut Ray) -> Option<Surfel> {
        match self {
            Hittable::Primitive(inner) => inner.intersect(ray),
            Hittable::Aggregate(inner) => inner.intersect(ray),
        }
    }

    fn intersect_any(&self, ray: &mut Ray) -> bool {
        match self {
            Hittable::Primitive(inner) => inner.intersect_any(ray),
            Hittable::Aggregate(inner) => inner.intersect_any(ray),
        }
    }
}
