use crate::dicebot::bcdice;
use regex::Regex;

pub struct State {
    servers: Vec<String>,
    names: Option<bcdice::Names>,
    system_info: Option<bcdice::SystemInfo>,
    selected_server_idx: usize,
    prefixs: Vec<Regex>,
    default_prefixs: Vec<Regex>,
}

impl State {
    fn default_prefixs() -> Vec<Regex> {
        vec![
            Regex::new(r"\d+[bdu]\d*").unwrap(),
            Regex::new(r"c\(").unwrap(),
            Regex::new(r"choice\[").unwrap(),
        ]
    }

    pub fn new() -> Self {
        Self {
            servers: vec![],
            names: None,
            system_info: None,
            selected_server_idx: 0,
            prefixs: vec![],
            default_prefixs: Self::default_prefixs(),
        }
    }

    pub fn set_servers(&mut self, servers: Vec<String>) {
        self.selected_server_idx = (crate::random_id::u128val() % servers.len() as u128) as usize;
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
        self.prefixs.clear();
        for prefix in system_info.prefixs() {
            if let Ok(prefix) = Regex::new(&prefix.to_lowercase()) {
                self.prefixs.push(prefix);
            }
        }
        self.system_info = Some(system_info);
    }

    pub fn system_name(&self) -> String {
        self.system_info
            .as_ref()
            .map(|si| si.game_type().clone())
            .unwrap_or(String::new())
    }

    pub fn match_to_prefix(&self, text: &str) -> bool {
        let text = text.to_lowercase();
        self.default_prefixs
            .iter()
            .any(|regex| regex.is_match(&text))
            || self.prefixs.iter().any(|regex| regex.is_match(&text))
    }
}
