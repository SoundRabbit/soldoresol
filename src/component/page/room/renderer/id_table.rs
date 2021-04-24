use crate::arena::block::BlockId;
use crate::libs::clone_of::CloneOf;
use std::collections::HashMap;

pub enum ObjectId {
    None,
    Character(BlockId),
}

impl ObjectId {
    pub fn is_none(&self) -> bool {
        match &self {
            Self::None => true,
            _ => false,
        }
    }
}

impl CloneOf for ObjectId {
    fn clone_of(this: &Self) -> Self {
        match this {
            Self::None => Self::None,
            Self::Character(b_id) => Self::Character(BlockId::clone(b_id)),
        }
    }
}

impl std::fmt::Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "[None]"),
            Self::Character(b_id) => write!(f, "[Character: {}]", &b_id),
        }
    }
}

pub type IdTable = HashMap<u32, ObjectId>;
