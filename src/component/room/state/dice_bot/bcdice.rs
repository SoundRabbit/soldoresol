use crate::dice_bot::bcdice;

pub struct State {
    servers: Vec<String>,
    names: Option<bcdice::Names>,
}

impl State {
    pub fn new() -> Self {
        Self {
            servers: vec![],
            names: None,
        }
    }

    pub fn servers(&self) -> &Vec<String> {
        &self.servers
    }

    pub fn set_servers(&mut self, servers: Vec<String>) {
        self.servers = servers;
    }

    pub fn set_names(&mut self, names: bcdice::Names) {
        self.names = Some(names);
    }
}
