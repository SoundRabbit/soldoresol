use crate::arena::{block, resource, BlockRef};
use crate::libs::random_id::U128Id;
use std::rc::Rc;

#[derive(Clone)]
pub enum TableTool {
    Selecter(Rc<Selecter>),
    Craftboard(Rc<Craftboard>),
    Pen(Rc<Pen>),
    Eraser(Rc<Eraser>),
    Character(Rc<Character>),
    Boxblock(Rc<Boxblock>),
    TerranBlock(Rc<TerranBlock>),
    Textboard(Rc<Textboard>),
    ComponentAllocater(Rc<ComponentAllocater>),
}

#[derive(Clone)]
pub enum Selecter {
    Point,
    Range,
}

#[derive(Clone)]
pub struct Craftboard {
    pub size: [f64; 3],
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
    pub size: f64,
    pub tex_size: f64,
    pub color: crate::libs::color::Pallet,
    pub texture: Option<BlockRef<resource::ImageData>>,
}

#[derive(Clone)]
pub struct Boxblock {
    pub color: crate::libs::color::Pallet,
    pub size: [f64; 3],
    pub texture: Option<BlockRef<resource::BlockTexture>>,
    pub shape: block::boxblock::Shape,
}

#[derive(Clone)]
pub struct TerranBlock {
    pub kind: TerranBlockKind,
    pub allocater_state: TerranBlockAllocater,
}

#[derive(Clone, PartialEq, Eq)]
pub enum TerranBlockKind {
    Allocater,
    Eraser,
}

#[derive(Clone)]
pub struct TerranBlockAllocater {
    pub color: crate::libs::color::Pallet,
}

#[derive(Clone)]
pub struct Textboard {}

#[derive(Clone)]
pub struct ComponentAllocater {
    pub component: U128Id,
}
