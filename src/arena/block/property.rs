use super::BlockId;
use std::rc::Rc;

pub enum Value {
    None,
    Text(Rc<String>),
    MultiLineText(Rc<String>),
}

pub struct Property {
    name: Rc<String>,
    values: Vec<Value>,
    children: Vec<BlockId>,
}

impl Value {
    pub fn clone(this: &Self) -> Self {
        match this {
            Self::None => Self::None,
            Self::Text(x) => Self::Text(Rc::clone(x)),
            Self::MultiLineText(x) => Self::MultiLineText(Rc::clone(x)),
        }
    }
}

impl Property {
    pub fn new(name: Rc<String>) -> Self {
        Self {
            name,
            values: vec![],
            children: vec![],
        }
    }

    pub fn clone(this: &Self) -> Self {
        Self {
            name: Rc::clone(&this.name),
            values: this.values.iter().map(|x| Value::clone(x)).collect(),
            children: this.children.iter().map(|x| BlockId::clone(x)).collect(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.values.iter()
    }

    pub fn add_value(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn set_value(&mut self, idx: usize, value: Value) {
        self.values[idx] = value;
    }

    pub fn children(&self) -> impl Iterator<Item = &BlockId> {
        self.children.iter()
    }

    pub fn add_child(&mut self, prop_id: BlockId) {
        self.children.push(prop_id);
    }
}
