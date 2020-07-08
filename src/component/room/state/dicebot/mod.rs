use crate::dicebot;
use regex::Regex;

pub mod bcdice;

pub struct State {
    run_time: dicebot::RunTime,
    config: dicebot::Config,
    regex: Regex,
    bcdice: bcdice::State,
}

impl State {
    pub fn new() -> Self {
        let mut run_time = dicebot::new_run_time();
        let config = dicebot::config();
        dicebot::set_env(&config, &mut run_time);
        let regex = Regex::new(r"(.*)[\s\n　](.*)").unwrap();
        Self {
            run_time,
            config,
            regex,
            bcdice: bcdice::State::new(),
        }
    }

    pub fn delimit<'a>(&self, text: &'a str) -> (&'a str, &'a str) {
        if let Some(caps) = self.regex.captures(text) {
            let left = caps.get(1).map_or("", |m| m.as_str());
            let right = caps.get(2).map_or("", |m| m.as_str());
            (left, right)
        } else {
            (text, "")
        }
    }

    pub fn bcdice(&self) -> &bcdice::State {
        &self.bcdice
    }

    pub fn bcdice_mut(&mut self) -> &mut bcdice::State {
        &mut self.bcdice
    }
}
