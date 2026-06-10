use std::sync::Arc;

use glam::{Vec2, Vec3A};

use crate::{
    core::ray::Ray,
    geometry::{
        bounding_box::BoundingBox, hittable::Hittable, primitive::Primitive, surfel::HitRecord,
    },
};

#[derive(Debug, Clone)]
pub struct TriangleMesh {
    pub vertex_indices: Vec<u32>,
    pub normal_indices: Vec<u32>,
    pub uvcoord_indices: Vec<u32>,

    pub vertices: Vec<Vec3A>,
    pub normals: Vec<Vec3A>,
    pub uvcoords: Vec<Vec2>,
}

impl TriangleMesh {
    pub fn from_obj(
        models: &[tobj::Model],
        reverse_vertex_order: bool,
        backface_cull: bool,
        material_id: usize,
    ) -> Vec<Hittable> {
        let mut triangles = Vec::new();

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

            let mut vertex_indices = tobj_mesh.indices.clone();
            let mut normal_indices = tobj_mesh.normal_indices.clone();
            let mut uvcoord_indices = tobj_mesh.texcoord_indices.clone();

            let n_triangles = vertex_indices.len() / 3;

            if n_triangles == 0 {
                continue;
            }

            if uvcoords.is_empty() {
                uvcoords = vec![Vec2::ZERO];
                uvcoord_indices = vec![0; n_triangles * 3];
            }

            if reverse_vertex_order {
                for i in (0..vertex_indices.len()).step_by(3) {
                    vertex_indices.swap(i + 1, i + 2);
                }

                if !normal_indices.is_empty() {
                    for i in (0..normal_indices.len()).step_by(3) {
                        normal_indices.swap(i + 1, i + 2);
                    }
                }

                for i in (0..uvcoord_indices.len()).step_by(3) {
                    uvcoord_indices.swap(i + 1, i + 2);
                }
            }

            let mesh = Arc::new(TriangleMesh {
                vertex_indices,
                normal_indices,
                uvcoord_indices,
                vertices,
                normals,
                uvcoords,
            });

            for i in 0..n_triangles {
                triangles.push(
                    Primitive::new(
                        Triangle::new(mesh.clone(), i, backface_cull).into(),
                        material_id,
                    )
                    .into(),
                );
            }
        }

        triangles
    }
}

#[derive(Debug, Clone)]
pub struct Triangle {
    v: [u32; 3],
    n: [u32; 3],
    uv: [u32; 3],
    mesh: Arc<TriangleMesh>,
    backface_cull: bool,
}

impl Triangle {
    pub fn new(mesh: Arc<TriangleMesh>, tri_id: usize, bfc: bool) -> Triangle {
        let index = 3 * tri_id;
        let v = [
            mesh.vertex_indices[index],
            mesh.vertex_indices[index + 1],
            mesh.vertex_indices[index + 2],
        ];

        let n = [
            mesh.normal_indices[index],
            mesh.normal_indices[index + 1],
            mesh.normal_indices[index + 2],
        ];

        let uv = [
            mesh.uvcoord_indices[index],
            mesh.uvcoord_indices[index + 1],
            mesh.uvcoord_indices[index + 2],
        ];

        Self {
            v,
            n,
            uv,
            mesh,
            backface_cull: bfc,
        }
    }

    pub fn bounding_box(&self) -> BoundingBox {
        let v0 = self.mesh.vertices[self.v[0] as usize];
        let v1 = self.mesh.vertices[self.v[1] as usize];
        let v2 = self.mesh.vertices[self.v[2] as usize];

        let x_min = v0.x.min(v1.x).min(v2.x);
        let x_max = v0.x.max(v1.x).max(v2.x);

        let y_min = v0.y.min(v1.y).min(v2.y);
        let y_max = v0.y.max(v1.y).max(v2.y);

        let z_min = v0.z.min(v1.z).min(v2.z);
        let z_max = v0.z.max(v1.z).max(v2.z);

        BoundingBox::new(
            Vec3A::new(x_min, y_min, z_min),
            Vec3A::new(x_max, y_max, z_max),
        )
    }

