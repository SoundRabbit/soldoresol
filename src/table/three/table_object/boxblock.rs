use super::super::TextureTable;
use super::util;
use crate::arena::{block, resource, BlockRef};
use crate::libs::random_id::U128Id;
use crate::libs::three;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::JsCast;

pub struct Boxblock {
    meshs: HashMap<U128Id, Mesh>,
    geometry: Geometry,
    geometry_nameplate: util::nameplate::XZGeometry,
}

pub struct Geometry {
    box_geometry: three::BoxGeometry,
    cylinder_geometry: three::CylinderGeometry,
    icosahedron_geometry: three::IcosahedronGeometry,
    slope_geometry: three::BufferGeometry,
}

struct Mesh {
    boxblock_material: three::MeshStandardMaterial,
    boxblock_data: three::Mesh,
    texture_id: U128Id,

    nameplate: util::Nameplate,
    nameplate_id: (String, String),

    color: crate::libs::color::Pallet,
    data: three::Group,
}

impl Boxblock {
    pub fn new() -> Self {
        Self {
            meshs: HashMap::new(),
            geometry: Geometry::new(),
            geometry_nameplate: util::nameplate::XZGeometry::new(0.5, true),
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
                    let boxblock_material = three::MeshStandardMaterial::new(&object! {});
                    let [r, g, b, ..] = boxblock.color().to_color().to_f64array();
                    boxblock_material.color().set_rgb(r, g, b);
                    let boxblock_data = three::Mesh::new(
                        self.geometry.get_geometry(boxblock.shape()),
                        &boxblock_material,
                    );
                    boxblock_data.set_user_data(&boxblock_id.to_jsvalue());

                    let nameplate = util::Nameplate::new(&self.geometry_nameplate);
                    nameplate.set_color(boxblock.color());
                    nameplate.scale().set(1.0, 1.0, 1.0);
                    nameplate.board().scale().set(0.0, 1.0, 0.0);
                    nameplate.arrow().unwrap().scale().set(0.0, 0.0, 0.0);

                    let data = three::Group::new();
                    data.add(&boxblock_data);
                    data.add(&nameplate);
                    data.set_render_order(super::ORDER_BOXBLOCK);
                    scene.add(&data);
                    self.meshs.insert(
                        U128Id::clone(&boxblock_id),
                        Mesh {
                            boxblock_material,
                            boxblock_data,
                            texture_id: U128Id::none(),
                            nameplate,
                            nameplate_id: (String::from(""), String::from("")),
                            color: boxblock.color().clone(),
                            data,
                        },
                    );
                }
                if let Some(mesh) = self.meshs.get_mut(&boxblock_id) {
                    mesh.boxblock_data
                        .set_geometry(self.geometry.get_geometry(boxblock.shape()));
                    let [px, py, pz] = boxblock.position().clone();
                    mesh.boxblock_data.position().set(px, py, pz);
                    let [sx, sy, sz] = boxblock.size().clone();
                    mesh.boxblock_data.scale().set(sx, sy, sz);
                    mesh.nameplate.position().set(px, py, pz + sz * 0.5);

                    let texture = boxblock.texture();
                    let texture_id = texture
                        .as_ref()
                        .map(|texture| texture.id())
                        .unwrap_or_else(|| U128Id::none());
                    if texture_id != mesh.texture_id {
                        if let Some(texture) = texture {
                            if let Some(texture) =
                                texture_table.load_block(BlockRef::clone(&texture))
                            {
                                mesh.boxblock_material.set_map(Some(&texture));
                                mesh.boxblock_material.color().set_rgb(1.0, 1.0, 1.0);
                                mesh.boxblock_material.set_needs_update(true);
                            }
                        } else {
                            mesh.boxblock_material.set_map(None);
                            mesh.boxblock_material.set_needs_update(true);
                        }
                        mesh.texture_id = texture_id;
                    }

                    if *boxblock.color() != mesh.color {
                        if mesh.texture_id.is_none() {
                            let [r, g, b, ..] = boxblock.color().to_color().to_f64array();
                            mesh.boxblock_material.color().set_rgb(r, g, b);
                            mesh.boxblock_material.set_needs_update(true);
                        }
                        mesh.nameplate.set_color(boxblock.color());
                        mesh.color = boxblock.color().clone();
                    }

                    if *boxblock.display_name() != mesh.nameplate_id {
                        let texture = texture_table.load_text(boxblock.display_name());
                        mesh.nameplate.text().set_alpha_map(Some(&texture.data));
                        mesh.nameplate.text().set_needs_update(true);

                        let texture_width = texture.size[0] * 0.5;
                        let texture_height = texture_width * texture.size[1] / texture.size[0];
                        mesh.nameplate
                            .board()
                            .scale()
                            .set(texture_width, 1.0, texture_height);
                        if boxblock.display_name().0 == "" && boxblock.display_name().1 == "" {
                            mesh.nameplate.arrow().unwrap().scale().set(0.0, 0.0, 0.0);
                        } else if mesh.nameplate_id.0 == "" && mesh.nameplate_id.1 == "" {
                            mesh.nameplate.arrow().unwrap().scale().set(1.0, 1.0, 1.0);
                        }
                        mesh.nameplate_id = boxblock.display_name().clone();
                    }
                }
            });
        }

        for unused_boxblock_id in unused {
            if let Some(mesh) = self.meshs.remove(&unused_boxblock_id) {
                scene.remove(&mesh.data);
            }
        }
    }
}

