use crate::dice_bot;
use regex::Regex;

pub struct State {
    run_time: dice_bot::RunTime,
    config: dice_bot::Config,
    regex: Regex,
}

impl State {
    pub fn new() -> Self {
        let mut run_time = dice_bot::new_run_time();
        let config = dice_bot::config();
        dice_bot::set_env(&config, &mut run_time);
        let regex = Regex::new(r"(.*)[\s\n　](.*)").unwrap();
        Self {
            run_time,
            config,
            regex,
        }
    }

    pub fn delimit<'a>(&self, text: &'a str) -> (&'a str, &'a str) {
        let caps = self.regex.captures(text).unwrap();
        let left = caps.get(1).map_or("", |m| m.as_str());
        let right = caps.get(2).map_or("", |m| m.as_str());
        (left, right)
    }
}
