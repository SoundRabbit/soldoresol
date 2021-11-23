uses! {
    super::BlockMut;
    super::util::Pack;
    crate::libs::color::Pallet;
}

block! {
    [pub Craftboard(constructor, pack)]
    (position): [f64; 3];
    name: String = String::new();
    size: [f64; 2] = [10.0, 10.0];
    is_bind_to_grid: bool = true;
    is_showing_grid: bool = true;
    terran_height: f64 = 1.0;
    grid_color: Pallet = Pallet::gray(9).a(100);
    env_light_intensity: f64 = 1.0;
}

impl Craftboard {
    pub fn position(&self) -> &[f64; 3] {
        &self.position
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn size(&self) -> &[f64; 2] {
        &self.size
    }
    pub fn is_bind_to_grid(&self) -> bool {
        self.is_bind_to_grid
    }
    pub fn is_showing_grid(&self) -> bool {
        self.is_showing_grid
    }
    pub fn grid_color(&self) -> &Pallet {
        &self.grid_color
    }
    pub fn env_light_intensity(&self) -> f64 {
        self.env_light_intensity
    }
}
