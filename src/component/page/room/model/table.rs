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

pub struct PenTool {
    pub line_width: f64,
    pub pallet: Pallet,
    pub alpha: u8,
}

impl CloneRef for PenTool {
    fn clone(this: &Self) -> Self {
        Self {
            line_width: this.line_width,
            pallet: this.pallet,
            alpha: this.alpha,
        }
    }
}

pub enum ShapeTool {
    Line,
    Rect,
    Ellipse,
}

impl ShapeTool {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Line => "直線",
            Self::Rect => "長方形",
            Self::Ellipse => "楕円",
        }
    }
}

impl CloneRef for ShapeTool {
    fn clone(this: &Self) -> Self {
        match this {
            Self::Line => Self::Line,
            Self::Rect => Self::Rect,
            Self::Ellipse => Self::Ellipse,
        }
    }
}
