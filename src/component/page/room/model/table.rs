pub enum TableTool {
    Hr(String),
    Selector,
    Pen,
    Shape,
    Eraser,
}

impl TableTool {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Eraser => "消しゴム",
            Self::Hr(..) => "",
            Self::Pen => "ペン",
            Self::Selector => "選択",
            Self::Shape => "図形",
        }
    }
}
