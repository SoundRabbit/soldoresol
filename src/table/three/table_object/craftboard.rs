use super::super::TextureTable;
use super::util;
use crate::arena::{block, resource, BlockRef};
use crate::libs::random_id::U128Id;
use crate::libs::three;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::JsCast;

struct Mesh {
    size: [i32; 2],
    grid_material: three::LineBasicMaterial,
    grid_data: three::LineSegments,
    texture_material: three::MeshBasicMaterial,
    texture_data: three::Mesh,
    texture_id: U128Id,

    nameplate: util::Nameplate,
    nameplate_id: (String, String),

    color: crate::libs::color::Pallet,

    data: three::Group,
}

pub struct Craftboard {
    geometry_square: three::BufferGeometry,
    geometry_nameplate: util::nameplate::XYGeometry,
    meshs: HashMap<U128Id, Mesh>,
}

impl Craftboard {
    pub fn new() -> Self {
        Self {
            geometry_square: Self::create_texture_geometry(),
            geometry_nameplate: util::nameplate::XYGeometry::new(-0.5, -0.5),
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
            unused.remove(&craftboard_id);

            craftboard.map(|craftboard| {
                let sz_f = craftboard.size().clone();
                let sz_i = [
                    sz_f[0].floor() as i32,
                    sz_f[1].floor() as i32,
                    sz_f[2].floor() as i32,
                ];
                if !self.meshs.contains_key(&craftboard_id) {
                    let grid_material = three::LineBasicMaterial::new(&object! {});
                    grid_material.set_stencil_write(true);
                    grid_material.set_stencil_func(web_sys::WebGl2RenderingContext::ALWAYS);
                    grid_material.set_stencil_ref(1);
                    grid_material.set_stencil_fail(web_sys::WebGl2RenderingContext::KEEP);
                    grid_material.set_stencil_z_fail(web_sys::WebGl2RenderingContext::KEEP);
                    grid_material.set_stencil_z_pass(web_sys::WebGl2RenderingContext::REPLACE);

                    let grid_data = three::LineSegments::new(
                        &Self::create_grid_geometry(&sz_i),
                        &grid_material,
                    );
                    grid_data.set_render_order(0.0);

                    let texture_material = three::MeshBasicMaterial::new(&object! {
                        "transparent": true
                    });
                    texture_material.color().set_rgb(1.0, 1.0, 1.0);
                    texture_material.set_stencil_write(true);
                    texture_material.set_stencil_func(web_sys::WebGl2RenderingContext::EQUAL);
                    texture_material.set_stencil_ref(0);
                    texture_material.set_stencil_fail(web_sys::WebGl2RenderingContext::REPLACE);
                    texture_material.set_stencil_z_fail(web_sys::WebGl2RenderingContext::REPLACE);
                    texture_material.set_stencil_z_pass(web_sys::WebGl2RenderingContext::REPLACE);

                    let texture_data = three::Mesh::new(&self.geometry_square, &texture_material);
                    texture_data.set_user_data(&craftboard_id.to_jsvalue());
                    texture_data.set_render_order(1.0);

                    let nameplate = util::Nameplate::new(&self.geometry_nameplate);
                    nameplate.set_color(craftboard.grid_color());
                    nameplate.set_user_data(&craftboard_id.to_jsvalue());
                    nameplate.scale().set(1.0, 1.0, 1.0);
                    nameplate.board().scale().set(0.0, 0.0, 1.0);

                    let data = three::Group::new();
                    data.add(&grid_data);
                    data.add(&texture_data);
                    scene.add(&data);
                    data.add(&nameplate);

                    self.meshs.insert(
                        U128Id::clone(&craftboard_id),
                        Mesh {
                            grid_data,
                            grid_material,
                            texture_data,
                            texture_material,
                            texture_id: U128Id::none(),
                            nameplate,
                            nameplate_id: (String::from(""), String::from("")),
                            color: craftboard.grid_color().clone(),
                            data,
                            size: [sz_i[0], sz_i[1]],
                        },
                    );
                }
                if let Some(mesh) = self.meshs.get_mut(&craftboard_id) {
                    if mesh.size[0] != sz_i[0] || mesh.size[1] != sz_i[1] {
                        mesh.grid_data
                            .set_geometry(&Self::create_grid_geometry(&sz_i));
                        mesh.size[0] = sz_i[0];
                        mesh.size[1] = sz_i[1];
                    }

                    let [p_x, p_y, p_z] = craftboard.position().clone();
                    mesh.data.position().set(p_x, p_y, p_z);

                    mesh.texture_data
                        .scale()
                        .set(sz_f[0] + 0.02, sz_f[1] + 0.02, sz_f[2] + 0.02);
                    mesh.nameplate
                        .position()
                        .set(sz_f[0] * 0.5 + 0.02, -sz_f[1] * 0.5 - 0.02, 0.0);

                    let [r, g, b, ..] = craftboard.grid_color().to_color().to_f64array();
                    mesh.grid_material.color().set_rgb(r, g, b);

                    let textures = craftboard.textures();
                    let texture_id = Self::set_texture(
                        texture_table,
                        &mesh.texture_id,
                        &mesh.texture_material,
                        textures.nz.as_ref().map(|nz| BlockRef::clone(&nz)),
                    );
                    mesh.texture_id = texture_id;

                    if *craftboard.grid_color() != mesh.color {
                        mesh.nameplate.set_color(craftboard.grid_color());
                        mesh.color = craftboard.grid_color().clone();
                    }

                    if *craftboard.display_name() != mesh.nameplate_id {
                        let texture = texture_table.load_text(craftboard.display_name());
                        mesh.nameplate.text().set_alpha_map(Some(&texture.data));
                        mesh.nameplate.text().set_needs_update(true);

                        let texture_width = texture.size[0] * 0.5;
                        let texture_height = texture_width * texture.size[1] / texture.size[0];
                        mesh.nameplate
                            .board()
                            .scale()
                            .set(texture_width, texture_height, 1.0);
                        mesh.nameplate_id = craftboard.display_name().clone();
                    }
                }
            });
        }

        for unused_craftboard_id in unused {
            if let Some(mesh) = self.meshs.get(&unused_craftboard_id) {
                scene.remove(&mesh.data);
            }
        }
    }

