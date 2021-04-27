use super::BlockId;
use crate::libs::clone_of::CloneOf;
use crate::libs::select_list::SelectList;
use std::rc::Rc;

pub enum Value {
    None,
    Text(Rc<String>),
    MultiLineText(Rc<String>),
    ResourceMinMax { min: f64, val: f64, max: f64 },
    MappedList(SelectList<(Rc<String>, Rc<String>)>),
}

#[derive(Clone)]
pub enum ValueMode {
    List,
    Column,
}

pub struct Property {
    name: Rc<String>,
    values: Vec<Value>,
    children: Vec<BlockId>,
    value_mode: ValueMode,
}

impl Value {
    pub fn clone(this: &Self) -> Self {
        match this {
            Self::None => Self::None,
            Self::Text(x) => Self::Text(Rc::clone(x)),
            Self::MultiLineText(x) => Self::MultiLineText(Rc::clone(x)),
            Self::ResourceMinMax { min, val, max } => Self::ResourceMinMax {
                min: *min,
                val: *val,
                max: *max,
            },
            Self::MappedList(x) => Self::MappedList(SelectList::clone_of(x)),
        }
    }
}

impl Property {
    pub fn new(name: Rc<String>) -> Self {
        Self {
            name,
            values: vec![],
            children: vec![],
            value_mode: ValueMode::List,
        }
    }

    pub fn clone(this: &Self) -> Self {
        Self {
            name: Rc::clone(&this.name),
            values: this.values.iter().map(|x| Value::clone(x)).collect(),
            children: this.children.iter().map(|x| BlockId::clone(x)).collect(),
            value_mode: this.value_mode.clone(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: Rc<String>) {
        self.name = name;
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

    pub fn remove_value(&mut self, idx: usize) {
        if idx < self.values.len() {
            self.values.remove(idx);
        }
    }

    pub fn children(&self) -> impl Iterator<Item = &BlockId> {
        self.children.iter()
    }

    pub fn add_child(&mut self, prop_id: BlockId) {
        self.children.push(prop_id);
    }

    pub fn value_mode(&self) -> &ValueMode {
        &self.value_mode
    }

    pub fn set_value_mode(&mut self, value_mode: ValueMode) {
        self.value_mode = value_mode;
    }
}
