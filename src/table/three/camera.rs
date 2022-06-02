use crate::libs::three;
use ndarray::{arr1, arr2, Array1, Array2};

enum CameraKind {
    Perspective,
}

pub struct Camera {
    aspect: f64,
    position: [f64; 3],
    rotation: [f64; 3],
    kind: CameraKind,
    camera_perspective: three::PerspectiveCamera,
}

impl Camera {
    pub fn new() -> Self {
        let camera_perspective = three::PerspectiveCamera::new(60.0, 1.0, 1.0, 1000.0);
        let position = [0.0, 0.0, 20.0];
        let rotation = [
            0.25 * std::f64::consts::PI,
            0.0,
            0.03125 * std::f64::consts::PI,
        ];

        camera_perspective
            .position()
            .set(position[0], position[1], position[2]);

        Self {
            aspect: 1.0,
            position,
            rotation,
            kind: CameraKind::Perspective,
            camera_perspective,
        }
    }

    pub fn position(&self) -> &[f64; 3] {
        &self.position
    }

    pub fn set_position(&mut self, position: [f64; 3]) {
        self.position = position;
    }

    pub fn rotation(&self) -> &[f64; 3] {
        &self.rotation
    }

    pub fn set_x_axis_rotation(&mut self, x_axis_rotation: f64) {
        self.rotation[0] = x_axis_rotation;
    }

    pub fn set_z_axis_rotation(&mut self, z_axis_rotation: f64) {
        self.rotation[2] = z_axis_rotation;
    }

    pub fn set_aspect(&mut self, aspect: f64) {
        self.aspect = aspect;
    }

    pub fn update(&mut self) {
        let rotation = self.camera_perspective.rotation();
        rotation.set_order("ZXY");
        rotation.set_x(self.rotation[0]);
        rotation.set_z(self.rotation[2]);

        let moved = self.rot_x().dot(&arr1(&[
            self.position[0],
            self.position[1],
            self.position[2],
            1.0,
        ]));
        let moved = self.rot_z().dot(&moved);

        self.camera_perspective
            .position()
            .set(moved[0], moved[1], moved[2]);
        self.camera_perspective.set_aspect(self.aspect);
        self.camera_perspective.update_projection_matrix();
    }

    fn rot_x(&self) -> Array2<f64> {
        let (s, c) = self.rotation[0].sin_cos();
        arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, c, -s, 0.0],
            [0.0, s, c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    fn rot_z(&self) -> Array2<f64> {
        let (s, c) = self.rotation[2].sin_cos();
        arr2(&[
            [c, -s, 0.0, 0.0],
            [s, c, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl std::ops::Deref for Camera {
    type Target = three::Camera;
    fn deref(&self) -> &Self::Target {
        match &self.kind {
            CameraKind::Perspective => &self.camera_perspective,
        }
    }
}
