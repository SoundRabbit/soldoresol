use super::super::TextureTable;
use super::util;
use crate::arena::{block, resource, BlockRef};
use crate::libs::color::Pallet;
use crate::libs::random_id::U128Id;
use crate::libs::three;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::JsCast;

pub struct Textboard {
    meshs: HashMap<U128Id, Mesh>,
    geometry_nameplate: util::nameplate::XZGeometry,
}

struct Mesh {
    nameplate_id: (String, String),
    color: Pallet,
    tex_size: [f64; 2],
    data: util::Nameplate,
}

impl Textboard {
    pub fn new() -> Self {
        Self {
            meshs: HashMap::new(),
            geometry_nameplate: util::nameplate::XZGeometry::new(-0.5, false),
        }
    }

    pub fn update(
        &mut self,
        texture_table: &mut TextureTable,
        scene: &three::Scene,
        textboards: impl Iterator<Item = BlockRef<block::Textboard>>,
    ) {
        let mut unused = self.meshs.keys().map(U128Id::clone).collect::<HashSet<_>>();

        for textboard in textboards {
            let textboard_id = textboard.id();
            unused.remove(&textboard_id);
            textboard.map(|textboard| {
                let color = textboard.color();

                if !self.meshs.contains_key(&textboard_id) {
                    let nameplate = util::Nameplate::new(&self.geometry_nameplate);
                    nameplate.set_user_data(&textboard_id.to_jsvalue());
                    nameplate.set_color(color);

                    scene.add(&nameplate);

                    self.meshs.insert(
                        U128Id::clone(&textboard_id),
                        Mesh {
                            nameplate_id: (String::from(""), String::from("")),
                            tex_size: [0.0, 0.0],
                            color: color.clone(),
                            data: nameplate,
                        },
                    );
                }

                if let Some(mesh) = self.meshs.get_mut(&textboard_id) {
                    if *textboard.title() != mesh.nameplate_id.1
                        || *textboard.text() != mesh.nameplate_id.0
                    {
                        let nameplate_id = (textboard.text().clone(), textboard.title().clone());
                        let texture = texture_table.load_text(&nameplate_id);
                        mesh.data.text().set_alpha_map(Some(&texture.data));
                        mesh.data.text().set_needs_update(true);

                        mesh.nameplate_id = nameplate_id;
                        mesh.tex_size = [texture.size[0], texture.size[1]];
                    }

                    if *color != mesh.color {
                        mesh.data.set_color(color);
                        mesh.color = color.clone();
                    }
                    let mesh_size = [
                        mesh.tex_size[0] * textboard.font_size(),
                        mesh.tex_size[1] * textboard.font_size(),
                    ];

                    let [px, py, pz] = textboard.position().clone();
                    let size = textboard.size();
                    let [sx, sz] = [size[0].max(mesh_size[0]), size[1].max(mesh_size[1])];
                    mesh.data.position().set(px, py, pz + sz);
                    mesh.data.data_background().scale().set(sx, 1.0, sz);
                    mesh.data
                        .data_text()
                        .scale()
                        .set(mesh_size[0], 1.0, mesh_size[1]);
                }
            });
        }

        for unused_textboard_id in unused {
            if let Some(mesh) = self.meshs.remove(&unused_textboard_id) {
                scene.remove(&mesh.data);
            }
        }
    }
}
