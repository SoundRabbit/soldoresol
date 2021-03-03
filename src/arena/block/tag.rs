use crate::libs::color::Pallet;
use std::rc::Rc;

pub struct Tag {
    name: Rc<String>,
    color: Pallet,
}

impl Tag {
    pub fn new() -> Self {
        Self {
            name: Rc::new(String::from("")),
            color: Pallet::blue(5).a(100),
        }
    }

    pub fn clone(this: &Self) -> Self {
        Self {
            name: Rc::clone(&this.name),
            color: this.color,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Rc::new(name);
    }

    pub fn color(&self) -> &Pallet {
        &self.color
    }

    pub fn set_color(&mut self, pallet: Pallet) {
        self.color = pallet;
    }
}
