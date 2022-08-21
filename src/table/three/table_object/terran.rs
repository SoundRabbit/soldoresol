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
                        let vd = craftboard.voxel_density();
                        mesh.scale().set(1.0 / vd[0], 1.0 / vd[1], 1.0 / vd[2]);
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
        let mut positions = vec![];
        let mut normals = vec![];
        let mut uvs = vec![];
        let mut indexs = vec![];
        let mut index_offset = 0;

        for (block_position, terran_block) in terran.blocks() {
            let geometry = util::block_geometry::box_geometry(&mut |uv| {
                block::TerranTexture::uv_f32(terran_block.tex_idx(), &uv)
            });
            let a_position = geometry
                .get_attribute("position")
                .array_as_f32array()
                .to_vec();
            let mut a_normal = geometry
                .get_attribute("normal")
                .array_as_f32array()
                .to_vec();
            let mut a_uv = geometry.get_attribute("uv").array_as_f32array().to_vec();
            let a_index = geometry.get_index().array_as_u16array().to_vec();

            let n = (a_position.len() / 3) as u16;
            positions.append(
                &mut a_position
                    .into_iter()
                    .enumerate()
                    .map(|(idx, position)| position + block_position[idx % 3] as f32)
                    .collect::<Vec<_>>(),
            );
            normals.append(&mut a_normal);
            uvs.append(&mut a_uv);
            indexs.append(
                &mut a_index
                    .into_iter()
                    .map(|idx| idx + index_offset)
                    .collect::<Vec<_>>(),
            );
            index_offset += n;
        }

        let geometry = three::BufferGeometry::new();
        geometry.set_attribute(
            "position",
            &three::BufferAttribute::new_with_f32array(
                &js_sys::Float32Array::from(positions.as_slice()),
                3,
                false,
            ),
        );
        geometry.set_attribute(
            "uv",
            &three::BufferAttribute::new_with_f32array(
                &js_sys::Float32Array::from(uvs.as_slice()),
                2,
                false,
            ),
        );
        geometry.set_attribute(
            "normal",
            &three::BufferAttribute::new_with_f32array(
                &js_sys::Float32Array::from(normals.as_slice()),
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
}

impl std::ops::Deref for TerranData {
    type Target = three::Mesh;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
