use crate::arena::resource::ResourceId;
use crate::libs::color::Pallet;
use crate::libs::select_list::SelectList;
use std::rc::Rc;

#[derive(Clone)]
pub enum TableTool {
    Hr(Rc<String>),
    Selector,
    TableEditor,
    Pen(PenTool),
    Shape(SelectList<ShapeTool>),
    Eraser(EraserTool),
    Character(CharacterTool),
    Boxblock(BoxblockTool),
    Terranblock(TerranblockTool),
    Pointlight(PointlightTool),
    TerranblockEraser,
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
            Self::Boxblock(..) => "ブロック作成",
            Self::Pointlight(..) => "点光源",
            Self::Terranblock(..) => "地形",
            Self::TerranblockEraser => "地形消去",
        }
    }
}

#[derive(Clone)]
pub struct PenTool {
    pub line_width: f64,
    pub pallet: Pallet,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct CharacterTool {
    pub size: f64,
    pub height: f64,
    pub tex_id: Option<ResourceId>,
    pub name: String,
}

#[derive(Clone)]
pub struct BoxblockTool {
    pub size: [f64; 3],
    pub shape: crate::arena::block::boxblock::Shape,
    pub color: Pallet,
}

#[derive(Clone)]
pub struct TerranblockTool {
    pub color: Pallet,
    pub is_fillable: bool,
}

#[derive(Clone)]
pub struct PointlightTool {
    pub light_intensity: f64,
    pub light_attenation: f64,
    pub color: Pallet,
}
