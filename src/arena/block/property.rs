#[allow(unused_imports)]
use super::util::prelude::*;

use super::util::Pack;
use super::BlockMut;

pub type NumberValue = f64;
pub type NumberMin = NumberValue;
pub type NumberMid = NumberValue;
pub type NumberMax = NumberValue;

pub enum Value {
    Number(NumberValue),
    NumberMinMax(NumberValue, NumberMin, NumberMax),
    NumberMid(NumberValue, NumberMid),
    Normal(String),
    Note(String),
    Check(bool),
    Select(usize, Vec<String>),
}

impl Value {
    pub fn to_number(&self) -> Self {
        match self {
            Self::Number(val) => Self::Number(*val),
            Self::NumberMinMax(val, ..) => Self::Number(*val),
            Self::NumberMid(val, ..) => Self::Number(*val),
            Self::Normal(val) => val
                .parse::<NumberValue>()
                .map(|val| Self::Number(val))
                .unwrap_or_else(|_| Self::Number(0.0)),
            Self::Note(val) => val
                .parse::<NumberValue>()
                .map(|val| Self::Number(val))
                .unwrap_or_else(|_| Self::Number(0.0)),
            Self::Check(val) => Self::Number(if *val { 1.0 } else { 0.0 }),
            Self::Select(idx, ..) => Self::Number(*idx as NumberValue),
        }
    }

    pub fn to_number_min_max(&self) -> Self {
        match self {
            Self::Number(val) => Self::NumberMinMax(*val, 0.0, 100.0),
            Self::NumberMinMax(val, min, max) => Self::NumberMinMax(*val, *min, *max),
            Self::NumberMid(val, mid) => Self::NumberMinMax(*val, *mid / 2.0, *mid * 2.0),
            Self::Normal(val) => val
                .parse::<NumberValue>()
                .map(|val| Self::NumberMinMax(val, 0.0, 100.0))
                .unwrap_or_else(|_| Self::NumberMinMax(0.0, 0.0, 100.0)),
            Self::Note(val) => val
                .parse::<NumberValue>()
                .map(|val| Self::NumberMinMax(val, 0.0, 100.0))
                .unwrap_or_else(|_| Self::NumberMinMax(0.0, 0.0, 100.0)),
            Self::Check(val) => Self::NumberMinMax(if *val { 1.0 } else { 0.0 }, 0.0, 1.0),
            Self::Select(idx, list) => {
                Self::NumberMinMax(*idx as NumberValue, 0.0, (list.len() - 1) as NumberMax)
            }
        }
    }

    pub fn to_number_mid(&self) -> Self {
        match self {
            Self::Number(val) => Self::NumberMid(*val, 10.0),
            Self::NumberMinMax(val, min, max) => Self::NumberMid(*val, (*min + *max) / 2.0),
            Self::NumberMid(val, mid) => Self::NumberMid(*val, *mid),
            Self::Normal(val) => val
                .parse::<NumberValue>()
                .map(|val| Self::NumberMid(val, 10.0))
                .unwrap_or_else(|_| Self::NumberMid(0.0, 10.0)),
            Self::Note(val) => val
                .parse::<NumberValue>()
                .map(|val| Self::NumberMid(val, 10.0))
                .unwrap_or_else(|_| Self::NumberMid(0.0, 10.0)),
            Self::Check(val) => Self::NumberMid(if *val { 1.0 } else { 0.0 }, 0.5),
            Self::Select(idx, list) => {
                Self::NumberMid(*idx as NumberValue, (list.len() - 1) as NumberMax / 2.0)
            }
        }
    }

    pub fn to_normal(&self) -> Self {
        match self {
            Self::Number(val) => Self::Normal(format!("{}", *val)),
            Self::NumberMinMax(val, ..) => Self::Normal(format!("{}", *val)),
            Self::NumberMid(val, ..) => Self::Normal(format!("{}", *val)),
            Self::Normal(val) => Self::Normal(val.clone()),
            Self::Note(val) => Self::Normal(val.clone()),
            Self::Check(val) => Self::Normal(format!("{}", *val)),
            Self::Select(idx, ..) => Self::Normal(format!("{}", *idx)),
        }
    }

    pub fn to_note(&self) -> Self {
        match self {
            Self::Number(val) => Self::Note(format!("{}", *val)),
            Self::NumberMinMax(val, ..) => Self::Note(format!("{}", *val)),
            Self::NumberMid(val, ..) => Self::Note(format!("{}", *val)),
            Self::Normal(val) => Self::Note(val.clone()),
            Self::Note(val) => Self::Note(val.clone()),
            Self::Check(val) => Self::Note(format!("{}", *val)),
            Self::Select(idx, ..) => Self::Note(format!("{}", *idx)),
        }
    }