    fn set_texture(
        texture_table: &mut TextureTable,
        prev_texture_id: &U128Id,
        material: &three::MeshBasicMaterial,
        texture: Option<BlockRef<resource::ImageData>>,
    ) -> U128Id {
        let texture_id = texture
            .as_ref()
            .map(|texture| texture.id())
            .unwrap_or_else(|| U128Id::none());

        if *prev_texture_id != texture_id {
            if let Some(texture) = texture {
                if let Some(texture) = texture_table.load_image(BlockRef::clone(&texture)) {
                    material.set_map(Some(&texture));
                    material.set_needs_update(true);
                }
            } else {
                material.set_map(None);
                material.set_needs_update(true);
            }
        }

        texture_id
    }

    fn create_grid_geometry(size: &[i32; 3]) -> three::BufferGeometry {
        let points = js_sys::Array::new();

        let xn = size[0] + 1;
        let yn = size[1] + 1;
        let offset_x = -size[0] as f64 / 2.0;
        let offset_y = -size[1] as f64 / 2.0;

        for x in 0..xn {
            let x = x as f64 + offset_x;
            points.push(&three::Vector3::new(x, offset_y, 0.0));
            points.push(&three::Vector3::new(x, size[1] as f64 + offset_y, 0.0));
        }

        for y in 0..yn {
            let y = y as f64 + offset_y;
            points.push(&three::Vector3::new(offset_x, y, 0.0));
            points.push(&three::Vector3::new(size[0] as f64 + offset_x, y, 0.0));
        }

        three::BufferGeometry::new().set_from_points(&points)
    }

    fn create_texture_geometry() -> three::BufferGeometry {
        let points = js_sys::Float32Array::from(
            [
                [0.5, 0.5, 0.0],
                [-0.5, 0.5, 0.0],
                [-0.5, -0.5, 0.0],
                [0.5, -0.5, 0.0],
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
}
