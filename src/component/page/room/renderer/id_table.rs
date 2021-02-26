use crate::arena::block::BlockId;
use std::collections::HashMap;

pub enum ObjectId {
    Character(BlockId),
}

pub type IdTable = HashMap<u32, ObjectId>;
