use super::super::TextureTable;
use super::util;
use crate::arena::{block, resource, BlockRef};
use crate::libs::random_id::U128Id;
use crate::libs::three;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::JsCast;

pub struct Character {
    meshs: HashMap<U128Id, Mesh>,
    geometry_border: util::BasicRoundedRectangleGeometry,
    geometry_base: three::PlaneGeometry,
    geometry_texture: three::BufferGeometry,
    geometry_nameplate: util::nameplate::XZGeometry,
    geometry_offset_value: util::nameplate::XZGeometry,
    geometry_offset_line: three::CylinderGeometry,
    material_border: three::MeshBasicMaterial,
    material_base: three::MeshBasicMaterial,
    material_offset_line: three::LineBasicMaterial,
}

pub struct Mesh {
    offset_line: three::Mesh,
    offset_base: three::Mesh,
    offset_border: util::RoundedRectangleMesh,
    offset_value: util::Nameplate,

    border_data: util::RoundedRectangleMesh,
    base_data: three::Mesh,

    texture_material: three::MeshBasicMaterial,
    texture_data: three::Mesh,
    texture_id: U128Id,

    nameplate: util::Nameplate,
    nameplate_id: (String, String),

    color: crate::libs::color::Pallet,

    z_offset: f64,

    data: three::Group,
}

