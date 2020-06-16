use sainome;

pub type RunTime = sainome::RunTime<'static>;

pub fn new() -> RunTime {
    RunTime::new(rand)
}

fn rand(n: u32) -> u32 {
    (js_sys::Math::random() * n as f64).floor() as u32
}
