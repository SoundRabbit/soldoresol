use crate::libs::random_id::U128Id;
use std::collections::HashMap;

#[derive(Clone)]
pub enum ObjectId {
    None,
    Character(U128Id, Surface),
    Boxblock(U128Id, Surface),
    Pointlight(U128Id, Surface),
    Terran(U128Id, Surface),
    Craftboard(U128Id, Surface),
}

// 点rを含むsベクトルとtベクトルが張る平面
#[derive(Clone)]
pub struct Surface {
    pub r: [f64; 3],
    pub s: [f64; 3],
    pub t: [f64; 3],
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct IdColor {
    id: u32,
}

pub struct IdTableBuilder {
    queue: Vec<U128Id>,
    object: HashMap<U128Id, HashMap<IdColor, ObjectId>>,
}

pub struct IdTable {
    offset: HashMap<U128Id, IdColor>,
    object: HashMap<IdColor, ObjectId>,
}

impl ObjectId {
    pub fn is_none(&self) -> bool {
        match &self {
            Self::None => true,
            _ => false,
        }
    }

    pub fn as_craftboard_id(&self) -> Option<&U128Id> {
        match self {
            Self::Craftboard(b_id, _) => Some(b_id),
            _ => None,
        }
    }
}

impl std::fmt::Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "[None]"),
            Self::Character(b_id, _) => write!(f, "[Character: {}]", &b_id),
            Self::Boxblock(b_id, _) => write!(f, "[Boxblock: {}]", &b_id),
            Self::Pointlight(b_id, _) => write!(f, "[Pointlight: {}]", &b_id),
            Self::Terran(b_id, _) => write!(f, "[Terran: {}]", &b_id),
            Self::Craftboard(b_id, _) => write!(f, "[Craftboard: {}]", &b_id),
        }
    }
}

impl ObjectId {
    pub fn is(&self, block_id: &U128Id) -> bool {
        match self {
            Self::None => false,
            Self::Character(b_id, _) => *b_id == *block_id,
            Self::Boxblock(b_id, _) => *b_id == *block_id,
            Self::Pointlight(b_id, _) => *b_id == *block_id,
            Self::Terran(b_id, _) => *b_id == *block_id,
            Self::Craftboard(b_id, _) => *b_id == *block_id,
        }
    }
}

impl Surface {
    pub fn n(&self) -> [f64; 3] {
        let n = [
            self.s[1] * self.t[2] - self.s[2] * self.t[1],
            self.s[2] * self.t[0] - self.s[0] * self.t[2],
            self.s[0] * self.t[1] - self.s[1] * self.t[0],
        ];
        let l = (n[0].powi(2) + n[1].powi(2) + n[2].powi(2)).sqrt();
        [n[0] / l, n[1] / l, n[2] / l]
    }
}

impl IdColor {
    pub fn to_f32array(&self) -> [f32; 4] {
        crate::libs::color::Color::from(self.id).to_f32array()
    }

    pub fn clone_with_offset(&self, offset: u32) -> Self {
        Self::from(self.id + offset)
    }

    pub fn value(&self) -> u32 {
        self.id
    }
}

impl From<u32> for IdColor {
    fn from(id: u32) -> Self {
        Self { id }
    }
}

impl IdTableBuilder {
    pub fn new() -> Self {
        Self {
            queue: vec![],
            object: map! {},
        }
    }

    pub fn insert(&mut self, block_id: &U128Id, delta_color: IdColor, object: ObjectId) {
        if let Some(objects) = self.object.get_mut(block_id) {
            objects.insert(delta_color, object);
        } else {
            let objects = map! {
                delta_color: object
            };
            self.queue.push(U128Id::clone(&block_id));
            self.object.insert(U128Id::clone(&block_id), objects);
        }
    }
}

impl IdTable {
    pub fn offset_color(&self, block_id: &U128Id) -> Option<&IdColor> {
        self.offset.get(block_id)
    }

    pub fn object_id(&self, id_color: &IdColor) -> Option<&ObjectId> {
        self.object.get(id_color)
    }
}

impl From<IdTableBuilder> for IdTable {
    fn from(mut builder: IdTableBuilder) -> Self {
        let mut offset = map! {};
        let mut object = map! {};
        let mut offset_color = 0;

        for block_id in builder.queue {
            if let Some(objects) = builder.object.remove(&block_id) {
                offset.insert(block_id, IdColor::from(offset_color));

                let mut delta_max = 0;
                for (delta_color, object_id) in objects {
                    delta_max = delta_max.max(delta_color.value());
                    object.insert(
                        delta_color.clone_with_offset(offset_color | 0xFF000000),
                        object_id,
                    );
                }

                offset_color += delta_max + 1;
            }
        }

        Self { offset, object }
    }
}
