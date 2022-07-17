use super::super::TextureTable;
use super::util;
use crate::arena::{block, resource, BlockRef};
use crate::libs::random_id::U128Id;
use crate::libs::three;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::JsCast;

pub struct Terran {
    meshs: HashMap<U128Id, TerranData>,
}

struct TerranData {
    timestamp: f64,
    tex_timestamp: f64,
    material: three::MeshStandardMaterial,
    data: three::Mesh,
}

impl Terran {
    pub fn new() -> Self {
        Self {
            meshs: HashMap::new(),
        }
    }

    pub fn update(
        &mut self,
        texture_table: &mut TextureTable,
        scene: &three::Scene,
        craftboards: impl Iterator<Item = BlockRef<block::Craftboard>>,
    ) {
        let mut unused = self.meshs.keys().map(U128Id::clone).collect::<HashSet<_>>();

        for craftboard in craftboards {
            let craftboard_id = craftboard.id();
            craftboard.map(|craftboard| {
                let terran = craftboard.terran();
                let timestamp = terran.timestamp();
                unused.remove(&craftboard_id);

                terran.map(|terran| {
                    if !self.meshs.contains_key(&craftboard_id) {
                        let data = TerranData::new(texture_table, timestamp, &terran);
                        data.set_user_data(&craftboard_id.to_jsvalue());
                        scene.add(&data);

                        self.meshs.insert(U128Id::clone(&craftboard_id), data);
                    }

                    if let Some(mesh) = self.meshs.get_mut(&craftboard_id) {
                        if mesh.timestamp < timestamp {
                            mesh.update_blocks(texture_table, timestamp, terran);
                        }

                        let cp = craftboard.position();
                        let cs = craftboard.size();
                        mesh.position().set(
                            cp[0] + (cs[0].rem_euclid(2.0) - 1.0) * 0.5,
                            cp[1] + (cs[1].rem_euclid(2.0) - 1.0) * 0.5,
                            cp[2] + 0.5,
                        );
                    }
                });
            });
        }

        for unused_boxblock_id in unused {
            if let Some(mesh) = self.meshs.remove(&unused_boxblock_id) {
                scene.remove(&mesh);
            }
        }
    }
}

impl TerranData {
    fn new(texture_table: &mut TextureTable, timestamp: f64, terran: &block::Terran) -> Self {
        let geometry = Self::create_geometry(terran);
        let material = three::MeshStandardMaterial::new(&object! {});
        material.color().set_rgb(1.0, 1.0, 1.0);

        if let Some(texture) = texture_table.load_terran(terran.texture().as_ref()) {
            material.set_map(Some(&texture));
        } else {
            material.set_map(None);
        }

        let data = three::Mesh::new(&geometry, &material);

        Self {
            timestamp,
            tex_timestamp: 0.0,
            data,
            material,
        }
    }

    fn update_blocks(
        &mut self,
        texture_table: &mut TextureTable,
        timestamp: f64,
        terran: &block::Terran,
    ) {
        self.data.set_geometry(&Self::create_geometry(terran));
        if let Some(texture) = texture_table.load_terran(terran.texture().as_ref()) {
            self.material.set_map(Some(&texture));
        } else {
            self.material.set_map(None);
        }
        self.material.set_needs_update(true);
        self.timestamp = timestamp;
    }

    fn create_geometry(terran: &block::Terran) -> three::BufferGeometry {
        let mut points = vec![];
        let mut normals = vec![];
        let mut uvs = vec![];
        let mut indexs = vec![];
        let mut index_num = 0;

        let k = [
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1],
            [-1, 0, 0],
            [0, -1, 0],
            [0, 0, -1],
        ];
        let uv = [
            [
                super::boxblock::Geometry::texture_coord(0, &[1.0, 1.0]),
                super::boxblock::Geometry::texture_coord(0, &[-1.0, 1.0]),
                super::boxblock::Geometry::texture_coord(0, &[-1.0, -1.0]),
                super::boxblock::Geometry::texture_coord(0, &[1.0, -1.0]),
            ],
            [
                super::boxblock::Geometry::texture_coord(1, &[1.0, 1.0]),
                super::boxblock::Geometry::texture_coord(1, &[-1.0, 1.0]),
                super::boxblock::Geometry::texture_coord(1, &[-1.0, -1.0]),
                super::boxblock::Geometry::texture_coord(1, &[1.0, -1.0]),
            ],
            [
                super::boxblock::Geometry::texture_coord(2, &[1.0, 1.0]),
                super::boxblock::Geometry::texture_coord(2, &[-1.0, 1.0]),
                super::boxblock::Geometry::texture_coord(2, &[-1.0, -1.0]),
                super::boxblock::Geometry::texture_coord(2, &[1.0, -1.0]),
            ],
            [
                super::boxblock::Geometry::texture_coord(3, &[1.0, 1.0]),
                super::boxblock::Geometry::texture_coord(3, &[-1.0, 1.0]),
                super::boxblock::Geometry::texture_coord(3, &[-1.0, -1.0]),
                super::boxblock::Geometry::texture_coord(3, &[1.0, -1.0]),
            ],
            [
                super::boxblock::Geometry::texture_coord(4, &[1.0, 1.0]),
                super::boxblock::Geometry::texture_coord(4, &[-1.0, 1.0]),
                super::boxblock::Geometry::texture_coord(4, &[-1.0, -1.0]),
                super::boxblock::Geometry::texture_coord(4, &[1.0, -1.0]),
            ],
            [
                super::boxblock::Geometry::texture_coord(5, &[1.0, 1.0]),
                super::boxblock::Geometry::texture_coord(5, &[-1.0, 1.0]),
                super::boxblock::Geometry::texture_coord(5, &[-1.0, -1.0]),
                super::boxblock::Geometry::texture_coord(5, &[1.0, -1.0]),
            ],
        ];
        let o = [
            [
                [0.5, 0.5, 0.5],
                [0.5, -0.5, 0.5],
                [0.5, -0.5, -0.5],
                [0.5, 0.5, -0.5],
            ],
            [
                [-0.5, 0.5, 0.5],
                [0.5, 0.5, 0.5],
                [0.5, 0.5, -0.5],
                [-0.5, 0.5, -0.5],
            ],
            [
                [0.5, 0.5, 0.5],
                [-0.5, 0.5, 0.5],
                [-0.5, -0.5, 0.5],
                [0.5, -0.5, 0.5],
            ],
            [
                [-0.5, -0.5, 0.5],
                [-0.5, 0.5, 0.5],
                [-0.5, 0.5, -0.5],
                [-0.5, -0.5, -0.5],
            ],
            [
                [0.5, -0.5, 0.5],
                [-0.5, -0.5, 0.5],
                [-0.5, -0.5, -0.5],
                [0.5, -0.5, -0.5],
            ],
            [
                [0.5, -0.5, -0.5],
                [-0.5, -0.5, -0.5],
                [-0.5, 0.5, -0.5],
                [0.5, 0.5, -0.5],
            ],
        ];
        let n = [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [-1.0, 0.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, 0.0, -1.0],
        ];

