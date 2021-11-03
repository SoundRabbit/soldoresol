use super::super::BlockId;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Clone)]
pub enum ChannelPermission {
    EveryOne,
    Players(HashSet<Rc<String>>),
}

#[derive(Clone)]
pub enum ChannelType {
    Public,
    Private {
        client_id: Rc<String>,
        read: ChannelPermission,
        write: ChannelPermission,
    },
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

#[derive(Clone)]
pub struct Channel {
    channel_type: ChannelType,
    name: Rc<String>,
    messages: Vec<BlockId>,
}

impl Channel {
    pub fn new(name: String, channel_type: ChannelType) -> Self {
        Self {
            channel_type,
            name: Rc::new(name),
            messages: vec![],
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
