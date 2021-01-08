use crate::libs::random_id::U128Id;
use std::collections::HashMap;

pub struct ModelessList<T> {
    table: HashMap<U128Id, (usize, T)>,
    max_z_index: usize,
    node_index: Vec<Option<U128Id>>,
}

impl<T> ModelessList<T> {
    pub fn new() -> Self {
        Self {
            table: map! {},
            max_z_index: 0,
            node_index: vec![],
        }
    }

    pub fn push(&mut self, m: T) -> U128Id {
        let modeless_id = U128Id::new();
        self.table
            .insert(U128Id::clone(&modeless_id), (self.max_z_index + 1, m));
        self.max_z_index += 1;
        if let Some(vacancy) = self.node_index.iter().position(|x| x.is_none()) {
            self.node_index[vacancy] = Some(U128Id::clone(&modeless_id));
        } else {
            self.node_index.push(Some(U128Id::clone(&modeless_id)));
        }
        modeless_id
    }

    pub fn remove(&mut self, modeless_id: &U128Id) -> Option<T> {
        if let Some(idx) = self.node_index.iter().position(|m_id| {
            m_id.as_ref()
                .map(|m_id| *modeless_id == *m_id)
                .unwrap_or(false)
        }) {
            self.node_index[idx] = None;
        }
        self.table.remove(modeless_id).map(|t| t.1)
    }

    pub fn focus(&mut self, modeless_id: &U128Id) -> bool {
        if let Some(t) = self.table.get_mut(modeless_id) {
            t.0 = self.max_z_index + 1;
            self.max_z_index += 1;
            true
        } else {
            false
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Option<(U128Id, usize, &T)>> {
        self.node_index
            .iter()
            .map(|maybe_modeless_id| {
                maybe_modeless_id.as_ref().and_then(|modeless_id| {
                    self.table
                        .get(modeless_id)
                        .map(|t| (U128Id::clone(&modeless_id), t.0, &t.1))
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}
