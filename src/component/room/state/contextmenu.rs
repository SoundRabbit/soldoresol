use crate::block::BlockId;
use std::ops::{Deref, DerefMut};

pub enum Contextmenu {
    Default,
    Character(BlockId),
    Tablemask(BlockId),
    Area(BlockId),
}

pub struct State {
    grobal_position: [f64; 2],
    canvas_position: [f64; 2],
    payload: Contextmenu,
}

impl State {
    pub fn new(grobal_position: [f64; 2], canvas_position: [f64; 2], payload: Contextmenu) -> Self {
        Self {
            grobal_position,
            canvas_position,
            payload,
        }
    }

    pub fn grobal_position(&self) -> &[f64; 2] {
        &self.grobal_position
    }

    pub fn canvas_position(&self) -> &[f64; 2] {
        &self.canvas_position
    }
}

impl Deref for State {
    type Target = Contextmenu;
    fn deref(&self) -> &Self::Target {
        &self.payload
    }
}

impl DerefMut for State {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.payload
    }
}
