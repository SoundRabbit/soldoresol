use crate::libs::clone_ref::CloneRef;
use crate::libs::color::Pallet;
use crate::libs::select_list::SelectList;
use std::rc::Rc;

pub enum TableTool {
    Hr(Rc<String>),
    Selector,
    Pen(PenTool),
    Shape(SelectList<ShapeTool>),
    Eraser,
}

impl TableTool {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Eraser => "消しゴム",
            Self::Hr(..) => "",
            Self::Pen(..) => "ペン",
            Self::Selector => "選択",
            Self::Shape(..) => "図形",
        }
    }
}

impl CloneRef for TableTool {
    fn clone(this: &Self) -> Self {
        match this {
            Self::Hr(x) => Self::Hr(<Rc<_> as Clone>::clone(x)),
            Self::Selector => Self::Selector,
            Self::Pen(x) => Self::Pen(PenTool::clone(x)),
            Self::Shape(x) => Self::Shape(SelectList::clone(x)),
            Self::Eraser => Self::Eraser,
        }
    }
}

#[derive(Clone)]
pub struct PenTool {
    pub line_width: f64,
    pub pallet: Pallet,
}

pub enum ShapeTool {
    Line(LineShapeTool),
    Rect(FillShapeTool),
    Ellipse(FillShapeTool),
}

impl ShapeTool {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Line(..) => "直線",
            Self::Rect(..) => "長方形",
            Self::Ellipse(..) => "楕円",
        }
    }

    pub fn set_line(&mut self, line: LineShapeTool) {
        match self {
            Self::Line(x) => {
                *x = line;
            }
            _ => {}
        }
    }

    pub fn set_fill(&mut self, fill: FillShapeTool) {
        match self {
            Self::Ellipse(x) | Self::Rect(x) => {
                *x = fill;
            }
            _ => {}
        }
    }
}

impl CloneRef for ShapeTool {
    fn clone(this: &Self) -> Self {
        match this {
            Self::Line(x) => Self::Line(LineShapeTool::clone(x)),
            Self::Rect(x) => Self::Rect(FillShapeTool::clone(x)),
            Self::Ellipse(x) => Self::Ellipse(FillShapeTool::clone(x)),
        }
    }
}

#[derive(Clone)]
pub struct LineShapeTool {
    pub line_width: f64,
    pub pallet: Pallet,
}

#[derive(Clone)]
pub struct FillShapeTool {
    pub line_width: f64,
    pub line_pallet: Pallet,
    pub fill_pallet: Pallet,
}