        for (position, terran_block) in terran.blocks() {
            for i in 0..6 {
                if !terran.blocks().contains_key(&[
                    position[0] + k[i][0],
                    position[1] + k[i][1],
                    position[2] + k[i][2],
                ]) {
                    Self::append_surface(
                        &position,
                        &uv[i],
                        &o[i],
                        &n[i],
                        terran_block.tex_idx(),
                        &mut index_num,
                        &mut points,
                        &mut normals,
                        &mut uvs,
                        &mut indexs,
                    );
                }
            }
        }

        let geometry = three::BufferGeometry::new();
        geometry.set_attribute(
            "position",
            &three::BufferAttribute::new_with_f32array(
                &js_sys::Float32Array::from(points.concat().as_slice()),
                3,
                false,
            ),
        );
        geometry.set_attribute(
            "uv",
            &three::BufferAttribute::new_with_f32array(
                &js_sys::Float32Array::from(uvs.concat().as_slice()),
                2,
                false,
            ),
        );
        geometry.set_attribute(
            "normal",
            &three::BufferAttribute::new_with_f32array(
                &js_sys::Float32Array::from(normals.concat().as_slice()),
                3,
                false,
            ),
        );
        geometry.set_index(&three::BufferAttribute::new_with_u16array(
            &js_sys::Uint16Array::from(indexs.as_slice()),
            1,
            false,
        ));

        geometry
    }

    fn append_surface(
        position: &[i32; 3],
        uv: &[[f32; 2]; 4],
        offset: &[[f32; 3]; 4],
        normal: &[f32; 3],
        tex_idx: u32,
        index_num: &mut u16,
        points: &mut Vec<[f32; 3]>,
        normals: &mut Vec<[f32; 3]>,
        uvs: &mut Vec<[f32; 2]>,
        indexs: &mut Vec<u16>,
    ) {
        let p = position;
        let o = offset;
        points.push([
            p[0] as f32 + o[0][0],
            p[1] as f32 + o[0][1],
            p[2] as f32 + o[0][2],
        ]);
        points.push([
            p[0] as f32 + o[1][0],
            p[1] as f32 + o[1][1],
            p[2] as f32 + o[1][2],
        ]);
        points.push([
            p[0] as f32 + o[2][0],
            p[1] as f32 + o[2][1],
            p[2] as f32 + o[2][2],
        ]);
        points.push([
            p[0] as f32 + o[3][0],
            p[1] as f32 + o[3][1],
            p[2] as f32 + o[3][2],
        ]);

        normals.push(normal.clone());
        normals.push(normal.clone());
        normals.push(normal.clone());
        normals.push(normal.clone());

        uvs.push(block::TerranTexture::uv_f32(tex_idx, &uv[0]));
        uvs.push(block::TerranTexture::uv_f32(tex_idx, &uv[1]));
        uvs.push(block::TerranTexture::uv_f32(tex_idx, &uv[2]));
        uvs.push(block::TerranTexture::uv_f32(tex_idx, &uv[3]));

        let i = *index_num;
        indexs.push(i + 0);
        indexs.push(i + 1);
        indexs.push(i + 2);
        indexs.push(i + 2);
        indexs.push(i + 3);
        indexs.push(i + 0);

        *index_num += 4;
    }
}

impl std::ops::Deref for TerranData {
    type Target = three::Mesh;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
