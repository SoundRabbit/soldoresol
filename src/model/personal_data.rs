use crate::data_block::BlockId;

pub struct PersonalData {
    name: String,
    icon: Option<BlockId>,
}

impl PersonalData {
    pub fn new() -> Self {
        Self {
            name: "Player".into(),
            icon: None,
        }
    }

    pub fn with_peer_id(mut self, peer_id: &str) -> Self {
        self.name = self.name + "_" + peer_id;
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn icon(&self) -> Option<&BlockId> {
        self.icon.as_ref()
    }

    pub fn set_icon(&mut self, icon: Option<BlockId>) {
        self.icon = icon;
    }
}
