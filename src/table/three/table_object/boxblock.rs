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
                                mesh.texture_id = texture_id;
                            }
                        } else {
                            mesh.boxblock_material.set_map(None);
                            mesh.boxblock_material.set_needs_update(true);
                            mesh.texture_id = texture_id;
                        }
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
            box_geometry: util::block_geometry::box_geometry(&mut crate::lazy),
            cylinder_geometry: util::block_geometry::cylinder_geometry(&mut crate::lazy),
            icosahedron_geometry: util::block_geometry::icosahedron_geometry(&mut crate::lazy),
            slope_geometry: util::block_geometry::slope_geometry(&mut crate::lazy),
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
}
