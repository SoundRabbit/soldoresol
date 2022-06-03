use super::super::TextureTable;
use crate::arena::{block, resource, BlockRef};
use crate::libs::random_id::U128Id;
use crate::libs::three;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::JsCast;

struct Mesh {
    size: [i32; 2],
    grid_material: three::LineDashedMaterial,
    grid_data: three::LineSegments,
    texture_material: three::MeshBasicMaterial,
    texture_data: three::Mesh,
    data: three::Group,
}

pub struct Craftboard {
    geometry_square: three::PlaneGeometry,
    meshs: HashMap<U128Id, Mesh>,
}

impl Craftboard {
    pub fn new() -> Self {
        Self {
            geometry_square: three::PlaneGeometry::new(1.0, 1.0),
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
                    sz_f[0].round() as i32,
                    sz_f[1].round() as i32,
                    sz_f[2].round() as i32,
                ];
                if !self.meshs.contains_key(&craftboard_id) {
                    let grid_material = three::LineDashedMaterial::new(&object! {
                        "dashSize": 0.15,
                        "gapSize": 0.1
                    });
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

                    let texture_material = three::MeshBasicMaterial::new(&object! {});
                    texture_material.set_stencil_write(true);
                    texture_material.set_stencil_func(web_sys::WebGl2RenderingContext::EQUAL);
                    texture_material.set_stencil_ref(0);
                    texture_material.set_stencil_fail(web_sys::WebGl2RenderingContext::REPLACE);
                    texture_material.set_stencil_z_fail(web_sys::WebGl2RenderingContext::REPLACE);
                    texture_material.set_stencil_z_pass(web_sys::WebGl2RenderingContext::REPLACE);

                    let texture_data = three::Mesh::new(&self.geometry_square, &texture_material);
                    texture_data.set_user_data(&craftboard_id.to_jsvalue());
                    texture_data.set_render_order(1.0);

                    let data = three::Group::new();
                    data.add(&grid_data);
                    data.add(&texture_data);
                    data.set_user_data(&craftboard_id.to_jsvalue());
                    scene.add(&data);

                    self.meshs.insert(
                        U128Id::clone(&craftboard_id),
                        Mesh {
                            grid_data,
                            grid_material,
                            texture_data,
                            texture_material,
                            data,
                            size: [sz_i[0], sz_i[1]],
                        },
                    );
                }
                if let Some(mesh) = self.meshs.get(&craftboard_id) {
                    if mesh.size[0] != sz_i[0] || mesh.size[1] != sz_i[1] {
                        mesh.grid_data
                            .set_geometry(&Self::create_grid_geometry(&sz_i));
                    }

                    let [p_x, p_y, p_z] = craftboard.position().clone();
                    mesh.data.position().set(p_x, p_y, p_z);

                    mesh.texture_data
                        .scale()
                        .set(sz_f[0] + 0.02, sz_f[1] + 0.02, sz_f[2] + 0.02);

                    let [r, g, b, ..] = craftboard.grid_color().to_color().to_f64array();
                    mesh.grid_material.color().set_rgb(r, g, b);

                    if let Some(texture) = &craftboard.textures().nz {
                        if let Some(texture) = texture_table.load_image(BlockRef::clone(&texture)) {
                            mesh.texture_material.set_map(&texture);
                        }
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
}
