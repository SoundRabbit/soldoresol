use super::atom::{
    btn::{self, Btn},
    fa,
};
use super::constant;
use super::util::styled::{Style, Styled};
use super::util::Prop;
use crate::libs::color::color_system;
use crate::libs::select_list::SelectList;
use kagura::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Props {
    pub size: [f32; 2],
    pub loc: [f32; 2],
    pub z_index: usize,
    pub container_element: Option<Prop<web_sys::Element>>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            size: [0.3, 0.3],
            loc: [0.0, 0.0],
            z_index: 0,
            container_element: None,
        }
    }
}

pub enum Msg {
    NoOp,
    DragStart {
        page_x: i32,
        page_y: i32,
        drag_type: DragType,
    },
    DragEnd,
    Drag {
        page_x: i32,
        page_y: i32,
    },
}

pub enum On {
    Focus,
}

pub struct Modeless {
    size: [f32; 2],
    loc: [f32; 2],
    z_index: usize,
    dragging: Option<([i32; 2], DragType)>,
    container_element: Option<Prop<web_sys::Element>>,
}

enum DragType {
    Move,
    Resize(DragDirection),
}

enum DragDirection {
    Top,
    Left,
    Bottom,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl std::fmt::Display for DragDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Top => write!(f, "top"),
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
            Self::Bottom => write!(f, "bottom"),
            Self::TopLeft => write!(f, "top-left"),
            Self::TopRight => write!(f, "right-bottom"),
            Self::BottomLeft => write!(f, "bottom-left"),
            Self::BottomRight => write!(f, "bottom-right"),
        }
    }
}

impl Constructor for Modeless {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            z_index: props.z_index,
            loc: props.loc,
            size: props.size,
            dragging: None,
            container_element: props.container_element,
        }
    }
}

macro_rules! on_drag {
    ($dragging:expr) => {{
        let dragging = $dragging;
        move |e| {
            e.stop_propagation();
            if dragging {
                Msg::Drag {
                    page_x: e.page_x(),
                    page_y: e.page_y(),
                }
            } else {
                Msg::NoOp
            }
        }
    }};
}

impl Component for Modeless {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.z_index = props.z_index;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::DragStart {
                page_x,
                page_y,
                drag_type,
            } => {
                self.dragging = Some(([page_x, page_y], drag_type));
                Cmd::sub(On::Focus)
            }
            Msg::DragEnd => {
                self.dragging = None;
                Cmd::none()
            }
            Msg::Drag { page_x, page_y } => {
                if let Some((dragging, container_element)) =
                    join_some!(self.dragging.as_mut(), self.container_element.as_ref())
                {
                    let mov_x = (page_x - dragging.0[0]) as f32;
                    let mov_y = (page_y - dragging.0[1]) as f32;
                    let container_width = container_element.client_width() as f32;
                    let container_height = container_element.client_height() as f32;

                    let mov_x = mov_x / container_width;
                    let mov_y = mov_y / container_height;

                    match &dragging.1 {
                        DragType::Move => {
                            self.loc[0] += mov_x;
                            self.loc[1] += mov_y;
                        }
                        DragType::Resize(DragDirection::Top) => {
                            self.loc[1] += mov_y;
                            self.size[1] -= mov_y;
                        }
                        DragType::Resize(DragDirection::Left) => {
                            self.loc[0] += mov_x;
                            self.size[0] -= mov_x;
                        }
                        DragType::Resize(DragDirection::Bottom) => {
                            self.size[1] += mov_y;
                        }
                        DragType::Resize(DragDirection::Right) => {
                            self.size[0] += mov_x;
                        }
                        DragType::Resize(DragDirection::TopLeft) => {
                            self.loc[0] += mov_x;
                            self.loc[1] += mov_y;
                            self.size[0] -= mov_x;
                            self.size[1] -= mov_y;
                        }
                        DragType::Resize(DragDirection::TopRight) => {
                            self.loc[1] += mov_y;
                            self.size[0] += mov_x;
                            self.size[1] -= mov_y;
                        }
                        DragType::Resize(DragDirection::BottomLeft) => {
                            self.loc[0] += mov_x;
                            self.size[0] -= mov_x;
                            self.size[1] += mov_y;
                        }
                        DragType::Resize(DragDirection::BottomRight) => {
                            self.size[0] += mov_x;
                            self.size[1] += mov_y;
                        }
                    }

                    if self.loc[0] < 0.0 {
                        self.loc[0] = 0.0;
                    }
                    if self.loc[1] < 0.0 {
                        self.loc[1] = 0.0;
                    }
                    if self.loc[0] + self.size[0] >= 1.0 {
                        self.loc[0] = 1.0 - self.size[0];
                    }
                    if self.loc[1] + self.size[1] >= 1.0 {
                        self.loc[1] = 1.0 - self.size[1];
                    }
                    if self.size[0] < 0.1 {
                        self.size[0] = 0.1;
                    }
                    if self.size[1] < 0.1 {
                        self.size[1] = 0.1;
                    }

                    dragging.0[0] = page_x;
                    dragging.0[1] = page_y;
                }
                Cmd::none()
            }
        }
    }

    fn render(&self, children: Vec<Html>) -> Html {
        Self::styled(Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .style("z-index", format!("{}", self.z_index))
                .style("left", format!("{}%", self.loc[0] * 100.0))
                .style("top", format!("{}%", self.loc[1] * 100.0))
                .style("width", format!("{}%", self.size[0] * 100.0))
                .style("height", format!("{}%", self.size[1] * 100.0)),
            Events::new()
                .on_mousedown(|e| {
                    e.stop_propagation();
                    Msg::DragStart {
                        page_x: e.page_x(),
                        page_y: e.page_y(),
                        drag_type: DragType::Move,
                    }
                })
                .on_mouseup(|_| Msg::DragEnd)
                .on_mousemove(on_drag!(self.dragging.is_some()))
                .on_mouseleave(on_drag!(self.dragging.is_some())),
            vec![
                Html::div(
                    Attributes::new().class(Self::class("container")),
                    Events::new(),
                    children,
                ),
                self.render_rsz(DragDirection::Top),
                self.render_rsz(DragDirection::TopLeft),
                self.render_rsz(DragDirection::Left),
                self.render_rsz(DragDirection::BottomLeft),
                self.render_rsz(DragDirection::Bottom),
                self.render_rsz(DragDirection::BottomRight),
                self.render_rsz(DragDirection::Right),
                self.render_rsz(DragDirection::TopRight),
            ],
        ))
    }
}

