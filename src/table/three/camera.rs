use crate::libs::three;

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
        Self {
            aspect: 1.0,
            position: [0.0, 0.0, 20.0],
            rotation: [
                0.25 * std::f64::consts::PI,
                0.0,
                0.03125 * std::f64::consts::PI,
            ],
            kind: CameraKind::Perspective,
            camera_perspective: three::PerspectiveCamera::new(
                60.0 * std::f64::consts::PI / 180.0,
                1.0,
                1.0,
                1000.0,
            ),
        }
    }

    pub fn position(&self) -> &[f64; 3] {
        &self.position
    }

    pub fn set_position(&mut self, position: [f64; 3]) {
        self.camera_perspective
            .position()
            .set(position[0], position[1], position[2]);
        self.position = position;
    }

    pub fn rotation(&self) -> &[f64; 3] {
        &self.rotation
    }

    pub fn set_x_axis_rotation(&mut self, x_axis_rotation: f64) {
        self.camera_perspective.rotation().set_x(x_axis_rotation);
        self.rotation[0] = x_axis_rotation;
    }

    pub fn set_z_axis_rotation(&mut self, z_axis_rotation: f64) {
        self.camera_perspective.rotation().set_z(z_axis_rotation);
        self.rotation[2] = z_axis_rotation;
    }

    pub fn set_aspect(&mut self, aspect: f64) {
        self.aspect = aspect;
        self.camera_perspective.set_aspect(aspect);
        self.camera_perspective.update_projection_matrix();
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
