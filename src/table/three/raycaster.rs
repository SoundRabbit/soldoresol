use crate::libs::three;

pub struct Raycaster {
    coords: three::Vector2,
    raycaster: three::Raycaster,
}

impl Raycaster {
    pub fn new() -> Self {
        Self {
            coords: three::Vector2::new(0.0, 0.0),
            raycaster: three::Raycaster::new(),
        }
    }

    pub fn set_from_camera(&mut self, coords: &[f64; 2], camera: &three::Camera) {
        self.coords.set(coords[0], coords[1]);
        self.raycaster.set_from_camera(&self.coords, camera);
    }
}

impl std::ops::Deref for Raycaster {
    type Target = three::Raycaster;

    fn deref(&self) -> &Self::Target {
        &self.raycaster
    }
}
