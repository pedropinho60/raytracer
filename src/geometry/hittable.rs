use std::{path::Path, sync::Arc};

use derive_more::From;
use glam::{Vec2, Vec3A};

use crate::{
    api::RenderState,
    core::ray::Ray,
    error::Result,
    geometry::{
        bounding_box::BoundingBox,
        primitive::{Plane, Primitive, Sphere, Triangle, TriangleMesh},
        surfel::Surfel,
    },
    parse::dto::{ObjectDTO, TriangleMeshDTO},
    render::aggregator::{PrimitiveAggregator, PrimitiveBVH, PrimitiveList},
};

pub trait Hit {
    fn bounding_box(&self) -> BoundingBox;
    fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Surfel>;
    fn intersect_any(&self, ray: Ray, t_min: f32, t_max: f32) -> bool;
}

#[derive(Debug, Clone, From)]
pub enum Hittable {
    Primitive(Primitive),
    Aggregate(PrimitiveAggregator),
}

impl Hittable {
    pub fn add_to_array(
        state: &mut RenderState,
        object_dto: ObjectDTO,
        material_id: usize,
        xml_file_path: &Path,
    ) -> Result<()> {
        match object_dto {
            ObjectDTO::Sphere { center, radius } => {
                state.primitives.push(
                    Primitive::new(
                        Sphere {
                            center: center.into(),
                            radius,
                        }
                        .into(),
                        material_id,
                    )
                    .into(),
                );
                Ok(())
            }
            ObjectDTO::Plane { point, normal } => {
                state.primitives.push(
                    Primitive::new(Plane::new(point.into(), normal.into()).into(), material_id)
                        .into(),
                );
                Ok(())
            }
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
                    n_triangles: ntriangles,
                    vertex_indices: vertex_indices.0,
                    normal_indices: normal_indices.0,
                    uvcoord_indices: uv_indices.map_or_else(|| vec![0; ntriangles * 3], |a| a.0),
                    vertices: vertices.0,
                    normals: normals.0,
                    uvcoords: uvs.map_or_else(|| vec![Vec2::ZERO], |a| a.0),
                });

                for i in 0..ntriangles {
                    state.primitives.push(
                        Primitive::new(
                            Triangle::new(mesh.clone(), i, backface_cull).into(),
                            material_id,
                        )
                        .into(),
                    );
                }

                Ok(())
            }
            ObjectDTO::TriangleMesh(TriangleMeshDTO::File { filename }) => {
                let load_options = tobj::LoadOptions {
                    triangulate: true,
                    ..Default::default()
                };

                let current_dir = xml_file_path.parent().unwrap_or_else(|| Path::new(""));
                let resolved_path = current_dir.join(filename);

                let (models, _) = tobj::load_obj(resolved_path, &load_options)?;

                for model in models {
                    let tobj_mesh = &model.mesh;

                    let vertices: Vec<Vec3A> = tobj_mesh
                        .positions
                        .chunks_exact(3)
                        .map(|c| Vec3A::new(c[0], c[1], c[2]))
                        .collect();

                    let normals: Vec<Vec3A> = tobj_mesh
                        .normals
                        .chunks_exact(3)
                        .map(|c| Vec3A::new(c[0], c[1], c[2]))
                        .collect();

                    let mut uvcoords: Vec<Vec2> = tobj_mesh
                        .texcoords
                        .chunks_exact(2)
                        .map(|c| Vec2::new(c[0], c[1]))
                        .collect();

                    let mut uvcoord_indices = tobj_mesh.texcoord_indices.clone();

                    let n_triangles = tobj_mesh.indices.len() / 3;

                    if n_triangles == 0 {
                        continue;
                    }

                    if uvcoords.is_empty() {
                        uvcoords = vec![Vec2::ZERO];
                        uvcoord_indices = vec![0; n_triangles * 3];
                    }

                    let mesh = Arc::new(TriangleMesh {
                        n_triangles,
                        vertex_indices: tobj_mesh.indices.clone(),
                        normal_indices: tobj_mesh.normal_indices.clone(),
                        uvcoord_indices,
                        vertices,
                        normals,
                        uvcoords,
                    });

                    for i in 0..n_triangles {
                        state.primitives.push(
                            Primitive::new(
                                Triangle::new(mesh.clone(), i, false).into(),
                                material_id,
                            )
                            .into(),
                        );
                    }
                }

                Ok(())
            }
        }
    }
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

    fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Surfel> {
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
