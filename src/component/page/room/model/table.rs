use crate::arena::resource::ResourceId;
use crate::libs::clone_of::CloneOf;
use crate::libs::color::Pallet;
use crate::libs::select_list::SelectList;
use std::rc::Rc;

pub enum TableTool {
    Hr(Rc<String>),
    Selector,
    TableEditor,
    Pen(PenTool),
    Shape(SelectList<ShapeTool>),
    Eraser(EraserTool),
    Character(CharacterTool),
}

impl TableTool {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Eraser(..) => "消しゴム",
            Self::Hr(..) => "",
            Self::Pen(..) => "ペン",
            Self::Selector => "選択",
            Self::Shape(..) => "図形",
            Self::TableEditor => "テーブル編集",
            Self::Character(..) => "キャラクター作成",
        }
    }
}

impl CloneOf for TableTool {
    fn clone_of(this: &Self) -> Self {
        match this {
            Self::Hr(x) => Self::Hr(Rc::clone_of(x)),
            Self::Selector => Self::Selector,
            Self::TableEditor => Self::TableEditor,
            Self::Pen(x) => Self::Pen(PenTool::clone_of(x)),
            Self::Shape(x) => Self::Shape(SelectList::clone_of(x)),
            Self::Eraser(x) => Self::Eraser(EraserTool::clone_of(x)),
            Self::Character(x) => Self::Character(CharacterTool::clone_of(x)),
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

impl CloneOf for ShapeTool {
    fn clone_of(this: &Self) -> Self {
        match this {
            Self::Line(x) => Self::Line(LineShapeTool::clone_of(x)),
            Self::Rect(x) => Self::Rect(FillShapeTool::clone_of(x)),
            Self::Ellipse(x) => Self::Ellipse(FillShapeTool::clone_of(x)),
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

#[derive(Clone)]
pub struct EraserTool {
    pub line_width: f64,
    pub alpha: u8,
}

pub struct CharacterTool {
    pub size: f32,
    pub tex_scale: f32,
    pub tex_id: Option<ResourceId>,
    pub name: String,
}

impl CloneOf for CharacterTool {
    fn clone_of(this: &Self) -> Self {
        Self {
            size: this.size,
            tex_scale: this.tex_scale,
            tex_id: this.tex_id.as_ref().map(|x| ResourceId::clone(x)),
            name: this.name.clone(),
        }
    }
}
