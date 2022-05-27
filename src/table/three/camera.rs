use crate::libs::three;

enum CameraKind {
    Perspective,
}

pub struct Camera {
    aspect: f64,
    kind: CameraKind,
    camera_perspective: three::PerspectiveCamera,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect: 1.0,
            kind: CameraKind::Perspective,
            camera_perspective: three::PerspectiveCamera::new(
                30.0 * std::f64::consts::PI / 180.0,
                1.0,
                1.0,
                1000.0,
            ),
        }
    }

    pub fn set_aspect(&self, aspect: f64) {
        self.camera_perspective.set_aspect(aspect);
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
