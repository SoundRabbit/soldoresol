use crate::libs::color::Pallet;

#[derive(Clone)]
pub struct Pointlight {
    position: [f32; 3],
    light_intensity: f32,
    light_attenation: f32,
    color: Pallet,
}

impl Pointlight {
    pub fn new(position: [f32; 3]) -> Self {
        Self {
            position,
            light_attenation: 0.1,
            light_intensity: 0.5,
            color: Pallet::gray(0),
        }
    }

    pub fn position(&self) -> &[f32; 3] {
        &self.position
    }

    pub fn set_position(&mut self, position: [f32; 3]) {
        self.position = position;
    }

    pub fn light_attenation(&self) -> f32 {
        self.light_attenation
    }

    pub fn set_light_attenation(&mut self, light_attenation: f32) {
        self.light_attenation = light_attenation;
    }

    pub fn light_intensity(&self) -> f32 {
        self.light_intensity
    }

    pub fn set_light_intensity(&mut self, light_intensity: f32) {
        self.light_intensity = light_intensity;
    }

    pub fn color(&self) -> &Pallet {
        &self.color
    }

    pub fn set_color(&mut self, color: Pallet) {
        self.color = color;
    }
}