impl Character {
    pub fn new() -> Self {
        let color_border = crate::libs::color::Pallet::blue(7).to_color().to_f64array();
        let color_base = crate::libs::color::Pallet::blue(5).to_color().to_f64array();
        Self {
            meshs: HashMap::new(),

            geometry_border: util::BasicRoundedRectangleGeometry::new(),
            geometry_base: three::PlaneGeometry::new(1.0, 1.0),
            geometry_texture: Self::create_texture_geometry(),
            geometry_nameplate: util::nameplate::XZGeometry::new(0.5, true),
            geometry_offset_value: util::nameplate::XZGeometry::new_with_offset(
                &[0.5, 0.0, 0.0],
                false,
            ),
            geometry_offset_line: Self::create_offset_line_geometry(),

            material_border: three::MeshBasicMaterial::new(&object! {
                "color": &three::Color::new(color_border[0], color_border[1], color_border[2])
            }),
            material_base: three::MeshBasicMaterial::new(&object! {
                "color": &three::Color::new(color_base[0], color_base[1], color_base[2]),
                "opacity": 0.25,
                "transparent": true
            }),
            material_offset_line: three::LineBasicMaterial::new(&object! {
                "color": &three::Color::new(color_base[0], color_base[1], color_base[2]),
            }),
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
                    let border_data = util::RoundedRectangleMesh::new(
                        &self.geometry_border,
                        &self.material_border,
                    );
                    border_data.set_user_data(&character_id.to_jsvalue());

                    let base_data = three::Mesh::new(&self.geometry_base, &self.material_base);
                    base_data.set_user_data(&character_id.to_jsvalue());

                    let texture_material = three::MeshBasicMaterial::new(&object! {
                        "transparent": true
                    });
                    texture_material.set_side(three::DOUBLE_SIDE);
                    let texture_data = three::Mesh::new(&self.geometry_texture, &texture_material);
                    texture_data.set_user_data(&character_id.to_jsvalue());
                    texture_data.scale().set(0.0, 0.0, 0.0);

                    let nameplate = util::Nameplate::new(&self.geometry_nameplate);
                    nameplate.set_color(character.color());

                    let offset_line =
                        three::Mesh::new(&self.geometry_offset_line, &self.material_offset_line);
                    offset_line.set_visible(false);

                    let offset_base = three::Mesh::new(&self.geometry_base, &self.material_base);
                    offset_base.set_user_data(&character_id.to_jsvalue());
                    offset_base.set_visible(false);

                    let offset_border = util::RoundedRectangleMesh::new(
                        &self.geometry_border,
                        &self.material_border,
                    );
                    offset_border.set_user_data(&character_id.to_jsvalue());
                    offset_border.data().set_visible(false);

                    let offset_value = util::Nameplate::new(&self.geometry_offset_value);
                    offset_value.set_color(&crate::libs::color::Pallet::gray(9));
                    offset_value.set_visible(false);

                    let data = three::Group::new();
                    data.set_render_order(super::ORDER_CHARACTER);
                    data.set_user_data(&character_id.to_jsvalue());
                    data.add(border_data.data());
                    data.add(&base_data);
                    data.add(&texture_data);
                    data.add(&nameplate);
                    data.add(&offset_line);
                    data.add(&offset_base);
                    data.add(offset_border.data());
                    data.add(&offset_value);
                    scene.add(&data);
                    self.meshs.insert(
                        U128Id::clone(&character_id),
                        Mesh {
                            offset_line,
                            offset_base,
                            offset_border,
                            offset_value,
                            border_data,
                            base_data,
                            texture_material,
                            texture_data,
                            texture_id: U128Id::none(),
                            nameplate,
                            nameplate_id: (String::from(""), String::from("")),
                            color: character.color().clone(),
                            z_offset: 0.0,
                            data,
                        },
                    );
                }

                if let Some(mesh) = self.meshs.get_mut(&character_id) {
                    let s = character.size();
                    mesh.border_data.set_scale(&[s - 0.1, s - 0.1], 0.1);
                    mesh.base_data.scale().set(s - 0.1, s - 0.1, 1.0);
                    let [px, py, pz] = character.position().clone();
                    mesh.data
                        .position()
                        .set(px, py, pz + 0.01 + character.z_offset());

                    if mesh.z_offset != character.z_offset() {
                        let z_offset = character.z_offset();
                        if z_offset > 0.0 {
                            mesh.offset_line.scale().set(1.0, 1.0, z_offset);
                            mesh.offset_base.scale().set(s - 0.1, s - 0.1, 1.0);
                            mesh.offset_border.set_scale(&[s - 0.1, s - 0.1], 0.1);
                            mesh.offset_line.position().set(0.0, 0.0, -z_offset);
                            mesh.offset_base.position().set(0.0, 0.0, -z_offset);
                            mesh.border_data.data().position().set(0.0, 0.0, -z_offset);
                            mesh.offset_line.set_visible(true);
                            mesh.offset_base.set_visible(true);
                            mesh.offset_border.data().set_visible(true);

                            let texture = texture_table
                                .load_text(&(format!("+{}ﾏｽ", z_offset), String::new()));
                            mesh.offset_value.text().set_alpha_map(Some(&texture.data));
                            mesh.offset_value.board().scale().set(
                                texture.size[0] * 0.25,
                                1.0,
                                texture.size[1] * 0.25,
                            );
                            mesh.offset_value.position().set(s * 0.5 + 0.1, 0.0, 0.0);
                            mesh.offset_value.set_visible(true);
                        } else {
                            mesh.offset_line.set_visible(false);
                            mesh.offset_base.set_visible(false);
                            mesh.offset_border.data().set_visible(false);
                            mesh.offset_value.set_visible(false);
                            mesh.border_data.data().position().set(0.0, 0.0, -z_offset);
                        }
                        mesh.z_offset = z_offset;
                    }

                    let texture_block = character
                        .selected_texture()
                        .as_ref()
                        .and_then(|texture| texture.image());
                    let texture_id = texture_block
                        .as_ref()
                        .map(|texture_block| texture_block.id())
                        .unwrap_or_else(|| U128Id::none());

                    if texture_id != mesh.texture_id {
                        if let Some(texture_block) = texture_block {
                            if let Some(texture) =
                                texture_table.load_image(BlockRef::clone(&texture_block))
                            {
                                mesh.texture_material.set_map(Some(&texture));
                                mesh.texture_material.set_needs_update(true);
                                mesh.texture_id = texture_id;
                            }
                        } else {
                            mesh.texture_material.set_map(None);
                            mesh.texture_material.set_needs_update(true);
                            mesh.texture_data.scale().set(0.0, 0.0, 0.0);
                            mesh.texture_id = texture_id;
                        }
                    }

                    if *character.color() != mesh.color {
                        mesh.nameplate.set_color(character.color());
                        mesh.color = character.color().clone();
                    }

                    if *character.display_name() != mesh.nameplate_id {
                        if character.display_name().0 == "" && character.display_name().1 == "" {
                            mesh.nameplate.board().set_visible(false);
                        } else {
                            let texture = texture_table.load_text(character.display_name());
                            mesh.nameplate.text().set_alpha_map(Some(&texture.data));
                            mesh.nameplate.text().set_needs_update(true);

                            let texture_width = f64::min(s * 2.0, texture.size[0] * 0.5);
                            let texture_height = texture_width * texture.size[1] / texture.size[0];
                            mesh.nameplate
                                .board()
                                .scale()
                                .set(texture_width, 1.0, texture_height);
                            mesh.nameplate.board().set_visible(true);
                        }

                        mesh.nameplate_id = character.display_name().clone();
                    }

                    if let Some(texture_block) = texture_block {
                        let tex_height = character.tex_size() * s;
                        texture_block.map(|texture| {
                            let tex_size = texture.size();
                            mesh.texture_data.scale().set(
                                tex_height * tex_size[0] / tex_size[1],
                                1.0,
                                tex_height,
                            );
                            mesh.nameplate.position().set(0.0, 0.0, tex_height + 0.1);
                        });
                    } else {
                        mesh.nameplate.position().set(0.0, 0.0, 0.0);
                    }
                }
            });
        }

        for unused_character_id in unused {
            if let Some(mesh) = self.meshs.remove(&unused_character_id) {
                scene.remove(&mesh.data);
            }
        }
    }

    fn create_texture_geometry() -> three::BufferGeometry {
        let points = js_sys::Float32Array::from(
            [
                [0.5, 0.0, 1.0],
                [-0.5, 0.0, 1.0],
                [-0.5, 0.0, 0.0],
                [0.5, 0.0, 0.0],
            ]
            .concat()
            .as_slice(),
        );
        let uv = js_sys::Float32Array::from(
            [[1.0, 1.0], [0.0, 1.0], [0.0, 0.0], [1.0, 0.0]]
                .concat()
                .as_slice(),
        );
        let index = js_sys::Uint16Array::from([0, 1, 2, 2, 3, 0].as_ref());

        let geometry = three::BufferGeometry::new();
        geometry.set_attribute(
            "position",
            &three::BufferAttribute::new_with_f32array(&points, 3, false),
        );
        geometry.set_attribute(
            "uv",
            &three::BufferAttribute::new_with_f32array(&uv, 2, false),
        );
        geometry.set_index(&three::BufferAttribute::new_with_u16array(&index, 1, false));

        geometry
    }

    fn create_offset_line_geometry() -> three::CylinderGeometry {
        let geometry = three::CylinderGeometry::new(0.05, 0.05, 1.0, 3);

        util::stand(&geometry, 0.5);

        geometry
    }
}
