use super::super::BlockId;
use std::collections::HashSet;
use std::rc::Rc;

pub enum ChannelPermission {
    EveryOne,
    Players(HashSet<Rc<String>>),
}

impl ChannelPermission {
    pub fn clone(this: &Self) -> Self {
        match this {
            Self::EveryOne => Self::EveryOne,
            Self::Players(ps) => Self::Players(ps.iter().map(|p| Rc::clone(p)).collect()),
        }
    }
}

pub enum ChannelType {
    Public,
    Private {
        client_id: Rc<String>,
        read: ChannelPermission,
        write: ChannelPermission,
    },
}

impl ChannelType {
    pub fn clone(this: &Self) -> Self {
        match this {
            Self::Public => Self::Public,
            Self::Private {
                client_id,
                read,
                write,
            } => Self::Private {
                client_id: Rc::clone(client_id),
                read: ChannelPermission::clone(read),
                write: ChannelPermission::clone(write),
            },
        }
    }

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

    pub fn clone(this: &Self) -> Self {
        Self {
            channel_type: ChannelType::clone(&this.channel_type),
            name: Rc::clone(&this.name),
            messages: this
                .messages
                .iter()
                .map(|b_id| BlockId::clone(b_id))
                .collect(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