    pub fn to_check(&self) -> Self {
        match self {
            Self::Number(val) => Self::Check(*val != 0.0),
            Self::NumberMinMax(val, ..) => Self::Check(*val != 0.0),
            Self::NumberMid(val, ..) => Self::Check(*val != 0.0),
            Self::Normal(val) => Self::Check(*val == "true"),
            Self::Note(val) => Self::Check(*val == "true"),
            Self::Check(val) => Self::Check(*val),
            Self::Select(idx, ..) => Self::Check(*idx != 0),
        }
    }

    pub fn to_select(&self) -> Self {
        match self {
            Self::Number(val) => Self::Select(0, vec![format!("{}", *val)]),
            Self::NumberMinMax(val, ..) => Self::Select(0, vec![format!("{}", *val)]),
            Self::NumberMid(val, ..) => Self::Select(0, vec![format!("{}", *val)]),
            Self::Normal(val) => Self::Select(0, vec![val.clone()]),
            Self::Note(val) => Self::Select(0, vec![val.clone()]),
            Self::Check(val) => Self::Select(
                if *val { 1 } else { 0 },
                vec![String::from("false"), String::from("true")],
            ),
            Self::Select(idx, list) => Self::Select(*idx, list.clone()),
        }
    }
}

#[async_trait(?Send)]
impl Pack for Value {
    async fn pack(&self, is_deep: bool) -> JsValue {
        match self {
            Self::Number(v) => (object! {
                "_tag": "Number",
                "_val": JsValue::from(*v)
            })
            .into(),
            Self::NumberMinMax(v, i, a) => (object! {
                "_tag": "NumberMinMax",
                "_val": array![*v, *i, *a]
            })
            .into(),
            Self::NumberMid(v, m) => (object! {
                "_tag": "NumberMid",
                "_val": array![*v, *m]
            })
            .into(),
            Self::Normal(v) => (object! {
                "_tag": "Normal",
                "_val": v
            })
            .into(),
            Self::Note(v) => (object! {
                "_tag": "Note",
                "_val": v
            })
            .into(),
            Self::Check(v) => (object! {
                "_tag": "Check",
                "_val": *v
            })
            .into(),
            Self::Select(v, s) => (object! {
                "_tag": "Select",
                "_val": array![*v, s.pack(is_deep).await]
            })
            .into(),
        }
    }
}

pub enum DataView {
    Tabular,
    List,
}

#[async_trait(?Send)]
impl Pack for DataView {
    async fn pack(&self, _is_deep: bool) -> JsValue {
        match self {
            Self::Tabular => JsValue::from("Tabular"),
            Self::List => JsValue::from("List"),
        }
    }
}

pub struct Data {
    view: DataView,
    values: Vec<Vec<Value>>,
}

#[async_trait(?Send)]
impl Pack for Data {
    async fn pack(&self, is_deep: bool) -> JsValue {
        (object! {
            "view": self.view.pack(is_deep).await,
            "values": self.values.pack(is_deep).await
        })
        .into()
    }
}

impl Data {
    pub fn new() -> Self {
        Self {
            view: DataView::List,
            values: vec![vec![]],
        }
    }

    pub fn view(&self) -> &DataView {
        &self.view
    }

    pub fn set_view(&mut self, view: DataView) {
        self.view = view;
    }

    pub fn values(&self) -> &Vec<Vec<Value>> {
        &self.values
    }

    pub fn push_value(&mut self, row_idx: usize, value: Value) {
        if let Some(row) = self.values.get_mut(row_idx) {
            row.push(value);
        }
    }

    pub fn push_row(&mut self) {
        self.values.push(vec![]);
    }

    pub fn set_value(&mut self, row: usize, col: usize, value: Value) {
        if let Some(cols) = self.values.get_mut(row) {
            if let Some(val) = cols.get_mut(col) {
                *val = value;
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty() || self.values.iter().all(Vec::is_empty)
    }
}

pub enum PropertyView {
    Board,
    List,
}

#[async_trait(?Send)]
impl Pack for PropertyView {
    async fn pack(&self, _is_deep: bool) -> JsValue {
        match self {
            Self::Board => JsValue::from("Board"),
            Self::List => JsValue::from("List"),
        }
    }
}

block! {
    [pub Property(constructor, pack)]
    name: String = String::from("");
    data: Data = Data::new();
    view: PropertyView = PropertyView::List;
    children: Vec<BlockMut<Self>> = vec![];
}

impl Property {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn data(&self) -> &Data {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Data {
        &mut self.data
    }

    pub fn view(&self) -> &PropertyView {
        &self.view
    }

    pub fn set_view(&mut self, view: PropertyView) {
        self.view = view;
    }

    pub fn children(&self) -> &Vec<BlockMut<Self>> {
        &self.children
    }

    pub fn push_child(&mut self, child: BlockMut<Self>) {
        self.children.push(child);
    }
}
