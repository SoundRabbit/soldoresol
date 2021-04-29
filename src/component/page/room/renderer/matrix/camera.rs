use ndarray::{arr1, arr2, Array1, Array2};

pub struct CameraMatrix {
    x_axis_rotation: f32,
    z_axis_rotation: f32,
    movement: [f32; 3],
    field_of_view: f32,
}

impl CameraMatrix {
    pub fn new() -> Self {
        Self {
            x_axis_rotation: 0.25 * std::f32::consts::PI,
            z_axis_rotation: 0.03125 * std::f32::consts::PI,
            movement: [0.0, 0.0, 20.0],
            field_of_view: 30.0,
        }
    }

    fn e() -> Array2<f32> {
        arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    fn rotate_view_matrix_with_x_axis(view: &Array2<f32>, x_axis_rotation: f32) -> Array2<f32> {
        let (s, c) = x_axis_rotation.sin_cos();
        let t = arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, c, s, 0.0],
            [0.0, -s, c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        t.dot(view)
    }

    fn rotate_view_matrix_with_z_axis(view: &Array2<f32>, z_axis_rotation: f32) -> Array2<f32> {
        let (s, c) = z_axis_rotation.sin_cos();
        let t = arr2(&[
            [c, s, 0.0, 0.0],
            [-s, c, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        t.dot(view)
    }

    fn move_view_matrix(view: &Array2<f32>, m: &[f32; 3]) -> Array2<f32> {
        let t = arr2(&[
            [1.0, 0.0, 0.0, -m[0]],
            [0.0, 1.0, 0.0, -m[1]],
            [0.0, 0.0, 1.0, -m[2]],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        t.dot(view)
    }

    pub fn set_x_axis_rotation(&mut self, x_axis_rotation: f32, clip: bool) {
        if clip {
            self.x_axis_rotation = x_axis_rotation.min(0.5 * std::f32::consts::PI).max(0.0);
        } else {
            self.x_axis_rotation = x_axis_rotation
        }
    }

    pub fn x_axis_rotation(&self) -> f32 {
        self.x_axis_rotation
    }

    pub fn set_z_axis_rotation(&mut self, z_axis_rotation: f32) {
        self.z_axis_rotation = z_axis_rotation;
    }

    pub fn z_axis_rotation(&self) -> f32 {
        self.z_axis_rotation
    }

    pub fn set_movement(&mut self, movement: [f32; 3]) {
        self.movement = movement;
    }

    pub fn movement(&self) -> &[f32; 3] {
        &self.movement
    }

    pub fn set_to_px(&mut self) {
        self.set_x_axis_rotation(0.5 * std::f32::consts::PI, false);
        self.set_z_axis_rotation(-0.5 * std::f32::consts::PI);
    }

    pub fn set_to_py(&mut self) {
        self.set_x_axis_rotation(0.5 * std::f32::consts::PI, false);
        self.set_z_axis_rotation(0.0 * std::f32::consts::PI);
    }

    pub fn set_to_pz(&mut self) {
        self.set_x_axis_rotation(1.0 * std::f32::consts::PI, false);
        self.set_z_axis_rotation(0.0 * std::f32::consts::PI);
    }

    pub fn set_to_nx(&mut self) {
        self.set_x_axis_rotation(0.5 * std::f32::consts::PI, false);
        self.set_z_axis_rotation(0.5 * std::f32::consts::PI);
    }

    pub fn set_to_ny(&mut self) {
        self.set_x_axis_rotation(-0.5 * std::f32::consts::PI, false);
        self.set_z_axis_rotation(0.0 * std::f32::consts::PI);
    }

    pub fn set_to_nz(&mut self) {
        self.set_x_axis_rotation(0.0 * std::f32::consts::PI, false);
        self.set_z_axis_rotation(0.0 * std::f32::consts::PI);
    }

    pub fn set_field_of_view(&mut self, field_of_view: f32) {
        self.field_of_view = field_of_view;
    }

    pub fn vp_matrix(&self, canvas_size: &[f32; 2], rev: bool) -> Array2<f32> {
        self.perspective_matrix(&canvas_size)
            .dot(&self.view_matrix(true))
    }

    pub fn view_matrix(&self, rev: bool) -> Array2<f32> {
        let view_matrix = Self::e();
        let view_matrix = Self::rotate_view_matrix_with_z_axis(&view_matrix, self.z_axis_rotation);
        let view_matrix = Self::rotate_view_matrix_with_x_axis(&view_matrix, self.x_axis_rotation);
        let view_matrix = Self::move_view_matrix(&view_matrix, &self.movement);
        view_matrix
    }

    pub fn perspective_matrix(&self, canvas_size: &[f32; 2]) -> Array2<f32> {
        let w = canvas_size[0];
        let h = canvas_size[1];
        let aspect = w / h;
        let field_of_view = self.field_of_view * std::f32::consts::PI / 180.0;
        let near = 1.0;
        let far = 100.0;
        let f = (std::f32::consts::PI * 0.5 - field_of_view * 0.5).tan();
        let range_inv = 1.0 / (near - far);
        arr2(&[
            [f / aspect, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (near + far) * range_inv, -1.0],
            [0.0, 0.0, near * far * range_inv * 2.0, 0.0],
        ])
    }

    pub fn inv_view_matrix(&self) -> Array2<f32> {
        let m = &self.movement;
        let view_matrix = Self::e();
        let view_matrix = Self::move_view_matrix(&view_matrix, &[-m[0], -m[1], -m[2]]);
        let view_matrix = Self::rotate_view_matrix_with_x_axis(&view_matrix, -self.x_axis_rotation);
        let view_matrix = Self::rotate_view_matrix_with_z_axis(&view_matrix, -self.z_axis_rotation);
        view_matrix
    }

    pub fn inv_perspective_matrix(&self, canvas_size: &[f32; 2]) -> Array2<f32> {
        let p = self.perspective_matrix(canvas_size);
        arr2(&[
            [1.0 / p.row(0)[0], 0.0, 0.0, 0.0],
            [0.0, 1.0 / p.row(1)[1], 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0 / p.row(3)[2]],
            [0.0, 0.0, -1.0, p.row(2)[2] / p.row(3)[2]],
        ])
    }

    pub fn collision_point_on_xy_plane(
        &self,
        canvas_size: &[f32; 2],
        screen_position: &[f32; 2],
    ) -> Array1<f32> {
        let inv_v = self.inv_view_matrix();
        let inv_p = self.inv_perspective_matrix(canvas_size);
        let inv = inv_v.dot(&inv_p);

        let p = [
            screen_position[0] / canvas_size[0] * 2.0 - 1.0,
            -(screen_position[1] / canvas_size[1] * 2.0 - 1.0),
        ];

        // inv * [p[0] * w, p[1] * w, screen_z, w] = [world_x, world_y, 0, 1] を解く
        //
        // inv[2][2] * z + ( inv[2][3] + inv[2][0] * p[0] + inv[2][1] * p[1] ) * w = 0
        // inv[3][2] * z + ( inv[3][3] + inv[3][0] * p[0] + inv[3][1] * p[1] ) * w = 1

        let a = inv.row(2)[2];
        let b = inv.row(2)[3] + inv.row(2)[0] * p[0] + inv.row(2)[1] * p[1];
        let c = inv.row(3)[2];
        let d = inv.row(3)[3] + inv.row(3)[0] * p[0] + inv.row(3)[1] * p[1];
        let aa = 0.0;
        let bb = 1.0;

        let screen_z = (d * aa - b * bb) / (a * d - b * c);
        let w = (a * bb - c * aa) / (a * d - b * c);

        inv.dot(&arr1(&[p[0] * w, p[1] * w, screen_z, w]))
    }

    /**
     * 3点r, r+s, r+tで貼られる面との衝突点を求める
     */
    pub fn collision_point(
        &self,
        canvas_size: &[f32; 2],
        screen_position: &[f32; 2],
        r: &[f32; 3],
        s: &[f32; 3],
        t: &[f32; 3],
    ) -> [f32; 3] {
        let inv_v = self.inv_view_matrix();
        let inv_p = self.inv_perspective_matrix(canvas_size);
        let inv = inv_v.dot(&inv_p);

        let p = [
            screen_position[0] / canvas_size[0] * 2.0 - 1.0,
            -(screen_position[1] / canvas_size[1] * 2.0 - 1.0),
        ];

        let ix = inv.row(0);
        let iy = inv.row(1);
        let iz = inv.row(2);
        let iw = inv.row(3);
        let xw = ix[0] * p[0] + ix[1] * p[1] + ix[3];
        let yw = iy[0] * p[0] + iy[1] * p[1] + iy[3];
        let zw = iz[0] * p[0] + iz[1] * p[1] + iz[3];
        let ww = iw[0] * p[0] + iw[1] * p[1] + iw[3];

        let mut m = [
            [ix[2], xw, -s[0], -t[0], r[0]],
            [iy[2], yw, -s[1], -t[1], r[1]],
            [iz[2], zw, -s[2], -t[2], r[2]],
            [iw[2], ww, 0.0, 0.0, 1.0],
        ];

        // 拡大係数行列mを解く
        for rc in 0..4 {
            for r in 0..4 {
                if r == rc {
                    let f = m[rc][rc];
                    for c in 0..5 {
                        m[r][c] /= f;
                    }
                } else {
                    let b = m[r][rc] / m[rc][rc];
                    for c in 0..5 {
                        m[r][c] -= m[rc][c] * b;
                    }
                }
            }
        }

        let u = m[2][4];
        let v = m[3][4];

        [
            u * s[0] + v * t[0] + r[0],
            u * s[1] + v * t[1] + r[1],
            u * s[2] + v * t[2] + r[2],
        ]
    }
}
