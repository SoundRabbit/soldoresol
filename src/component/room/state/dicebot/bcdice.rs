use crate::dicebot::bcdice;

pub struct State {
    servers: Vec<String>,
    names: Option<bcdice::Names>,
    system_info: Option<bcdice::SystemInfo>,
}

impl State {
    pub fn new() -> Self {
        Self {
            servers: vec![],
            names: None,
            system_info: None,
        }
    }

    pub fn servers(&self) -> &Vec<String> {
        &self.servers
    }

    pub fn set_servers(&mut self, servers: Vec<String>) {
        self.servers = servers;
    }

    pub fn names(&self) -> Option<&bcdice::Names> {
        self.names.as_ref()
    }

    pub fn set_names(&mut self, names: bcdice::Names) {
        self.names = Some(names);
    }

    pub fn system_info(&self) -> Option<&bcdice::SystemInfo> {
        self.system_info.as_ref()
    }

    pub fn set_system_info(&mut self, system_info: bcdice::SystemInfo) {
        self.system_info = Some(system_info);
    }
}
