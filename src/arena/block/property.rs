#[allow(unused_imports)]
use super::util::prelude::*;
use super::util::{Pack, PackDepth};
use super::BlockMut;

pub type NumberValue = f64;
pub type NumberMin = NumberValue;
pub type NumberMid = NumberValue;
pub type NumberMax = NumberValue;

#[derive(Clone)]
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
    pub fn to_string_with_option(&self, option: Option<&String>) -> String {
        let option = option.map(|option| option.as_str());
        match (self, option) {
            (Self::Number(val), None) => val.to_string(),
            (Self::Number(val), Some("val")) => val.to_string(),

            (Self::NumberMinMax(val, ..), None) => val.to_string(),
            (Self::NumberMinMax(val, ..), Some("val")) => val.to_string(),
            (Self::NumberMinMax(_, min, ..), Some("min")) => min.to_string(),
            (Self::NumberMinMax(_, _, max), Some("max")) => max.to_string(),
            (Self::NumberMid(val, ..), None) => val.to_string(),
            (Self::NumberMid(val, ..), Some("val")) => val.to_string(),
            (Self::NumberMid(_, mid), Some("mid")) => mid.to_string(),
            (Self::Normal(text), None) => text.clone(),
            (Self::Normal(text), Some("val")) => text.clone(),
            (Self::Note(text), None) => text.clone(),
            (Self::Note(text), Some("val")) => text.clone(),

            (Self::Check(val), None) => val.to_string(),
            (Self::Check(val), Some("val")) => val.to_string(),

            (Self::Select(idx, vals), None) => vals
                .get(*idx)
                .map(|val| val.clone())
                .unwrap_or_else(|| String::new()),
            (Self::Select(idx, vals), Some("val")) => vals
                .get(*idx)
                .map(|val| val.clone())
                .unwrap_or_else(|| String::new()),

            (Self::Select(idx, ..), Some("idx")) => idx.to_string(),
            (Self::Select(_, vals), Some(idx)) if idx.parse::<usize>().is_ok() => vals
                .get(idx.parse::<usize>().unwrap())
                .map(|val| val.clone())
                .unwrap_or_else(|| String::new()),

            _ => String::new(),
        }
    }

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

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Check(val) => write!(f, "{}", val),
            Self::Normal(val) => write!(f, "{}", val),
            Self::Note(val) => write!(f, "{}", val),
            Self::Number(val) => write!(f, "{}", val),
            Self::NumberMid(val, ..) => write!(f, "{}", val),
            Self::NumberMinMax(val, ..) => write!(f, "{}", val),
            Self::Select(idx, list) => list
                .get(*idx)
                .map(|val| write!(f, "{}", val))
                .unwrap_or_else(|| write!(f, "")),
        }
    }
}

#[async_trait(?Send)]
impl Pack for Value {
    async fn pack(&self, pack_depth: PackDepth) -> JsValue {
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
                "_val": array![*v, s.pack(pack_depth).await]
            })
            .into(),
        }
    }

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        let data = data.dyn_ref::<crate::libs::js_object::Object>()?;
        let tag = data.get("_tag")?.as_string()?;
        let val = data.get("_val")?;
        match tag.as_str() {
            "Number" => val
                .as_f64()
                .map(|v| Box::new(Self::Number(v as NumberValue))),
            "NumberMinMax" => {
                let val = js_sys::Array::from(&val).to_vec();
                let val_0 = val.get(0)?.as_f64()?;
                let val_1 = val.get(0)?.as_f64()?;
                let val_2 = val.get(0)?.as_f64()?;
                Some(Box::new(Self::NumberMinMax(
                    val_0 as NumberValue,
                    val_1 as NumberValue,
                    val_2 as NumberValue,
                )))
            }
            "NumberMid" => {
                let val = js_sys::Array::from(&val).to_vec();
                let val_0 = val.get(0)?.as_f64()?;
                let val_1 = val.get(0)?.as_f64()?;
                Some(Box::new(Self::NumberMid(
                    val_0 as NumberValue,
                    val_1 as NumberValue,
                )))
            }
            "Normal" => val.as_string().map(|v| Box::new(Self::Normal(v))),
            "Note" => val.as_string().map(|v| Box::new(Self::Note(v))),
            "Check" => val.as_bool().map(|v| Box::new(Self::Check(v))),
            "Select" => {
                let val = js_sys::Array::from(&val).to_vec();
                let val_0 = val.get(0)?.as_f64()? as usize;
                let val_1 = js_sys::Array::from(unwrap!(val.get(1); None).as_ref()).to_vec();
                let mut list = vec![];
                for item in val_1 {
                    if let Some(item) = item.as_string() {
                        list.push(item);
                    }
                }
                Some(Box::new(Self::Select(val_0, list)))
            }
            _ => None,
        }
    }
}

pub enum DataView {
    Tabular,
    List,
}

#[async_trait(?Send)]
impl Pack for DataView {
    async fn pack(&self, _: PackDepth) -> JsValue {
        match self {
            Self::Tabular => JsValue::from("Tabular"),
            Self::List => JsValue::from("List"),
        }
    }

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        match data.as_string()?.as_str() {
            "Tabular" => Some(Box::new(Self::Tabular)),
            "List" => Some(Box::new(Self::List)),
            _ => None,
        }
    }
}

