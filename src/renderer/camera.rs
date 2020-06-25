use ndarray::{arr1, arr2, Array1, Array2};

pub struct Camera {
    x_axis_rotation: f64,
    z_axis_rotation: f64,
    movement: [f64; 3],
}

impl Camera {
    pub fn new() -> Self {
        Self {
            x_axis_rotation: -0.25 * std::f64::consts::PI,
            z_axis_rotation: -0.03125 * std::f64::consts::PI,
            movement: [0.0, 0.0, -20.0],
        }
    }

    fn e() -> Array2<f64> {
        arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    fn rotate_view_matrix_with_x_axis(view: Array2<f64>, x_axis_rotation: f64) -> Array2<f64> {
        let (s, c) = x_axis_rotation.sin_cos();
        let t = arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, c, s, 0.0],
            [0.0, -s, c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        view.dot(&t)
    }

    fn rotate_view_matrix_with_z_axis(view: Array2<f64>, z_axis_rotation: f64) -> Array2<f64> {
        let (s, c) = z_axis_rotation.sin_cos();
        let t = arr2(&[
            [c, s, 0.0, 0.0],
            [-s, c, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        view.dot(&t)
    }

    fn move_view_matrix(view: Array2<f64>, m: &[f64; 3]) -> Array2<f64> {
        let t = arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [m[0], m[1], m[2], 1.0],
        ]);
        view.dot(&t)
    }

    pub fn set_x_axis_rotation(&mut self, x_axis_rotation: f64) {
        self.x_axis_rotation = x_axis_rotation.max(-0.5 * std::f64::consts::PI).min(0.0);
    }

    pub fn x_axis_rotation(&self) -> f64 {
        self.x_axis_rotation
    }

    pub fn set_z_axis_rotation(&mut self, z_axis_rotation: f64) {
        self.z_axis_rotation = z_axis_rotation;
    }

    pub fn z_axis_rotation(&self) -> f64 {
        self.z_axis_rotation
    }

    pub fn set_movement(&mut self, movement: [f64; 3]) {
        self.movement = movement;
    }

    pub fn movement(&self) -> &[f64; 3] {
        &self.movement
    }

    pub fn view_matrix(&self) -> Array2<f64> {
        let view_matrix = Self::e();
        let view_matrix = Self::rotate_view_matrix_with_z_axis(view_matrix, self.z_axis_rotation);
        let view_matrix = Self::rotate_view_matrix_with_x_axis(view_matrix, self.x_axis_rotation);
        let view_matrix = Self::move_view_matrix(view_matrix, &self.movement);
        view_matrix
    }

    pub fn perspective_matrix(&self, canvas_size: &[f64; 2]) -> Array2<f64> {
        let w = canvas_size[0];
        let h = canvas_size[1];
        let aspect = w / h;
        let field_of_view = 60.0 * std::f64::consts::PI / 180.0;
        let near = 1.0;
        let far = 1000.0;
        let f = (std::f64::consts::PI * 0.5 - field_of_view * 0.5).tan();
        let range_inv = 1.0 / (near - far);
        arr2(&[
            [f / aspect, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (near + far) * range_inv, -1.0],
            [0.0, 0.0, near * far * range_inv * 2.0, 0.0],
        ])
    }

    pub fn inv_view_matrix(&self) -> Array2<f64> {
        let t = self.view_matrix();
        arr2(&[
            [t.row(0)[0], t.row(1)[0], t.row(2)[0], 0.0],
            [t.row(0)[1], t.row(1)[1], t.row(2)[1], 0.0],
            [t.row(0)[2], t.row(1)[2], t.row(2)[2], 0.0],
            [
                -t.row(0).dot(&t.row(3)),
                -t.row(1).dot(&t.row(3)),
                -t.row(2).dot(&t.row(3)),
                1.0,
            ],
        ])
    }

    pub fn inv_perspective_matrix(&self, canvas_size: &[f64; 2]) -> Array2<f64> {
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
        canvas_size: &[f64; 2],
        screen_position: &[f64; 2],
    ) -> Array1<f64> {
        let inv_v = self.inv_view_matrix();
        let inv_p = self.inv_perspective_matrix(canvas_size);
        let inv = inv_p.dot(&inv_v);
        let inv = inv.t();

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
}
