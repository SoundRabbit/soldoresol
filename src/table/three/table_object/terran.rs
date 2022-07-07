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
        scene: &three::Scene,
        craftboards: impl Iterator<Item = BlockRef<block::Craftboard>>,
    ) {
        let mut unused = self.meshs.keys().map(U128Id::clone).collect::<HashSet<_>>();

        for craftboard in craftboards {
            let craftboard_id = craftboard.id();
            craftboard.map(|craftboard| {
                let terran = craftboard.terran();
                let terran_id = terran.id();
                let timestamp = terran.timestamp();
                unused.remove(&terran_id);

                terran.map(|terran| {
                    if !self.meshs.contains_key(&terran_id) {
                        let data = TerranData::new(timestamp, &terran);
                        data.set_user_data(&craftboard_id.to_jsvalue());
                        scene.add(&data);

                        self.meshs.insert(U128Id::clone(&terran_id), data);
                    }

                    if let Some(mesh) = self.meshs.get_mut(&terran_id) {
                        if mesh.timestamp < timestamp {
                            mesh.update_blocks(timestamp, terran);
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
    fn new(timestamp: f64, terran: &block::Terran) -> Self {
        let geometry = Self::create_geometry(terran);
        let material = three::MeshStandardMaterial::new(&object! {
            "vertexColors": true
        });

        let data = three::Mesh::new(&geometry, &material);

        Self {
            timestamp,
            data,
            material,
        }
    }

    fn update_blocks(&mut self, timestamp: f64, terran: &block::Terran) {
        self.data.set_geometry(&Self::create_geometry(terran));
        self.timestamp = timestamp;
    }

    fn create_geometry(terran: &block::Terran) -> three::BufferGeometry {
        let mut points = vec![];
        let mut normals = vec![];
        let mut colors = vec![];
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
            let color = terran_block.color().to_color().to_f64array();
            let color = [color[0] as f32, color[1] as f32, color[2] as f32];

            for i in 0..6 {
                if !terran.blocks().contains_key(&[
                    position[0] + k[i][0],
                    position[1] + k[i][1],
                    position[2] + k[i][2],
                ]) {
                    Self::append_surface(
                        &position,
                        &color,
                        &o[i],
                        &n[i],
                        &mut index_num,
                        &mut points,
                        &mut normals,
                        &mut colors,
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
            "color",
            &three::BufferAttribute::new_with_f32array(
                &js_sys::Float32Array::from(colors.concat().as_slice()),
                3,
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
        color: &[f32; 3],
        offset: &[[f32; 3]; 4],
        normal: &[f32; 3],
        index_num: &mut u16,
        points: &mut Vec<[f32; 3]>,
        normals: &mut Vec<[f32; 3]>,
        colors: &mut Vec<[f32; 3]>,
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

        colors.push(color.clone());
        colors.push(color.clone());
        colors.push(color.clone());
        colors.push(color.clone());

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
