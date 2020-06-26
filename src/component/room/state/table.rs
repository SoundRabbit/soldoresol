#[derive(Clone)]
pub struct Measure {
    line_width: f64,
    is_rounded: bool,
    is_share_result: bool,
}

#[derive(Clone)]
pub enum Tool {
    Selector,
    Pen,
    Eracer,
    Measure(Measure),
}

pub struct State {
    selecting_tool: Tool,
    measure_length: Option<f64>,
    last_mouse_position: [f64; 2],
    last_mouse_down_position: [f64; 2],
    last_mouse_up_position: [f64; 2],
    is_2d_mode: bool,
}

impl Tool {
    pub fn is_selector(&self) -> bool {
        match self {
            Self::Selector => true,
            _ => false,
        }
    }
    pub fn is_pen(&self) -> bool {
        match self {
            Self::Pen => true,
            _ => false,
        }
    }
    pub fn is_eracer(&self) -> bool {
        match self {
            Self::Eracer => true,
            _ => false,
        }
    }
    pub fn is_measure(&self) -> bool {
        match self {
            Self::Measure(..) => true,
            _ => false,
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            selecting_tool: Tool::Selector,
            measure_length: None,
            last_mouse_position: [0.0, 0.0],
            last_mouse_down_position: [0.0, 0.0],
            last_mouse_up_position: [0.0, 0.0],
            is_2d_mode: false,
        }
    }
}
