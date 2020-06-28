use crate::block::BlockId;

#[derive(Clone)]
pub enum Tool {
    Selector,
    Pen,
    Eracer,
    Measure,
    Area { line_width: f64, is_rounded: bool },
    Route(BlockId),
}

#[derive(Clone)]
pub enum Focused {
    None,
    Character(BlockId),
    Tablemask(BlockId),
}

pub struct State {
    selecting_tool: Tool,
    info: Vec<(String, String)>,
    last_mouse_position: [f32; 2],
    last_mouse_down_position: [f32; 2],
    last_mouse_up_position: [f32; 2],
    is_2d_mode: bool,
    focused: Focused,
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
            Self::Measure => true,
            _ => false,
        }
    }
    pub fn is_area(&self) -> bool {
        match self {
            Self::Area { .. } => true,
            _ => false,
        }
    }
    pub fn is_route(&self) -> bool {
        match self {
            Self::Route(..) => true,
            _ => false,
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            selecting_tool: Tool::Selector,
            info: vec![],
            last_mouse_position: [0.0, 0.0],
            last_mouse_down_position: [0.0, 0.0],
            last_mouse_up_position: [0.0, 0.0],
            is_2d_mode: false,
            focused: Focused::None,
        }
    }

    pub fn selecting_tool(&self) -> &Tool {
        &self.selecting_tool
    }

    pub fn set_selecting_tool(&mut self, tool: Tool) {
        self.selecting_tool = tool;
    }

    pub fn info(&self) -> &Vec<(String, String)> {
        &self.info
    }

    pub fn set_info(&mut self, info: Vec<(String, String)>) {
        self.info = info;
    }

    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    pub fn add_info(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.info.push((key.into(), value.into()));
    }

    pub fn last_mouse_position(&self) -> &[f32; 2] {
        &self.last_mouse_position
    }

    pub fn set_last_mouse_position(&mut self, pos: [f32; 2]) {
        self.last_mouse_position = pos;
    }

    pub fn last_mouse_down_position(&self) -> &[f32; 2] {
        &self.last_mouse_down_position
    }

    pub fn set_last_mouse_down_position(&mut self, pos: [f32; 2]) {
        self.last_mouse_down_position = pos;
    }

    pub fn set_last_mouse_up_position(&mut self, pos: [f32; 2]) {
        self.last_mouse_up_position = pos;
    }

    pub fn is_2d_mode(&self) -> bool {
        self.is_2d_mode
    }

    pub fn focused(&self) -> &Focused {
        &self.focused
    }

    pub fn set_focused(&mut self, focused: Focused) {
        self.focused = focused
    }
}
