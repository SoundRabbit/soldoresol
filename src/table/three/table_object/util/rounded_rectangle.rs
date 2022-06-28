use crate::libs::three;
use wasm_bindgen::prelude::*;

pub trait RoundedRectangleGeometry {
    fn top_left(&self) -> &three::BufferGeometry;
    fn top_right(&self) -> &three::BufferGeometry;
    fn bottom_left(&self) -> &three::BufferGeometry;
    fn bottom_right(&self) -> &three::BufferGeometry;
    fn left(&self) -> &three::BufferGeometry;
    fn right(&self) -> &three::BufferGeometry;
    fn top(&self) -> &three::BufferGeometry;
    fn bottom(&self) -> &three::BufferGeometry;
}

macro_rules! geometry {
    (pub $t:ident) => {
        pub struct $t {
            pub top_left: three::BufferGeometry,
            pub top_right: three::BufferGeometry,
            pub bottom_left: three::BufferGeometry,
            pub bottom_right: three::BufferGeometry,
            pub left: three::BufferGeometry,
            pub right: three::BufferGeometry,
            pub top: three::BufferGeometry,
            pub bottom: three::BufferGeometry,
        }

        impl RoundedRectangleGeometry for $t {
            fn top_left(&self) -> &three::BufferGeometry {
                &self.top_left
            }
            fn top_right(&self) -> &three::BufferGeometry {
                &self.top_right
            }
            fn bottom_left(&self) -> &three::BufferGeometry {
                &self.bottom_left
            }
            fn bottom_right(&self) -> &three::BufferGeometry {
                &self.bottom_right
            }
            fn left(&self) -> &three::BufferGeometry {
                &self.left
            }
            fn right(&self) -> &three::BufferGeometry {
                &self.right
            }
            fn top(&self) -> &three::BufferGeometry {
                &self.top
            }
            fn bottom(&self) -> &three::BufferGeometry {
                &self.bottom
            }
        }
    };
}

geometry!(pub BasicRoundedRectangleGeometry);
geometry!(pub CornerRoundedRectangleGeometry);

pub struct RoundedRectangleMesh {
    tl_data: three::Mesh,
    tr_data: three::Mesh,
    bl_data: three::Mesh,
    br_data: three::Mesh,
    l_data: three::Mesh,
    r_data: three::Mesh,
    t_data: three::Mesh,
    b_data: three::Mesh,
    data: three::Group,
}

impl BasicRoundedRectangleGeometry {
    pub fn new() -> Self {
        let theta_unit = std::f64::consts::FRAC_PI_2;
        let top_left = three::CircleGeometry::new_with_theta(1.0, 8, theta_unit * 1.0, theta_unit);
        let top_right = three::CircleGeometry::new_with_theta(1.0, 8, theta_unit * 0.0, theta_unit);
        let bottom_left =
            three::CircleGeometry::new_with_theta(1.0, 8, theta_unit * 2.0, theta_unit);
        let bottom_right =
            three::CircleGeometry::new_with_theta(1.0, 8, theta_unit * 3.0, theta_unit);

        let left = three::PlaneGeometry::new(1.0, 1.0);
        let right = three::PlaneGeometry::new(1.0, 1.0);
        let top = three::PlaneGeometry::new(1.0, 1.0);
        let bottom = three::PlaneGeometry::new(1.0, 1.0);

        Self {
            top_left: top_left.into(),
            top_right: top_right.into(),
            bottom_left: bottom_left.into(),
            bottom_right: bottom_right.into(),
            left: left.into(),
            right: right.into(),
            top: top.into(),
            bottom: bottom.into(),
        }
    }
}

