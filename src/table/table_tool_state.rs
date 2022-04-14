use crate::arena::BlockKind;
use crate::libs::random_id::U128Id;

pub enum TableToolState {
    None,
    Selecter(SelecterState),
}

impl TableToolState {
    pub fn selecter_mut(&mut self) -> &mut SelecterState {
        match self {
            Self::Selecter(state) => state,
            _ => {
                *self = Self::Selecter(SelecterState {
                    grabbed_object: None,
                });
                self.selecter_mut()
            }
        }
    }
}

pub struct SelecterState {
    pub grabbed_object: Option<(BlockKind, U128Id)>,
}
