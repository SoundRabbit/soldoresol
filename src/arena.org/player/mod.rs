use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod player;

pub use player::Player;

type Table = Rc<RefCell<HashMap<Rc<String>, Player>>>;

pub struct ArenaRef {
    arena: Arena,
}

pub struct Arena {
    table: Table,
}

impl ArenaRef {
    pub fn clone(this: &Self) -> Self {
        Self {
            arena: Arena::clone(&this.arena),
        }
    }
}

impl std::ops::Deref for ArenaRef {
    type Target = Arena;
    fn deref(&self) -> &Self::Target {
        &self.arena
    }
}

impl Arena {
    fn clone(this: &Self) -> Self {
        Self {
            table: Rc::clone(&this.table),
        }
    }

    pub fn new() -> Self {
        Self {
            table: Rc::new(RefCell::new(map! {})),
        }
    }

    pub fn as_ref(&self) -> ArenaRef {
        let arena = Self::clone(self);
        ArenaRef { arena }
    }

    pub fn map<T>(&self, player_id: &Rc<String>, f: impl FnOnce(&Player) -> T) -> Option<T> {
        self.table.borrow().get(player_id).map(|p| f(p))
    }

    pub fn insert(&mut self, player_id: Rc<String>, player: Player) {
        self.table.borrow_mut().insert(player_id, player);
    }
}
