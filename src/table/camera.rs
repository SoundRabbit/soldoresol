use ndarray::{arr2, Array2};

pub struct Camera {
    v_matrix: Array2<f32>,
    width: f32,
    height: f32,
    near: f32,
    far: f32,
}

impl Camera {
    fn e() -> Array2<f32> {
        arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn p_matrix(&self) -> Array2<f32> {
        let h = self.height;
        let w = self.width;
        let a = w / h;
        let field_of_view = 30.0;
        let nr = self.near;
        let fr = self.far;
        let f = (std::f32::consts::PI * 0.5 - field_of_view * 0.5).tan();
        let ri = 1.0 / (nr - fr);

        arr2(&[
            [f / a, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (nr + fr) * ri, -1.0],
            [0.0, 0.0, nr * fr * ri * 2.0, 0.0],
        ])
    }

    pub fn p_matrix_inv(&self) -> Array2<f32> {
        let p = self.p_matrix();
        arr2(&[
            [1.0 / p.row(0)[0], 0.0, 0.0, 0.0],
            [0.0, 1.0 / p.row(1)[1], 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0 / p.row(3)[2]],
            [0.0, 0.0, -1.0, p.row(2)[2] / p.row(3)[2]],
        ])
    }

    pub fn v_matrix(&self) -> Array2<f32> {
        self.v_matrix.clone()
    }

    pub fn v_matrix_inv(&self) -> Array2<f32> {
        let v = &self.v_matrix;
        arr2(&[
            [v.row(0)[0], v.row(1)[0], v.row(2)[0], 0.0],
            [v.row(0)[1], v.row(1)[1], v.row(2)[1], 0.0],
            [v.row(0)[2], v.row(1)[2], v.row(2)[2], 0.0],
            [
                -v.row(0).dot(&v.row(3)),
                -v.row(1).dot(&v.row(3)),
                -v.row(2).dot(&v.row(3)),
                1.0,
            ],
        ])
    }

    pub fn vp_matrix(&self) -> Array2<f32> {
        self.v_matrix().dot(&self.p_matrix())
    }

    pub fn vp_matrix_inv(&self) -> Array2<f32> {
        self.p_matrix_inv().dot(&self.v_matrix_inv())
    }
}