impl Geometry {
    pub fn new() -> Self {
        Self {
            box_geometry: Self::create_box_geometry(),
            cylinder_geometry: Self::create_cylinder_geometry(),
            icosahedron_geometry: Self::create_icosahedron_geometry(),
            slope_geometry: Self::create_slope_geometry(),
        }
    }

    pub fn get_geometry(&self, shape: block::boxblock::Shape) -> &three::BufferGeometry {
        match shape {
            block::boxblock::Shape::Cube => &self.box_geometry,
            block::boxblock::Shape::Cylinder => &self.cylinder_geometry,
            block::boxblock::Shape::Sphere => &self.icosahedron_geometry,
            block::boxblock::Shape::Slope => &self.slope_geometry,
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
        let geometry = three::CylinderGeometry::new(0.5, 0.5, 1.0, 48);

        let a_position = geometry
            .get_attribute("position")
            .array_as_f32array()
            .to_vec();
        let a_normal = geometry
            .get_attribute("normal")
            .array_as_f32array()
            .to_vec();
        let a_uv = geometry.get_attribute("uv").array_as_f32array().to_vec();
        let mut position = vec![];
        let mut normal = vec![];
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

            position.push(p_x);
            position.push(-p_z);
            position.push(p_y);

            normal.push(n_x);
            normal.push(-n_z);
            normal.push(n_y);

            uv.push(u);
            uv.push(v);
        }

        geometry.set_attribute(
            "position",
            &three::BufferAttribute::new_with_f32array(
                &js_sys::Float32Array::from(position.as_slice()),
                3,
                false,
            ),
        );

        geometry.set_attribute(
            "normal",
            &three::BufferAttribute::new_with_f32array(
                &js_sys::Float32Array::from(normal.as_slice()),
                3,
                false,
            ),
        );

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

    fn create_icosahedron_geometry() -> three::IcosahedronGeometry {
        let geometry = three::IcosahedronGeometry::new(0.5, 5);

        util::stand(&geometry, 0.0);

        geometry
    }

    fn create_slope_geometry() -> three::BufferGeometry {
        let points = js_sys::Float32Array::from(
            [
                // PY
                [0.5, 0.5, -0.5],
                [-0.5, 0.5, -0.5],
                [-0.5, 0.5, 0.5],
                // NY
                [-0.5, -0.5, 0.5],
                [-0.5, -0.5, -0.5],
                [0.5, -0.5, -0.5],
                // NX
                [-0.5, 0.5, -0.5],
                [-0.5, -0.5, -0.5],
                [-0.5, -0.5, 0.5],
                [-0.5, 0.5, 0.5],
                // NZ
                [0.5, -0.5, -0.5],
                [-0.5, -0.5, -0.5],
                [-0.5, 0.5, -0.5],
                [0.5, 0.5, -0.5],
                // 斜面
                [-0.5, 0.5, 0.5],
                [-0.5, -0.5, 0.5],
                [0.5, -0.5, -0.5],
                [0.5, 0.5, -0.5],
            ]
            .concat()
            .as_slice(),
        );
        let normal = js_sys::Float32Array::from(
            [
                // PY
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                // NY
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                // NX
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                // NZ
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                // 斜面
                [1.0, 0.0, 1.0],
                [1.0, 0.0, 1.0],
                [1.0, 0.0, 1.0],
                [1.0, 0.0, 1.0],
            ]
            .concat()
            .as_slice(),
        );
        let uv = js_sys::Float32Array::from(
            [
                //PY
                Self::texture_coord(1, &[-1.0, -1.0]),
                Self::texture_coord(1, &[1.0, -1.0]),
                Self::texture_coord(1, &[1.0, 1.0]),
                //NY
                Self::texture_coord(4, &[-1.0, 1.0]),
                Self::texture_coord(4, &[-1.0, -1.0]),
                Self::texture_coord(4, &[1.0, -1.0]),
                // NX
                Self::texture_coord(3, &[1.0, -1.0]),
                Self::texture_coord(3, &[-1.0, -1.0]),
                Self::texture_coord(3, &[-1.0, 1.0]),
                Self::texture_coord(3, &[1.0, 1.0]),
                // NZ
                Self::texture_coord(3, &[1.0, 1.0]),
                Self::texture_coord(3, &[-1.0, 1.0]),
                Self::texture_coord(3, &[-1.0, -1.0]),
                Self::texture_coord(3, &[1.0, -1.0]),
                // 斜面
                Self::texture_coord(2, &[-1.0, 1.0]),
                Self::texture_coord(2, &[-1.0, -1.0]),
                Self::texture_coord(2, &[1.0, -1.0]),
                Self::texture_coord(2, &[1.0, 1.0]),
            ]
            .concat()
            .as_slice(),
        );
        let index = js_sys::Uint16Array::from(
            [
                vec![0, 1, 2],
                vec![3, 4, 5],
                vec![6, 7, 8, 8, 9, 6],
                vec![10, 11, 12, 12, 13, 10],
                vec![14, 15, 16, 16, 17, 14],
            ]
            .concat()
            .as_slice(),
        );

        let geometry = three::BufferGeometry::new();

        geometry.set_attribute(
            "position",
            &three::BufferAttribute::new_with_f32array(&points, 3, false),
        );
        geometry.set_attribute(
            "normal",
            &three::BufferAttribute::new_with_f32array(&normal, 3, true),
        );
        geometry.set_attribute(
            "uv",
            &three::BufferAttribute::new_with_f32array(&uv, 2, false),
        );
        geometry.set_index(&three::BufferAttribute::new_with_u16array(&index, 1, false));

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
    pub fn texture_coord(surface: usize, coord: &[f32; 2]) -> [f32; 2] {
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
