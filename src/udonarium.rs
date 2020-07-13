use xmltree::{Element, XMLNode};

pub use data::Data;

pub struct Character {
    data: Data,
}

impl Character {
    pub fn from_str(text: &str) -> Option<Self> {
        Element::parse(text.as_bytes()).ok().and_then(|element| {
            if element.name == "character" {
                let mut data = None;
                for node in element.children {
                    match node {
                        XMLNode::Element(element) => {
                            if element.name == "data" {
                                data = Data::from_element(element);
                                break;
                            }
                        }
                        _ => (),
                    }
                }
                data.map(|data| Self { data })
            } else {
                None
            }
        })
    }

    pub fn data(&self) -> &Data {
        &self.data
    }
}

pub mod data {
    use xmltree::{Element, XMLNode};

    #[derive(Clone, Debug)]
    pub enum Value {
        Children(Vec<Data>),
        Text(String),
        None,
    }

    #[derive(Clone, Debug)]
    pub struct Data {
        name: String,
        type_: String,
        value: Value,
    }

    impl Value {
        pub fn from_node(nodes: Vec<XMLNode>) -> Option<Self> {
            let mut text = None;
            let mut children = vec![];
            for node in nodes {
                match node {
                    XMLNode::Element(element) => {
                        if let Some(data) = Data::from_element(element) {
                            children.push(data);
                        }
                    }
                    XMLNode::Text(suf) => {
                        text = Some(text.map(|pre| pre + suf.as_str()).unwrap_or(suf));
                    }
                    _ => (),
                }
            }
            if text.is_some() && !children.is_empty() {
                None
            } else if let Some(text) = text {
                Some(Value::Text(text))
            } else if !children.is_empty() {
                Some(Value::Children(children))
            } else {
                Some(Value::None)
            }
        }
    }

    impl Data {
        pub fn from_str(text: &str) -> Option<Self> {
            Element::parse(text.as_bytes())
                .ok()
                .and_then(Self::from_element)
        }

        pub fn from_element(mut element: Element) -> Option<Self> {
            if element.name == "data" {
                let name = element.attributes.remove("name").unwrap_or(String::new());
                let type_ = element.attributes.remove("type").unwrap_or(String::new());
                Value::from_node(element.children).map(|value| Self { name, type_, value })
            } else {
                None
            }
        }

        pub fn find(&self, name: &str) -> Option<&Value> {
            if self.name == name {
                Some(&self.value)
            } else if let Value::Children(children) = &self.value {
                children.iter().find_map(|child| child.find(name))
            } else {
                None
            }
        }
    }
}
