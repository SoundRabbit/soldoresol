use crate::dice_bot;

pub struct State {
    run_time: dice_bot::RunTime,
    config: dice_bot::Config,
}

impl State {
    pub fn new() -> Self {
        let mut run_time = dice_bot::new_run_time();
        let config = dice_bot::config();
        dice_bot::set_env(&config, &mut run_time);
        Self { run_time, config }
    }
}
