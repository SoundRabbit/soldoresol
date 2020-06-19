use regex::Regex;
use sainome;
use serde::Deserialize;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

thread_local! { static RUN_TIME: Rc<sainome::RunTime<'static>> = Rc::new(sainome::RunTime::new(rand)); }

pub type RunTime = sainome::RunTime<'static>;

#[derive(Deserialize)]
pub struct Config {
    def: Vec<HashMap<String, String>>,
    pattern: Vec<Pattern>,
}

#[derive(Deserialize)]
pub struct Pattern {
    capture: String,
    replace: String,
}

pub fn new_run_time() -> RunTime {
    RunTime::new(rand)
}

pub fn config() -> Config {
    toml::from_str(include_str!("sword_world.toml")).unwrap()
}

pub fn set_env(config: &Config, run_time: &mut RunTime) {
    for defs in &config.def {
        for (name, def) in defs {
            let mut def = def.clone();
            def.retain(|c| c != '\n');
            let code = name.clone() + ":=" + def.as_str();
            sainome::exec_mut(code.as_str(), run_time).unwrap();
        }
    }
}

pub fn cmd_with_config(cmd: String, config: &Config) -> String {
    for pattern in &config.pattern {
        if let Ok(capture) = Regex::new(&pattern.capture) {
            if capture.is_match(cmd.as_str()) {
                return capture
                    .replace_all(cmd.as_str(), pattern.replace.as_str())
                    .to_string();
            }
        }
    }
    cmd
}

fn rand(n: u32) -> u32 {
    (js_sys::Math::random() * n as f64).floor() as u32
}

pub mod bc_dice {
    use serde::Deserialize;
    use std::ops::Deref;

    #[derive(Deserialize, Debug)]
    pub struct ImplNames {
        name: String,
        system: String,
        sort_key: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct Names {
        names: Vec<ImplNames>,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ImplSystemInfo {
        name: String,
        game_type: String,
        sort_key: String,
        prefix: Vec<String>,
        info: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct SystemInfo {
        systeminfo: ImplSystemInfo,
    }

    impl ImplNames {
        pub fn name(&self) -> &String {
            &self.name
        }

        pub fn system(&self) -> &String {
            &self.system
        }
    }

    impl Names {
        pub fn sorted(mut self) -> Self {
            self.names.sort_by(|a, b| a.name.cmp(&b.name));
            self
        }
    }

    impl Deref for Names {
        type Target = Vec<ImplNames>;
        fn deref(&self) -> &Self::Target {
            &self.names
        }
    }

    impl ImplSystemInfo {
        pub fn name(&self) -> &String {
            &self.name
        }

        pub fn game_type(&self) -> &String {
            &self.game_type
        }

        pub fn prefix(&self) -> &Vec<String> {
            &self.prefix
        }

        pub fn info(&self) -> &String {
            &self.info
        }
    }

    impl Deref for SystemInfo {
        type Target = ImplSystemInfo;
        fn deref(&self) -> &Self::Target {
            &self.systeminfo
        }
    }
}
