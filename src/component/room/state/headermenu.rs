pub enum State {
    None,
    Config,
    Resource,
}

impl State {
    pub fn is_config(&self) -> bool {
        match self {
            Self::Config => true,
            _ => false,
        }
    }

    pub fn is_resource(&self) -> bool {
        match self {
            Self::Resource => true,
            _ => false,
        }
    }
}
