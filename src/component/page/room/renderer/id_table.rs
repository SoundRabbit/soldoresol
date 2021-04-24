use crate::arena::block::BlockId;
use crate::libs::clone_of::CloneOf;
use std::collections::HashMap;

pub enum ObjectId {
    None,
    Character(BlockId, Surface),
    Boxblock(BlockId, Surface),
}

#[derive(Clone)]
pub struct Surface {
    pub r: [f32; 3],
    pub s: [f32; 3],
    pub t: [f32; 3],
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
            Self::Character(b_id, s) => Self::Character(BlockId::clone(b_id), s.clone()),
            Self::Boxblock(b_id, s) => Self::Boxblock(BlockId::clone(b_id), s.clone()),
        }
    }
}

impl std::fmt::Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "[None]"),
            Self::Character(b_id, _) => write!(f, "[Character: {}]", &b_id),
            Self::Boxblock(b_id, _) => write!(f, "[Boxblock: {}]", &b_id),
        }
    }
}

impl ObjectId {
    pub fn eq(&self, block_id: &BlockId) -> bool {
        match self {
            Self::None => false,
            Self::Character(b_id, _) => *b_id == *block_id,
            Self::Boxblock(b_id, _) => *b_id == *block_id,
        }
    }
}

impl Surface {
    pub fn n(&self) -> [f32; 3] {
        let n = [
            self.s[1] * self.t[2] - self.s[2] * self.t[1],
            self.s[2] * self.t[0] - self.s[0] * self.t[2],
            self.s[0] * self.t[1] - self.s[1] * self.t[0],
        ];
        let l = (n[0].powi(2) + n[1].powi(2) + n[2].powi(2)).sqrt();
        [n[0] / l, n[1] / l, n[2] / l]
    }
}

pub type IdTable = HashMap<u32, ObjectId>;
