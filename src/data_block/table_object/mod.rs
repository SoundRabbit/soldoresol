use super::Block;
use super::BlockId;

pub mod character;
pub mod tablemask;

pub use character::Character;
pub use tablemask::Tablemask;

trait TableObject: Block {
    fn size(&self) -> &[f32; 3];
    fn set_size(&mut self, size: [f32; 3]);
    fn position(&self) -> &[f32; 3];
    fn set_position(&mut self, position: [f32; 3]);
    fn property_id(&self) -> &BlockId;
}
