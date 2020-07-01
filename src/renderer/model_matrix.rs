use ndarray::{arr2, Array2};
use std::convert::Into;

pub struct ModelMatrix {
    model_matrix: Array2<f32>,
}

impl ModelMatrix {
    pub fn new() -> Self {
        Self {
            model_matrix: arr2(&[
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
        }
    }

    pub fn with_movement(mut self, movement: &[f32; 3]) -> Self {
        let m = movement;
        let t = arr2(&[
            [1.0, 0.0, 0.0, m[0]],
            [0.0, 1.0, 0.0, m[1]],
            [0.0, 0.0, 1.0, m[2]],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        self.model_matrix = t.dot(&self.model_matrix);
        self
    }

    pub fn with_scale(mut self, scale: &[f32; 3]) -> Self {
        let s = scale;
        let t = arr2(&[
            [s[0], 0.0, 0.0, 0.0],
            [0.0, s[1], 0.0, 0.0],
            [0.0, 0.0, s[2], 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        self.model_matrix = t.dot(&self.model_matrix);
        self
    }

    pub fn with_x_axis_rotation(mut self, x_axis_rotation: f32) -> Self {
        let (s, c) = x_axis_rotation.sin_cos();
        let t = arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, c, -s, 0.0],
            [0.0, s, c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        self.model_matrix = t.dot(&self.model_matrix);
        self
    }

    #[allow(dead_code)]
    pub fn with_y_axis_rotation(mut self, y_axis_rotation: f32) -> Self {
        let (s, c) = y_axis_rotation.sin_cos();
        let t = arr2(&[
            [c, 0.0, s, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-s, 0.0, c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        self.model_matrix = t.dot(&self.model_matrix);
        self
    }

    pub fn with_z_axis_rotation(mut self, z_axis_rotation: f32) -> Self {
        let (s, c) = z_axis_rotation.sin_cos();
        let t = arr2(&[
            [c, -s, 0.0, 0.0],
            [s, c, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        self.model_matrix = t.dot(&self.model_matrix);
        self
    }
}

impl Into<Array2<f32>> for ModelMatrix {
    fn into(self) -> Array2<f32> {
        self.model_matrix
    }
}
