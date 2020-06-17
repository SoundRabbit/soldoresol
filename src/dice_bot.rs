use sainome;

pub type RunTime = sainome::RunTime<'static>;
pub type ExecEnv<'a> = sainome::ExecEnv<'a>;

pub fn new_run_time() -> RunTime {
    RunTime::new(rand)
}

pub fn new_exec_env<'a>() -> ExecEnv<'a> {
    ExecEnv::new()
}

fn rand(n: u32) -> u32 {
    (js_sys::Math::random() * n as f64).floor() as u32
}

pub mod bc_dice_bot {
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
