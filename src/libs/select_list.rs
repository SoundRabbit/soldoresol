pub struct SelectList<T> {
    payload: Vec<T>,
    selected_idx: usize,
}

impl<T> SelectList<T> {
    pub fn new(payload: Vec<T>, selected_idx: usize) -> Self {
        Self {
            payload,
            selected_idx,
        }
    }

    pub fn selected_idx(&self) -> usize {
        self.selected_idx
    }

    pub fn set_selected_idx(&mut self, idx: usize) {
        self.selected_idx = idx;
    }

    pub fn selected(&self) -> Option<&T> {
        self.payload.get(self.selected_idx)
    }

    pub fn selected_mut(&mut self) -> Option<&mut T> {
        self.payload.get_mut(self.selected_idx)
    }

    pub fn remove(&mut self, idx: usize) -> Option<T> {
        if self.payload.len() > idx {
            if self.selected_idx + 1 == self.payload.len() && self.selected_idx > 0 {
                self.selected_idx -= 1;
            }
            Some(self.payload.remove(idx))
        } else {
            None
        }
    }

    pub fn push(&mut self, v: T) {
        if self.payload.len() < 1 {
            self.payload.push(v);
            self.selected_idx = 0;
        } else {
            self.payload.push(v);
        }
    }
}

impl<T> std::ops::Deref for SelectList<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.payload
    }
}

impl<T> std::ops::DerefMut for SelectList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.payload
    }
}
