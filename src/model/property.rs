pub enum PropertyValue {
    None,
    Num(f64),
    Str(String),
    Children(Vec<Property>),
}

pub struct Property {
    name: String,
    value: PropertyValue,
}

impl Property {
    pub fn new_as_none() -> Self {
        Self {
            name: "".into(),
            value: PropertyValue::None,
        }
    }

    pub fn new_as_num() -> Self {
        Self {
            name: "".into(),
            value: PropertyValue::Num(0.0),
        }
    }

    pub fn new_as_str() -> Self {
        Self {
            name: "".into(),
            value: PropertyValue::Str("".into()),
        }
    }

    pub fn new_as_parent() -> Self {
        Self {
            name: "".into(),
            value: PropertyValue::Children(vec![]),
        }
    }

    pub fn get(&self, idx: usize) -> Option<&Self> {
        if let PropertyValue::Children(children) = &self.value {
            children.get(idx)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Self> {
        if let PropertyValue::Children(children) = &mut self.value {
            children.get_mut(idx)
        } else {
            None
        }
    }

    pub fn get_with_address(&self, address: &Vec<usize>) -> Option<&Self> {
        self.impl_get_with_address(address, 0)
    }

    fn impl_get_with_address(&self, address: &Vec<usize>, idx: usize) -> Option<&Self> {
        let child_pos = address[idx];
        if idx < address.len() - 1 {
            if let Some(child) = self.get(child_pos) {
                child.impl_get_with_address(address, idx + 1)
            } else {
                None
            }
        } else {
            self.get(child_pos)
        }
    }

    pub fn get_mut_with_address(&mut self, address: &Vec<usize>) -> Option<&mut Self> {
        self.impl_get_mut_with_address(address, 0)
    }

    fn impl_get_mut_with_address(&mut self, address: &Vec<usize>, idx: usize) -> Option<&mut Self> {
        let child_pos = address[idx];
        if idx < address.len() - 1 {
            if let Some(child) = self.get_mut(child_pos) {
                child.impl_get_mut_with_address(address, idx + 1)
            } else {
                None
            }
        } else {
            self.get_mut(child_pos)
        }
    }
}