impl CornerRoundedRectangleGeometry {
    pub fn new() -> Self {
        let top_left = Self::create_geometry(2.0);
        let top_right = Self::create_geometry(1.0);
        let bottom_left = Self::create_geometry(3.0);
        let bottom_right = Self::create_geometry(0.0);

        let left = three::BufferGeometry::new();
        let right = three::BufferGeometry::new();
        let top = three::BufferGeometry::new();
        let bottom = three::BufferGeometry::new();

        Self {
            top_left: top_left.into(),
            top_right: top_right.into(),
            bottom_left: bottom_left.into(),
            bottom_right: bottom_right.into(),
            left: left.into(),
            right: right.into(),
            top: top.into(),
            bottom: bottom.into(),
        }
    }

    fn create_geometry(offset: f64) -> three::BufferGeometry {
        let geometry = three::CylinderGeometry::new_with_option(
            1.0,
            1.0,
            1.0,
            4,
            1,
            true,
            std::f64::consts::FRAC_PI_2 * offset + std::f64::consts::FRAC_PI_4,
            std::f64::consts::FRAC_PI_4,
        );
        super::stand(&geometry, 0.5);
        geometry.into()
    }
}

impl RoundedRectangleMesh {
    pub fn new(geometry: &impl RoundedRectangleGeometry, material: &three::Material) -> Self {
        let tl_data = three::Mesh::new(geometry.top_left(), &material);
        let tr_data = three::Mesh::new(geometry.top_right(), &material);
        let bl_data = three::Mesh::new(geometry.bottom_left(), &material);
        let br_data = three::Mesh::new(geometry.bottom_right(), &material);
        let l_data = three::Mesh::new(geometry.left(), &material);
        let r_data = three::Mesh::new(geometry.right(), &material);
        let t_data = three::Mesh::new(geometry.top(), &material);
        let b_data = three::Mesh::new(geometry.bottom(), &material);
        let data = three::Group::new();

        data.add(&tl_data);
        data.add(&tr_data);
        data.add(&bl_data);
        data.add(&br_data);
        data.add(&l_data);
        data.add(&r_data);
        data.add(&t_data);
        data.add(&b_data);

        Self {
            tl_data,
            tr_data,
            bl_data,
            br_data,
            l_data,
            r_data,
            t_data,
            b_data,
            data,
        }
    }

    pub fn data(&self) -> &three::Group {
        &self.data
    }

    pub fn set_scale(&self, inner_size: &[f64; 2], border_width: f64) {
        let sw = inner_size[0];
        let sh = inner_size[1];

        let xc = sw * 0.5;
        let yc = sh * 0.5;
        let xcc = xc + border_width * 0.5;
        let ycc = yc + border_width * 0.5;

        self.tl_data.position().set(-xc, yc, 0.0);
        self.tl_data.scale().set(border_width, border_width, 1.0);

        self.tr_data.position().set(xc, yc, 0.0);
        self.tr_data.scale().set(border_width, border_width, 1.0);

        self.bl_data.position().set(-xc, -yc, 0.0);
        self.bl_data.scale().set(border_width, border_width, 1.0);

        self.br_data.position().set(xc, -yc, 0.0);
        self.br_data.scale().set(border_width, border_width, 1.0);

        self.l_data.position().set(-xcc, 0.0, 0.0);
        self.l_data.scale().set(border_width, sh, 1.0);

        self.r_data.position().set(xcc, 0.0, 0.0);
        self.r_data.scale().set(border_width, sh, 1.0);

        self.t_data.position().set(0.0, ycc, 0.0);
        self.t_data.scale().set(sw, border_width, 1.0);

        self.b_data.position().set(0.0, -ycc, 0.0);
        self.b_data.scale().set(sw, border_width, 1.0);
    }

    pub fn set_user_data(&self, user_data: &JsValue) {
        self.tl_data.set_user_data(user_data);
        self.tr_data.set_user_data(user_data);
        self.bl_data.set_user_data(user_data);
        self.br_data.set_user_data(user_data);
        self.l_data.set_user_data(user_data);
        self.r_data.set_user_data(user_data);
        self.t_data.set_user_data(user_data);
        self.b_data.set_user_data(user_data);
        self.data.set_user_data(user_data);
    }
}
