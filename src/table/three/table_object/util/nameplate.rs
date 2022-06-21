use crate::libs::three;

pub struct Nameplate {
    material_text: three::MeshBasicMaterial,
    material_background: three::MeshBasicMaterial,

    mesh_front: three::Mesh,
    mesh_back: three::Mesh,
    mesh_background: three::Mesh,

    data: three::Group,
}

pub trait NameplateGeometry {
    fn front(&self) -> &three::BufferGeometry;
    fn back(&self) -> &three::BufferGeometry;
    fn background(&self) -> &three::BufferGeometry;
}

pub struct XZGeometry {
    front: three::BufferGeometry,
    back: three::BufferGeometry,
    background: three::BufferGeometry,
}

impl Nameplate {
    pub fn new(geometry: &impl NameplateGeometry) -> Self {
        let material_text = three::MeshBasicMaterial::new(&object! {
            "transparent": true
        });
        let material_background = three::MeshBasicMaterial::new(&object! {});
        material_background.set_side(three::DOUBLE_SIDE);

        let mesh_front = three::Mesh::new(geometry.front(), &material_text);
        let mesh_back = three::Mesh::new(geometry.back(), &material_text);
        let mesh_background = three::Mesh::new(geometry.background(), &material_background);

        let data = three::Group::new();
        data.add(&mesh_front);
        data.add(&mesh_back);
        data.add(&mesh_background);

        Self {
            material_text,
            material_background,
            mesh_front,
            mesh_back,
            mesh_background,
            data,
        }
    }

    pub fn text(&self) -> &three::MeshBasicMaterial {
        &self.material_text
    }

    pub fn background(&self) -> &three::MeshBasicMaterial {
        &self.material_background
    }
}

impl std::ops::Deref for Nameplate {
    type Target = three::Group;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl XZGeometry {
    pub fn new(z_offset: f32) -> Self {
        Self {
            front: Self::create_geometry(-0.01, z_offset, false),
            back: Self::create_geometry(0.01, z_offset, true),
            background: Self::create_geometry(0.0, z_offset, false),
        }
    }

    fn create_geometry(y_offset: f32, z_offset: f32, inv: bool) -> three::BufferGeometry {
        let inv = if inv { -1.0 } else { 1.0 };
        let points = js_sys::Float32Array::from(
            [
                [0.5 * inv, y_offset, 0.5 + z_offset],
                [-0.5 * inv, y_offset, 0.5 + z_offset],
                [-0.5 * inv, y_offset, -0.5 + z_offset],
                [0.5 * inv, y_offset, -0.5 + z_offset],
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

impl NameplateGeometry for XZGeometry {
    fn front(&self) -> &three::BufferGeometry {
        &self.front
    }

    fn back(&self) -> &three::BufferGeometry {
        &self.back
    }

    fn background(&self) -> &three::BufferGeometry {
        &self.background
    }
}
