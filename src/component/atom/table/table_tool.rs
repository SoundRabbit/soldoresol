use crate::arena::{block, resource, BlockMut};
use std::rc::Rc;

#[derive(Clone)]
pub enum TableTool {
    Selecter(Rc<Selecter>),
    Pen(Rc<Pen>),
    Eraser(Rc<Eraser>),
    Character(Rc<Character>),
    Boxblock(Rc<Boxblock>),
}

#[derive(Clone)]
pub enum Selecter {
    Point,
    Range,
}

#[derive(Clone)]
pub struct Pen {
    pub color: crate::libs::color::Pallet,
    pub width: f64,
}

#[derive(Clone)]
pub struct Eraser {
    pub width: f64,
}

#[derive(Clone)]
pub struct Character {
    pub name: String,
    pub texture: Option<BlockMut<resource::ImageData>>,
}

#[derive(Clone)]
pub struct Boxblock {
    pub color: crate::libs::color::Pallet,
    pub size: [f64; 3],
    pub texture: Option<BlockMut<resource::BlockTexture>>,
    pub shape: block::boxblock::Shape,
}
