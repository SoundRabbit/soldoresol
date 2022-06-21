use crate::libs::three;

pub struct Nameplate {
    material_text: three::MeshBasicMaterial,
    material_background: three::MeshBasicMaterial,

    mesh_front: three::Mesh,
    mesh_back: three::Mesh,
    mesh_background: three::Mesh,
    mesh_arrow: Option<three::Mesh>,

    data_board: three::Group,
    data: three::Group,
}

pub trait NameplateGeometry {
    fn front(&self) -> &three::BufferGeometry;
    fn back(&self) -> &three::BufferGeometry;
    fn background(&self) -> &three::BufferGeometry;
    fn arrow(&self) -> Option<&three::BufferGeometry>;
}

pub struct XZGeometry {
    front: three::BufferGeometry,
    back: three::BufferGeometry,
    background: three::BufferGeometry,
    arrow: Option<three::BufferGeometry>,
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
        let mesh_arrow = geometry
            .arrow()
            .map(|arrow_geometry| three::Mesh::new(&arrow_geometry, &material_background));

        let data_board = three::Group::new();
        data_board.add(&mesh_front);
        data_board.add(&mesh_back);
        data_board.add(&mesh_background);

        let data = three::Group::new();
        data.add(&data_board);
        if let Some(mesh_arrow) = mesh_arrow.as_ref() {
            data.add(&mesh_arrow);
        }

        Self {
            material_text,
            material_background,
            mesh_front,
            mesh_back,
            mesh_background,
            mesh_arrow,
            data_board,
            data,
        }
    }

    pub fn text(&self) -> &three::MeshBasicMaterial {
        &self.material_text
    }

    pub fn background(&self) -> &three::MeshBasicMaterial {
        &self.material_background
    }

    pub fn board(&self) -> &three::Group {
        &self.data_board
    }

    pub fn set_color(&self, pallet: &crate::libs::color::Pallet) {
        let color = pallet.to_color();
        let [r, g, b, ..] = color.to_f64array();
        self.background().color().set_rgb(r, g, b);
        if color.v() > 0.9 {
            self.text().color().set_rgb(0.0, 0.0, 0.0);
        } else {
            self.text().color().set_rgb(1.0, 1.0, 1.0);
        }
        self.background().set_needs_update(true);
        self.text().set_needs_update(true);
    }
}

impl std::ops::Deref for Nameplate {
    type Target = three::Group;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl XZGeometry {
    pub fn new(z_offset: f32, arrow: bool) -> Self {
        let ext = 0.05;
        let arrow_height = 0.25;
        let z_offset = if arrow {
            z_offset + arrow_height
        } else {
            z_offset
        };
        Self {
            front: Self::create_geometry(-0.01, z_offset + ext, 0.0, false),
            back: Self::create_geometry(0.01, z_offset + ext, 0.0, true),
            background: Self::create_geometry(0.0, z_offset + ext, ext, false),
            arrow: if arrow {
                Some(Self::create_arrow(arrow_height, z_offset))
            } else {
                None
            },
        }
    }

    fn create_geometry(y_offset: f32, z_offset: f32, ext: f32, inv: bool) -> three::BufferGeometry {
        let inv = if inv { -1.0 } else { 1.0 };
        let points = js_sys::Float32Array::from(
            [
                [0.5 * inv + ext, y_offset, 0.5 + z_offset + ext],
                [-0.5 * inv - ext, y_offset, 0.5 + z_offset + ext],
                [-0.5 * inv - ext, y_offset, -0.5 + z_offset - ext],
                [0.5 * inv + ext, y_offset, -0.5 + z_offset - ext],
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

    fn create_arrow(height: f32, z_offset: f32) -> three::BufferGeometry {
        let points = js_sys::Float32Array::from(
            [
                [height, 0.0, -0.5 + z_offset],
                [-height, 0.0, -0.5 + z_offset],
                [0.0, 0.0, -0.5 - height + z_offset],
            ]
            .concat()
            .as_slice(),
        );
        let uv =
            js_sys::Float32Array::from([[1.0, 1.0], [0.0, 1.0], [0.5, 0.0]].concat().as_slice());
        let index = js_sys::Uint16Array::from([0, 1, 2].as_ref());
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

    fn arrow(&self) -> Option<&three::BufferGeometry> {
        self.arrow.as_ref()
    }
}