pub struct Data {
    view: DataView,
    values: Vec<Vec<Value>>,
}

#[async_trait(?Send)]
impl Pack for Data {
    async fn pack(&self, pack_depth: PackDepth) -> JsValue {
        (object! {
            "view": self.view.pack(pack_depth).await,
            "values": self.values.pack(pack_depth).await
        })
        .into()
    }

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        let data = data.dyn_ref::<crate::libs::js_object::Object>()?;
        let view = DataView::unpack(
            unwrap!(data.get("view"); None).as_ref(),
            ArenaMut::clone(&arena),
        )
        .await?;
        let view = *view;
        let items = js_sys::Array::from(unwrap!(data.get("values"); None).as_ref()).to_vec();
        let mut values = vec![];

        for col_items in items {
            let col_items = js_sys::Array::from(&col_items).to_vec();
            let mut col_values = vec![];
            for item in col_items {
                if let Some(item) = Value::unpack(&item, ArenaMut::clone(&arena)).await {
                    col_values.push(*item);
                }
            }
            values.push(col_values);
        }

        Some(Box::new(Self { view, values }))
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

    pub fn get_value0(&self) -> Option<&Value> {
        self.get_value1(0)
    }

    pub fn get_value1(&self, mut idx: usize) -> Option<&Value> {
        for row_offset in 0..self.values.len() {
            if let Some(value) = self.get_value2(row_offset, idx) {
                return Some(value);
            }
            idx -= self.values[row_offset].len();
        }

        None
    }

    pub fn get_value2(&self, r: usize, c: usize) -> Option<&Value> {
        self.values.get(r).and_then(|cols| cols.get(c))
    }

    fn remove_empty_row(&mut self) {
        let mut row_idx = 1;
        while row_idx < self.values.len() {
            if self.values[row_idx].len() == 0 {
                self.values.remove(row_idx);
            } else {
                row_idx += 1;
            }
        }
    }

    pub fn remove_value1(&mut self, mut idx: usize) -> bool {
        for row_offset in 0..self.values.len() {
            if self.remove_value2(row_offset, idx) {
                return true;
            }
            if let Some(cols) = self.values.get(row_offset) {
                idx -= cols.len();
            }
        }

        false
    }

    pub fn remove_value2(&mut self, r: usize, c: usize) -> bool {
        if let Some(cols) = self.values.get_mut(r) {
            if c < cols.len() {
                cols.remove(c);
                self.remove_empty_row();
                return true;
            }
        }

        false
    }

    pub fn ref_value(&self, args: Vec<&(String, Option<String>)>) -> Option<Value> {
        if args.len() == 0 {
            return self.get_value0().map(|val| val.clone());
        } else if args.len() == 1 {
            if args[0].1.is_none() {
                if let Ok(idx) = args[0].0.parse::<usize>() {
                    return self.get_value1(idx).map(|val| val.clone());
                }
            }
        } else {
            if args[0].1.is_none() && args[1].1.is_none() {
                if let (Ok(r), Ok(c)) = (args[0].0.parse::<usize>(), args[1].0.parse::<usize>()) {
                    return self.get_value2(r, c).map(|val| val.clone());
                }
            }
        }

        None
    }
}

pub enum PropertyView {
    Board,
    List,
}

#[async_trait(?Send)]
impl Pack for PropertyView {
    async fn pack(&self, _: PackDepth) -> JsValue {
        match self {
            Self::Board => JsValue::from("Board"),
            Self::List => JsValue::from("List"),
        }
    }

    async fn unpack(data: &JsValue, _arena: ArenaMut) -> Option<Box<Self>> {
        let data = data.as_string()?;
        match data.as_str() {
            "Board" => Some(Box::new(Self::Board)),
            "List" => Some(Box::new(Self::List)),
            _ => None,
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

    pub fn remove_child(&mut self, block_id: &U128Id) {
        if let Some(child_idx) = self
            .children
            .iter()
            .position(|child| child.id() == *block_id)
        {
            self.children.remove(child_idx);
        }
    }

    pub fn ref_value(
        &self,
        mut name: Vec<&String>,
        args: Vec<&(String, Option<String>)>,
    ) -> Option<Value> {
        if name.len() > 1 {
            let name = if self.name == *name[0] {
                name.drain(1..).collect()
            } else {
                name
            };

            self.ref_children_value(name, args)
        } else if name.len() == 1 {
            if self.name == *name[0] {
                self.data.ref_value(args)
            } else {
                self.ref_children_value(name, args)
            }
        } else {
            None
        }
    }

    fn ref_children_value(
        &self,
        name: Vec<&String>,
        args: Vec<&(String, Option<String>)>,
    ) -> Option<Value> {
        for child in &self.children {
            if let Some(value) = child
                .map(|child| child.ref_value(name.clone(), args.clone()))
                .unwrap_or_default()
            {
                return Some(value);
            }
        }

        None
    }
}
