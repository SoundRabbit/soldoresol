use crate::libs::color::Pallet;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum Surface {
    PX,
    PY,
    PZ,
    NX,
    NY,
    NZ,
}

#[derive(Clone)]
pub struct TerranBlock {
    color: Pallet,
}

#[derive(Clone)]
pub struct Terran {
    list: VecDeque<[i32; 3]>,
    table: HashMap<[i32; 3], TerranBlock>,
}

impl Surface {
    pub fn iter() -> impl Iterator<Item = Self> {
        vec![Self::PX, Self::PY, Self::PZ, Self::NX, Self::NY, Self::NZ].into_iter()
    }
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
            list: VecDeque::new(),
            table: HashMap::new(),
        }
    }

    pub fn enqueue(&mut self, p: [i32; 3], c: TerranBlock) {
        if self.table.insert(p.clone(), c).is_none() {
            self.list.push_back(p);
        }
    }

    pub fn is_covered(&self, p: &[i32; 3], surface: &Surface) -> bool {
        let p = match surface {
            Surface::PX => [p[0] + 1, p[1] + 0, p[2] + 0],
            Surface::PY => [p[0] + 0, p[1] + 1, p[2] + 0],
            Surface::PZ => [p[0] + 0, p[1] + 0, p[2] + 1],
            Surface::NX => [p[0] - 1, p[1] + 0, p[2] + 0],
            Surface::NY => [p[0] + 0, p[1] - 1, p[2] + 0],
            Surface::NZ => [p[0] + 0, p[1] + 0, p[2] - 1],
        };
        self.table.get(&p).is_some()
    }

    pub fn is_adjasted(&self, p: &[i32; 3], surface: &Surface) -> bool {
        let p = match surface {
            Surface::PX => [p[0] + 1, p[1] + 0, p[2] + 0],
            Surface::PY => [p[0] + 0, p[1] + 1, p[2] + 0],
            Surface::PZ => [p[0] + 0, p[1] + 0, p[2] + 1],
            Surface::NX => [p[0] - 1, p[1] + 0, p[2] + 0],
            Surface::NY => [p[0] + 0, p[1] - 1, p[2] + 0],
            Surface::NZ => [p[0] + 0, p[1] + 0, p[2] - 1],
        };
        self.table.get(&p).is_some()
    }

    pub fn table(&self) -> &HashMap<[i32; 3], TerranBlock> {
        &self.table
    }

    pub fn remove_all(&mut self) -> HashMap<[i32; 3], TerranBlock> {
        let mut x = HashMap::new();
        std::mem::swap(&mut self.table, &mut x);
        self.list.clear();
        x
    }

    pub fn dequeue(&mut self) -> Option<([i32; 3], TerranBlock)> {
        self.list
            .pop_front()
            .and_then(|p| self.table.remove(&p).map(|x| (p, x)))
    }

    pub fn remove_at(&mut self, p: &[i32; 3]) {
        self.table.remove(p);
        self.list = self.list.drain(..).filter(|x| x != p).collect();
    }
}
