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
            values: vec![],
        }
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
    children: Vec<BlockMut<Self>> = vec![];
}
