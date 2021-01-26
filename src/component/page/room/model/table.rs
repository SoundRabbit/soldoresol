use super::super::super::molecule::color_pallet::Pallet;
use crate::libs::clone_ref::CloneRef;
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
    pub alpha: u8,
}

pub enum ShapeTool {
    Line(LineShapeTool),
    Rect(RectShapeTool),
    Ellipse,
}

impl ShapeTool {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Line(..) => "直線",
            Self::Rect(..) => "長方形",
            Self::Ellipse => "楕円",
        }
    }
}

impl CloneRef for ShapeTool {
    fn clone(this: &Self) -> Self {
        match this {
            Self::Line(x) => Self::Line(LineShapeTool::clone(x)),
            Self::Rect(x) => Self::Rect(RectShapeTool::clone(x)),
            Self::Ellipse => Self::Ellipse,
        }
    }
}

#[derive(Clone)]
pub struct LineShapeTool {
    pub line_width: f64,
    pub pallet: Pallet,
    pub alpha: u8,
}

#[derive(Clone)]
pub struct RectShapeTool {
    pub line_width: f64,
    pub line_pallet: Pallet,
    pub line_alpha: u8,
    pub fill_pallet: Pallet,
    pub fill_alpha: u8,
}
