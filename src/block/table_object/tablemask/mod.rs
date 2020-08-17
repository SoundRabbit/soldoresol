use super::{Block, Field};
use crate::Color;
use crate::Promise;
use wasm_bindgen::{prelude::*, JsCast};

pub mod ellipse;
pub mod line;
pub mod rect;

pub use ellipse::Ellipse;
pub use line::Line;
pub use rect::Rect;

#[derive(Clone)]
pub enum Tablemask {
    Ellipse(Ellipse),
    Line(Line),
    Rect(Rect),
}

impl Tablemask {}

impl Block for Tablemask {
    fn pack(&self) -> Promise<JsValue> {
        todo!();
    }
    fn unpack(_: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        todo!();
    }
}
