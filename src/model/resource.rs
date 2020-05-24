use std::collections::HashMap;
use std::rc::Rc;

pub enum Data {
    Image(Rc<web_sys::HtmlImageElement>),
}

pub struct Resource {
    data: HashMap<u128, Data>,
}

impl Resource {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, data_id: u128, data: Data) {
        self.data.insert(data_id, data);
    }

    pub fn get(&self, data_id: u128) -> Option<&Data> {
        self.data.get(&data_id)
    }

    pub fn get_as_image(&self, data_id: &u128) -> Option<Rc<web_sys::HtmlImageElement>> {
        self.data.get(data_id).and_then(|data| match data {
            Data::Image(image) => Some(Rc::clone(image)),
        })
    }

    pub fn get_images(&self) -> Vec<Rc<web_sys::HtmlImageElement>> {
        self.data
            .iter()
            .filter_map(|(data_id, _)| self.get_as_image(data_id))
            .collect()
    }
}
