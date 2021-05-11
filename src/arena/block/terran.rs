use crate::libs::color::Pallet;
use std::collections::HashMap;

#[derive(Clone)]
pub struct TerranBlock {
    color: Pallet,
}

#[derive(Clone)]
pub struct Terran {
    table: HashMap<[i32; 3], TerranBlock>,
}

impl TerranBlock {
    pub fn new(color: Pallet) -> Self {
        Self { color }
    }

    pub fn color(&self) -> &Pallet {
        &self.color
    }
}

impl Terran {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn insert(&mut self, p: [i32; 3], c: TerranBlock) {
        self.table.insert(p, c);
    }

    pub fn is_covered(&self, p: &[i32; 3], surface: usize) -> bool {
        let p = match surface % 6 {
            0 => [p[0] + 1, p[1] + 0, p[2] + 0],
            1 => [p[0] + 0, p[1] + 1, p[2] + 0],
            2 => [p[0] + 0, p[1] + 0, p[2] + 1],
            3 => [p[0] - 1, p[1] + 0, p[2] + 0],
            4 => [p[0] + 0, p[1] - 1, p[2] + 0],
            5 => [p[0] + 0, p[1] + 0, p[2] - 1],
            _ => unreachable!(),
        };
        self.table.get(&p).is_some()
    }

    pub fn is_adjasted(&self, p: &[i32; 3], surface: usize) -> bool {
        let p = match surface % 6 {
            0 => [p[0] + 1, p[1] + 0, p[2] + 0],
            1 => [p[0] + 0, p[1] + 1, p[2] + 0],
            2 => [p[0] + 0, p[1] + 0, p[2] + 1],
            3 => [p[0] - 1, p[1] + 0, p[2] + 0],
            4 => [p[0] + 0, p[1] - 1, p[2] + 0],
            5 => [p[0] + 0, p[1] + 0, p[2] - 1],
            _ => unreachable!(),
        };
        self.table.get(&p).is_some()
    }

    pub fn table(&self) -> &HashMap<[i32; 3], TerranBlock> {
        &self.table
    }
}