impl Modeless {
    fn render_rsz(&self, drag_direction: DragDirection) -> Html {
        Html::div(
            Attributes::new().class(Self::class(&format!("rsz-{}", &drag_direction))),
            Events::new().on_mousedown(|e| {
                e.stop_propagation();
                Msg::DragStart {
                    page_x: e.page_x(),
                    page_y: e.page_y(),
                    drag_type: DragType::Resize(drag_direction),
                }
            }),
            vec![],
        )
    }
}

impl Styled for Modeless {
    fn style() -> Style {
        style! {
            "base" {
                "position": "absolute";
                "overflow": "visible";
            }

            "container" {
                "width": "100%";
                "height": "100%";
                "overflow": "hidden";
                "border-radius": "2px";
                "box-shadow": format!("0 0 0.1rem 0.1rem {}", color_system::gray(255, 9));
            }

            "rsz-top" {
                "position": "absolute";
                "top": "-1rem";
                "left": "0";
                "width": "100%";
                "height": "1rem";
            }

            "rsz-top:hover" {
                "cursor": "ns-resize";
            }

            "rsz-top-left" {
                "position": "absolute";
                "top": "-1rem";
                "left": "-1rem";
                "width": "1rem";
                "height": "1rem";
            }

            "rsz-top-left:hover" {
                "cursor": "nwse-resize";
            }

            "rsz-left" {
                "position": "absolute";
                "top": "0";
                "left": "-1rem";
                "width": "1rem";
                "height": "100%";
            }

            "rsz-left:hover" {
                "cursor": "ew-resize";
            }

            "rsz-bottom-left" {
                "position": "absolute";
                "top": "100%";
                "left": "-1rem";
                "width": "1rem";
                "height": "1rem";
            }

            "rsz-bottom-left:hover" {
                "cursor": "nesw-resize";
            }

            "rsz-bottom" {
                "position": "absolute";
                "top": "100%";
                "left": "0";
                "width": "100%";
                "height": "1rem";
            }

            "rsz-bottom:hover" {
                "cursor": "ns-resize";
            }

            "rsz-bottom-right" {
                "position": "absolute";
                "top": "100%";
                "left": "100%";
                "width": "1rem";
                "height": "1rem";
            }

            "rsz-bottom-right:hover" {
                "cursor": "nwse-resize";
            }

            "rsz-right" {
                "position": "absolute";
                "top": "0";
                "left": "100%";
                "width": "1rem";
                "height": "100%";
            }

            "rsz-right:hover" {
                "cursor": "ew-resize";
            }

            "rsz-top-right" {
                "position": "absolute";
                "top": "-1rem";
                "left": "100%";
                "width": "1rem";
                "height": "1rem";
            }

            "rsz-top-right:hover" {
                "cursor": "nesw-resize";
            }
        }
    }
}
