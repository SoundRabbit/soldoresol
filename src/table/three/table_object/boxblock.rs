use super::super::TextureTable;
use crate::arena::{block, resource, BlockRef};
use crate::libs::random_id::U128Id;
use crate::libs::three;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::JsCast;

struct Mesh {
    material: three::MeshStandardMaterial,
    data: three::Mesh,
}

pub struct Boxblock {
    geometry_box: three::BoxGeometry,
    geometry_cylinder: three::CylinderGeometry,
    geometry_icosahedron: three::IcosahedronGeometry,
    meshs: HashMap<U128Id, Mesh>,
}

impl Boxblock {
    pub fn new() -> Self {
        Self {
            geometry_box: Self::create_box_geometry(),
            geometry_cylinder: Self::create_cylinder_geometry(),
            geometry_icosahedron: Self::create_icosahedron(),
            meshs: HashMap::new(),
        }
    }

    pub fn update(
        &mut self,
        texture_table: &mut TextureTable,
        scene: &three::Scene,
        boxblocks: impl Iterator<Item = BlockRef<block::Boxblock>>,
    ) {
        let mut unused = self.meshs.keys().map(U128Id::clone).collect::<HashSet<_>>();

        for boxblock in boxblocks {
            let boxblock_id = boxblock.id();
            unused.remove(&boxblock_id);

            boxblock.map(|boxblock| {
                if !self.meshs.contains_key(&boxblock_id) {
                    let material = three::MeshStandardMaterial::new(&object! {});
                    let data = three::Mesh::new(self.get_geometry(boxblock.shape()), &material);
                    data.set_render_order(super::ORDER_BOXBLOCK);
                    data.set_user_data(&boxblock_id.to_jsvalue());
                    scene.add(&data);
                    self.meshs
                        .insert(U128Id::clone(&boxblock_id), Mesh { material, data });
                }
                if let Some(mesh) = self.meshs.get(&boxblock_id) {
                    mesh.data.set_geometry(self.get_geometry(boxblock.shape()));
                    let [px, py, pz] = boxblock.position().clone();
                    mesh.data.position().set(px, py, pz);
                    let [sx, sy, sz] = boxblock.size().clone();
                    mesh.data.scale().set(sx, sy, sz);
                    let [r, g, b, ..] = boxblock.color().to_color().to_f64array();
                    mesh.material.color().set_rgb(r, g, b);

                    if let Some(texture) = boxblock.texture() {
                        if let Some(texture) = texture_table.load_block(BlockRef::clone(&texture)) {
                            mesh.material.set_map(&texture);
                            mesh.material.set_needs_update(true);
                        }
                    }
                }
            });
        }

        for unused_boxblock_id in unused {
            if let Some(mesh) = self.meshs.get(&unused_boxblock_id) {
                scene.remove(&mesh.data);
            }
        }
    }

    fn get_geometry(&self, shape: block::boxblock::Shape) -> &three::BufferGeometry {
        match shape {
            block::boxblock::Shape::Cube => &self.geometry_box,
            block::boxblock::Shape::Cylinder => &self.geometry_cylinder,
            block::boxblock::Shape::Sphere => &self.geometry_icosahedron,
            _ => &self.geometry_box,
        }
    }

    fn create_box_geometry() -> three::BoxGeometry {
        let geometry = three::BoxGeometry::new(1.0, 1.0, 1.0);
        let a_position = geometry
            .get_attribute("position")
            .array_as_f32array()
            .to_vec();
        let a_normal = geometry
            .get_attribute("normal")
            .array_as_f32array()
            .to_vec();
        let mut uv = vec![];

        for i in 0..(a_position.len() / 3) {
            let p_x = a_position[i * 3];
            let p_y = a_position[i * 3 + 1];
            let p_z = a_position[i * 3 + 2];

            let n_x = a_normal[i * 3];
            let n_y = a_normal[i * 3 + 1];
            let n_z = a_normal[i * 3 + 2];

            let [u, v] = match Self::factor(n_x, n_y, n_z) {
                0 => Self::texture_coord(0, &[p_y * 2.0, p_z * 2.0]),
                1 => Self::texture_coord(1, &[-p_x * 2.0, p_z * 2.0]),
                2 => Self::texture_coord(2, &[p_x * 2.0, p_y * 2.0]),
                3 => Self::texture_coord(3, &[-p_y * 2.0, p_z * 2.0]),
                4 => Self::texture_coord(4, &[p_x * 2.0, p_z * 2.0]),
                5 => Self::texture_coord(5, &[p_x * 2.0, p_y * 2.0]),
                _ => unreachable!(),
            };

            uv.push(u);
            uv.push(v);
        }

        geometry.set_attribute(
            "uv",
            &three::BufferAttribute::new_with_f32array(
                &js_sys::Float32Array::from(uv.as_slice()),
                2,
                false,
            ),
        );

        geometry
    }

