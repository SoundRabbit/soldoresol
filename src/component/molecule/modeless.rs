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
    pub page_x: i32,
    pub page_y: i32,
    pub z_index: usize,
    pub container_element: Option<Prop<web_sys::Element>>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            size: [0.3, 0.3],
            z_index: 0,
            container_element: None,
            page_x: 0,
            page_y: 0,
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
    Move(i32, i32),
    Resize([f32; 2]),
}

pub struct Modeless {
    size: [f32; 2],
    loc: [f32; 2],
    z_index: usize,
    dragging: Option<([i32; 2], DragType)>,
    container_element: Option<Prop<web_sys::Element>>,
}

pub enum DragType {
    Move,
    Resize(DragDirection),
}

pub enum DragDirection {
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
            Self::TopRight => write!(f, "top-right"),
            Self::BottomLeft => write!(f, "bottom-left"),
            Self::BottomRight => write!(f, "bottom-right"),
        }
    }
}

impl Constructor for Modeless {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        let loc = if let Some(el) = props.container_element.as_ref() {
            let rect = el.get_bounding_client_rect();
            let client_x = props.page_x as f64 - rect.left();
            let client_y = props.page_y as f64 - rect.top();
            let loc_x = client_x / rect.width();
            let loc_y = client_y / rect.height();
            [(loc_x as f32).max(0.0), (loc_y as f32).max(0.0)]
        } else {
            [0.0, 0.0]
        };

        Self {
            z_index: props.z_index,
            loc: loc,
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
                let cmd;
                if let Some((dragging, container_element)) =
                    join_some!(self.dragging.as_mut(), self.container_element.as_ref())
                {
                    let mov_x = (page_x - dragging.0[0]) as f32;
                    let mov_y = (page_y - dragging.0[1]) as f32;
                    let container_element = container_element.get_bounding_client_rect();
                    let container_width = container_element.width() as f32;
                    let container_height = container_element.height() as f32;

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
                    if self.size[0] > 1.0 {
                        self.size[0] = 1.0;
                    }
                    if self.size[1] > 1.0 {
                        self.size[1] = 1.0;
                    }
                    if self.size[0] < 0.1 {
                        self.size[0] = 0.1;
                    }
                    if self.size[1] < 0.1 {
                        self.size[1] = 0.1;
                    }
                    if self.loc[0] + self.size[0] > 1.0 {
                        self.loc[0] = 1.0 - self.size[0];
                    }
                    if self.loc[1] + self.size[1] > 1.0 {
                        self.loc[1] = 1.0 - self.size[1];
                    }

                    dragging.0[0] = page_x;
                    dragging.0[1] = page_y;

                    cmd = match &dragging.1 {
                        DragType::Move => {
                            let client_x = (self.loc[0] * container_width).round() as i32;
                            let client_y = (self.loc[1] * container_height).round() as i32;
                            let page_x = client_x + container_element.left() as i32;
                            let page_y = client_y + container_element.top() as i32;
                            Cmd::sub(On::Move(page_x, page_y))
                        }
                        _ => Cmd::sub(On::Resize(self.size.clone())),
                    }
                } else {
                    cmd = Cmd::none();
                }

                if let Some(container_element) = self.container_element.as_ref() {
                    let rect = container_element.get_bounding_client_rect();
                    if page_x < rect.left() as i32
                        || page_x > rect.right() as i32
                        || page_y < rect.top() as i32
                        || page_y > rect.bottom() as i32
                    {
                        self.dragging = None;
                    }
                }

                cmd
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
            Events::new()
                .on_mousedown(|e| {
                    e.stop_propagation();
                    Msg::DragStart {
                        page_x: e.page_x(),
                        page_y: e.page_y(),
                        drag_type: DragType::Resize(drag_direction),
                    }
                })
                .on_mouseup(|_| Msg::DragEnd)
                .on_mousemove(on_drag!(self.dragging.is_some()))
                .on_mouseleave(on_drag!(self.dragging.is_some())),
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
                "top": "-0.9rem";
                "left": "0.1rem";
                "width": "calc(100% - 0.2rem)";
                "height": "1rem";
            }

            "rsz-top:hover" {
                "cursor": "ns-resize";
            }

            "rsz-top-left" {
                "position": "absolute";
                "top": "-0.9rem";
                "left": "-0.9rem";
                "width": "1rem";
                "height": "1rem";
            }

            "rsz-top-left:hover" {
                "cursor": "nwse-resize";
            }

            "rsz-left" {
                "position": "absolute";
                "top": "0.1rem";
                "left": "-0.9rem";
                "width": "1rem";
                "height": "calc(100% - 0.2rem)";
            }

            "rsz-left:hover" {
                "cursor": "ew-resize";
            }

            "rsz-bottom-left" {
                "position": "absolute";
                "top": "calc(100% - 0.2rem)";
                "left": "-0.9rem";
                "width": "1rem";
                "height": "1rem";
            }

            "rsz-bottom-left:hover" {
                "cursor": "nesw-resize";
            }

            "rsz-bottom" {
                "position": "absolute";
                "top": "calc(100% - 0.2rem)";
                "left": "0.1rem";
                "width": "calc(100% - 0.2rem)";
                "height": "1rem";
            }

            "rsz-bottom:hover" {
                "cursor": "ns-resize";
            }

            "rsz-bottom-right" {
                "position": "absolute";
                "top": "calc(100% - 0.2rem)";
                "left": "calc(100% - 0.2rem)";
                "width": "1rem";
                "height": "1rem";
            }

            "rsz-bottom-right:hover" {
                "cursor": "nwse-resize";
            }

            "rsz-right" {
                "position": "absolute";
                "top": "0.1rem";
                "left": "calc(100% - 0.2rem)";
                "width": "1rem";
                "height": "calc(100% - 0.2rem)";
            }

            "rsz-right:hover" {
                "cursor": "ew-resize";
            }

            "rsz-top-right" {
                "position": "absolute";
                "top": "-0.9rem";
                "left": "calc(100% - 0.2rem)";
                "width": "1rem";
                "height": "1rem";
            }

            "rsz-top-right:hover" {
                "cursor": "nesw-resize";
            }
        }
    }
}
