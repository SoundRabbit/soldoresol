use crate::arena::{block, resource, BlockRef};
use crate::libs::random_id::U128Id;
use crate::libs::three;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::JsCast;

struct Mesh {
    material: three::MeshStandardMaterial,
    data: three::Mesh,
}

pub struct Boxblock {
    geometry_box: three::BoxGeometry,
    geometry_cylinder: three::CylinderGeometry,
    meshs: HashMap<U128Id, Mesh>,
}

impl Boxblock {
    pub fn new() -> Self {
        Self {
            geometry_box: three::BoxGeometry::new(1.0, 1.0, 1.0),
            geometry_cylinder: three::CylinderGeometry::new(0.5, 0.5, 1.0, 16),
            meshs: HashMap::new(),
        }
    }

    pub fn update(
        &mut self,
        scene: &three::Scene,
        boxblocks: impl Iterator<Item = BlockRef<block::Boxblock>>,
    ) {
        let mut unused = self.meshs.keys().map(U128Id::clone).collect::<HashSet<_>>();

        for boxblock in boxblocks {
            let boxblock_id = boxblock.id();
            unused.remove(&boxblock_id);

            boxblock.map(|boxblock| {
                if !self.meshs.contains_key(&boxblock_id) {
                    let material = three::MeshStandardMaterial::new(&object! {});
                    let data = three::Mesh::new(self.get_geometry(boxblock.shape()), &material);
                    data.set_render_order(super::ORDER_BOXBLOCK);
                    data.set_user_data(&boxblock_id.to_jsvalue());
                    scene.add(&data);
                    self.meshs
                        .insert(U128Id::clone(&boxblock_id), Mesh { material, data });
                }
                if let Some(mesh) = self.meshs.get(&boxblock_id) {
                    mesh.data.set_geometry(self.get_geometry(boxblock.shape()));
                    let [px, py, pz] = boxblock.position().clone();
                    mesh.data.position().set(px, py, pz);
                    let [sx, sy, sz] = boxblock.size().clone();
                    mesh.data.scale().set(sx, sy, sz);
                    let [r, g, b, ..] = boxblock.color().to_color().to_f64array();
                    mesh.material.color().set_rgb(r, g, b);
                }
            });
        }

        for unused_boxblock_id in unused {
            if let Some(mesh) = self.meshs.get(&unused_boxblock_id) {
                scene.remove(&mesh.data);
            }
        }
    }

    fn get_geometry(&self, shape: block::boxblock::Shape) -> &three::BufferGeometry {
        match shape {
            block::boxblock::Shape::Cube => &self.geometry_box,
            block::boxblock::Shape::Cylinder => &self.geometry_cylinder,
            _ => &self.geometry_box,
        }
    }
}
