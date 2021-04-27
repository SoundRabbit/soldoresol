use super::BlockId;
use crate::libs::clone_of::CloneOf;
use crate::libs::select_list::SelectList;

pub enum Value {
    None,
    Text(String),
    MultiLineText(String),
    ResourceMinMax { min: f64, val: f64, max: f64 },
    MappedList(SelectList<(String, String)>),
}

#[derive(Clone)]
pub enum ValueMode {
    List,
    Column,
}

pub struct Property {
    name: String,
    values: Vec<Value>,
    children: Vec<BlockId>,
    value_mode: ValueMode,
}

impl Value {
    pub fn clone(this: &Self) -> Self {
        match this {
            Self::None => Self::None,
            Self::Text(x) => Self::Text(x.clone()),
            Self::MultiLineText(x) => Self::MultiLineText(x.clone()),
            Self::ResourceMinMax { min, val, max } => Self::ResourceMinMax {
                min: *min,
                val: *val,
                max: *max,
            },
            Self::MappedList(x) => Self::MappedList(SelectList::clone_of(x)),
        }
    }

    pub async fn pack_to_toml(&self) -> toml::Value {
        let mut packed = toml::value::Table::new();

        match self {
            Self::None => {
                packed.insert(
                    String::from("_type"),
                    toml::Value::String(String::from("None")),
                );
            }
            Self::Text(x) => {
                packed.insert(
                    String::from("_type"),
                    toml::Value::String(String::from("Text")),
                );
                packed.insert(String::from("_payload"), toml::Value::String(x.clone()));
            }
            Self::MultiLineText(x) => {
                packed.insert(
                    String::from("_type"),
                    toml::Value::String(String::from("MultiLineText")),
                );
                packed.insert(String::from("_payload"), toml::Value::String(x.clone()));
            }
            Self::ResourceMinMax { min, val, max } => {
                packed.insert(
                    String::from("_type"),
                    toml::Value::String(String::from("ResourceMinMax")),
                );

                let payload = {
                    let mut payload = toml::value::Table::new();

                    payload.insert(String::from("min"), toml::Value::Float(*min));
                    payload.insert(String::from("val"), toml::Value::Float(*val));
                    payload.insert(String::from("max"), toml::Value::Float(*max));

                    payload
                };
                packed.insert(String::from("_payload"), toml::Value::Table(payload));
            }
            Self::MappedList(x) => {
                packed.insert(
                    String::from("_type"),
                    toml::Value::String(String::from("MappedList")),
                );
                let payload = {
                    let mut select_list = toml::value::Table::new();

                    select_list.insert(
                        String::from("_selected_idx"),
                        toml::Value::Integer(x.selected_idx() as i64),
                    );

                    let payload = {
                        let mut payload = toml::value::Array::new();

                        for x in x.iter() {
                            let mut pair = toml::value::Array::new();

                            pair.push(toml::Value::String(x.0.clone()));
                            pair.push(toml::Value::String(x.1.clone()));

                            payload.push(toml::Value::Array(pair));
                        }

                        payload
                    };
                    select_list.insert(String::from("_payload"), toml::Value::Array(payload));

                    select_list
                };
                packed.insert(String::from("payload"), toml::Value::Table(payload));
            }
        }

        toml::Value::Table(packed)
    }
}

impl Property {
    pub fn new(name: String) -> Self {
        Self {
            name,
            values: vec![],
            children: vec![],
            value_mode: ValueMode::List,
        }
    }

    pub fn clone(this: &Self) -> Self {
        Self {
            name: this.name.clone(),
            values: this.values.iter().map(|x| Value::clone(x)).collect(),
            children: this.children.iter().map(|x| BlockId::clone(x)).collect(),
            value_mode: this.value_mode.clone(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
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

    pub async fn pack_to_toml(&self) -> toml::Value {
        let mut packed = toml::value::Table::new();

        packed.insert(
            String::from("name"),
            toml::Value::String(self.name.to_string()),
        );
        match &self.value_mode {
            ValueMode::List => {
                packed.insert(
                    String::from("value_mode"),
                    toml::Value::String(String::from("List")),
                );
            }
            ValueMode::Column => {
                packed.insert(
                    String::from("value_mode"),
                    toml::Value::String(String::from("Column")),
                );
            }
        }

        let values = {
            let mut values = toml::value::Array::new();

            for value in self.values.iter() {
                values.push(value.pack_to_toml().await);
            }

            values
        };
        packed.insert(String::from("values"), toml::Value::Array(values));

        let children = {
            let mut children = toml::value::Array::new();

            for prop_id in self.children.iter() {
                children.push(toml::Value::String(prop_id.to_string()));
            }

            children
        };
        packed.insert(String::from("children"), toml::Value::Array(children));

        toml::Value::Table(packed)
    }

    pub fn flat_tree(block_arena: &super::Arena, prop_id: &BlockId) -> Vec<BlockId> {
        let mut children = vec![BlockId::clone(prop_id)];

        block_arena.map(prop_id, |prop: &Property| {
            for child_id in prop.children() {
                let child_children = Self::flat_tree(block_arena, child_id);

                for child_child_id in child_children {
                    children.push(child_child_id);
                }
            }
        });

        children
    }
}