    pub fn intersect(&self, ray: &mut Ray) -> Option<HitRecord> {
        let v0 = self.mesh.vertices[self.v[0] as usize];
        let v1 = self.mesh.vertices[self.v[1] as usize];
        let v2 = self.mesh.vertices[self.v[2] as usize];

        let edge1 = v1 - v0;
        let edge2 = v2 - v0;

        let pvec = ray.direction.cross(edge2);

        let det = edge1.dot(pvec);

        let mut t;
        let mut u;
        let mut v;

        if self.backface_cull {
            if det < 1e-6 {
                return None;
            }

            let tvec = ray.origin - v0;

            u = tvec.dot(pvec);

            if u < 0.0 || u > det {
                return None;
            }

            let qvec = tvec.cross(edge1);

            v = ray.direction.dot(qvec);

            if v < 0.0 || u + v > det {
                return None;
            }

            t = edge2.dot(qvec);
            let inv_det = 1.0 / det;

            t *= inv_det;
            u *= inv_det;
            v *= inv_det;
        } else {
            if det > -1e-6 && det < 1e-6 {
                return None;
            }

            let inv_det = 1.0 / det;

            let tvec = ray.origin - v0;

            u = tvec.dot(pvec) * inv_det;
            if !(0.0..=1.0).contains(&u) {
                return None;
            }

            let qvec = tvec.cross(edge1);

            v = ray.direction.dot(qvec) * inv_det;
            if v < 0.0 || u + v > 1.0 {
                return None;
            }

            t = edge2.dot(qvec) * inv_det;
        }

        if t < ray.t_min || t > ray.t_max {
            return None;
        }

        let point = ray.origin + ray.direction * t;

        let n0 = self.mesh.normals[self.n[0] as usize];
        let n1 = self.mesh.normals[self.n[1] as usize];
        let n2 = self.mesh.normals[self.n[2] as usize];

        let w = 1.0 - u - v;

        let mut np = w * n0 + u * n1 + v * n2;
        np = np.normalize();

        if ray.direction.dot(np) > 0.0 {
            np = -np;
        }

        let uv0 = self.mesh.uvcoords[self.uv[0] as usize];
        let uv1 = self.mesh.uvcoords[self.uv[1] as usize];
        let uv2 = self.mesh.uvcoords[self.uv[2] as usize];

        let uvp = w * uv0 + u * uv1 + v * uv2;

        Some(HitRecord {
            point,
            normal: np,
            u: uvp.x,
            v: uvp.y,
            t,
        })
    }

    pub fn intersect_any(&self, ray: &mut Ray) -> bool {
        let v0 = self.mesh.vertices[self.v[0] as usize];
        let v1 = self.mesh.vertices[self.v[1] as usize];
        let v2 = self.mesh.vertices[self.v[2] as usize];

        let edge1 = v1 - v0;
        let edge2 = v2 - v0;

        let pvec = ray.direction.cross(edge2);

        let det = edge1.dot(pvec);

        let mut t;
        let u;
        let v;

        if self.backface_cull {
            if det < 1e-6 {
                return false;
            }

            let tvec = ray.origin - v0;

            u = tvec.dot(pvec);

            if u < 0.0 || u > det {
                return false;
            }

            let qvec = tvec.cross(edge1);

            v = ray.direction.dot(qvec);

            if v < 0.0 || u + v > det {
                return false;
            }

            t = edge2.dot(qvec);
            let inv_det = 1.0 / det;

            t *= inv_det;
        } else {
            if det > -1e-6 && det < 1e-6 {
                return false;
            }

            let inv_det = 1.0 / det;

            let tvec = ray.origin - v0;

            u = tvec.dot(pvec) * inv_det;
            if !(0.0..=1.0).contains(&u) {
                return false;
            }

            let qvec = tvec.cross(edge1);

            v = ray.direction.dot(qvec) * inv_det;
            if v < 0.0 || u + v > 1.0 {
                return false;
            }

            t = edge2.dot(qvec) * inv_det;
        }

        if t < ray.t_min || t > ray.t_max {
            return false;
        }

        true
    }
}
