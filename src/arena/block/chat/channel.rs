use super::super::BlockId;

pub struct Channel {
    name: String,
    messages: Vec<BlockId>,
}

pub enum ChannelType {
    Public,
    Private { client_id: String },
}

impl Channel {
    pub fn new(name: String) -> Self {
        Self {
            name,
            messages: vec![],
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl ChannelType {
    pub fn is_public(&self) -> bool {
        match self {
            Self::Public => true,
            _ => false,
        }
    }

    pub fn is_private(&self) -> bool {
        match self {
            Self::Private { .. } => true,
            _ => false,
        }
    }
}