    fn create_cylinder_geometry() -> three::CylinderGeometry {
        let geometry = three::CylinderGeometry::new(0.5, 0.5, 1.0, 16);

        let a_position = geometry
            .get_attribute("position")
            .array_as_f32array()
            .to_vec();
        let a_normal = geometry
            .get_attribute("normal")
            .array_as_f32array()
            .to_vec();
        let a_uv = geometry.get_attribute("uv").array_as_f32array().to_vec();
        let mut uv = vec![];

        for i in 0..(a_position.len() / 3) {
            let p_x = a_position[i * 3];
            let p_y = a_position[i * 3 + 1];
            let p_z = a_position[i * 3 + 2];

            let n_x = a_normal[i * 3];
            let n_y = a_normal[i * 3 + 1];
            let n_z = a_normal[i * 3 + 2];

            let u = a_uv[i * 2];

            let [u, v] = match Self::factor(n_x, n_y, n_z) {
                0 => [u, p_y * 0.5 + 0.25 + 0.5 / 2.0],
                1 => Self::texture_coord(2, &[p_x * 2.0, p_z * 2.0]),
                2 => [u, p_y * 0.5 + 0.25 + 0.5 / 2.0],
                3 => [u, p_y * 0.5 + 0.25 + 0.5 / 2.0],
                4 => Self::texture_coord(5, &[p_x * 2.0, p_z * 2.0]),
                5 => [u, p_y * 0.5 + 0.25 + 0.5 / 2.0],
                _ => unreachable!(),
            };

            uv.push(u);
            uv.push(v);
        }

        geometry.set_attribute(
            "uv",
            &three::BufferAttribute::new_with_f32array(
                &js_sys::Float32Array::from(uv.as_slice()),
                2,
                false,
            ),
        );

        geometry
    }

    fn create_icosahedron() -> three::IcosahedronGeometry {
        let geometry = three::IcosahedronGeometry::new(0.5, 5);
        geometry
    }

    fn factor(x: f32, y: f32, z: f32) -> usize {
        let ax = x.abs();
        let ay = y.abs();
        let az = z.abs();

        if ax > ay && ax > az {
            if x > 0.0 {
                0
            } else {
                3
            }
        } else if ay > ax && ay > az {
            if y > 0.0 {
                1
            } else {
                4
            }
        } else {
            if z > 0.0 {
                2
            } else {
                5
            }
        }
    }

    // 0: px
    // 1: py
    // 2: pz
    // 3: nx
    // 4: ny
    // 5: nz
    fn texture_coord(surface: usize, coord: &[f32; 2]) -> [f32; 2] {
        let [ox, oy] = match surface % 6 {
            0 => [0.25, 0.25],
            1 => [0.50, 0.25],
            2 => [0.00, 0.75],
            3 => [0.75, 0.25],
            4 => [0.00, 0.25],
            5 => [0.00, 0.00],
            _ => unreachable!(),
        };

        let [w, h] = match surface % 6 {
            0 => [0.25, 0.50],
            1 => [0.25, 0.50],
            2 => [1.00, 0.25],
            3 => [0.25, 0.50],
            4 => [0.25, 0.50],
            5 => [1.00, 0.25],
            _ => unreachable!(),
        };

        [
            coord[0].clamp(-1.0, 1.0) * (w / 2.0) + ox + (w / 2.0),
            coord[1].clamp(-1.0, 1.0) * (h / 2.0) + oy + (h / 2.0),
        ]
    }
}
