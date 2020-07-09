use crate::dicebot::bcdice;

pub struct State {
    servers: Vec<String>,
    names: Option<bcdice::Names>,
    system_info: Option<bcdice::SystemInfo>,
    selected_server_idx: usize,
}

impl State {
    pub fn new() -> Self {
        Self {
            servers: vec![],
            names: None,
            system_info: None,
            selected_server_idx: 0,
        }
    }

    pub fn set_servers(&mut self, servers: Vec<String>) {
        self.servers = servers;
    }

    pub fn server(&self) -> String {
        self.servers
            .get(self.selected_server_idx)
            .map(|s| s.clone())
            .unwrap_or(String::new())
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
