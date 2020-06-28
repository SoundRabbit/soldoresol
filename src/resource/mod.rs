use crate::random_id;
use std::collections::HashMap;
use std::ops::Deref;

mod data;

pub use data::Data;

pub type ResourceId = u128;

pub struct Resource {
    data: HashMap<ResourceId, Data>,
}

impl Resource {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, data: Data) -> ResourceId {
        let resource_id = random_id::u128val();
        self.assign(resource_id, data);
        resource_id
    }

    pub fn assign(&mut self, resource_id: ResourceId, data: Data) {
        self.data.insert(resource_id, data);
    }

    pub fn all(&self) -> impl Iterator<Item = (&ResourceId, &Data)> {
        self.data.iter()
    }
}

impl Deref for Resource {
    type Target = HashMap<ResourceId, Data>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
