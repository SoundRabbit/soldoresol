use super::super::TextureTable;
use super::util;
use crate::arena::{block, resource, BlockRef};
use crate::libs::random_id::U128Id;
use crate::libs::three;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::JsCast;

pub struct Character {
    meshs: HashMap<U128Id, Mesh>,
    geometry_base: util::RoundedRectangleGeometry,
}

pub struct Mesh {
    base_data: util::RoundedRectangleMesh,
    data: three::Group,
}

impl Character {
    pub fn new() -> Self {
        Self {
            meshs: HashMap::new(),
            geometry_base: util::RoundedRectangleGeometry::new(),
        }
    }

    pub fn update(
        &mut self,
        texture_table: &mut TextureTable,
        scene: &three::Scene,
        characters: impl Iterator<Item = BlockRef<block::Character>>,
    ) {
        let mut unused = self.meshs.keys().map(U128Id::clone).collect::<HashSet<_>>();

        for character in characters {
            let character_id = character.id();
            unused.remove(&character_id);

            character.map(|character| {
                if !self.meshs.contains_key(&character_id) {
                    let base_data = util::RoundedRectangleMesh::new(&self.geometry_base);
                    base_data.set_user_data(&character_id.to_jsvalue());

                    let data = three::Group::new();
                    data.set_render_order(super::ORDER_CHARACTER);
                    data.set_user_data(&character_id.to_jsvalue());
                    data.add(base_data.data());
                    scene.add(&data);
                    self.meshs
                        .insert(U128Id::clone(&character_id), Mesh { base_data, data });
                }

                if let Some(mesh) = self.meshs.get_mut(&character_id) {
                    let s = character.size();
                    mesh.base_data.set_scale(&[s - 0.1, s - 0.1], 0.1);
                    mesh.base_data.material().color().set_rgb(0.0, 0.0, 0.0);

                    let [px, py, pz] = character.position().clone();
                    mesh.data.position().set(px, py, pz);
                }
            });
        }

        for unused_character_id in unused {
            if let Some(mesh) = self.meshs.get(&unused_character_id) {
                scene.remove(&mesh.data);
            }
        }
    }
}
