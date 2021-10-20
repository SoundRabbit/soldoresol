use super::atom::tab_btn::{self, TabBtn};
use crate::libs::color::color_system;
use crate::libs::random_id::U128Id;
use crate::libs::select_list::SelectList;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

pub struct Props<T> {
    pub size: [f32; 2],
    pub page_x: i32,
    pub page_y: i32,
    pub z_index: usize,
    pub container_element: Rc<web_sys::Element>,
    pub contents: Rc<RefCell<SelectList<T>>>,
    pub modeless_id: U128Id,
}

pub enum Msg<T> {
    NoOp,
    Sub(On<T>),
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
    DisconnectTab {
        event_id: U128Id,
        tab_idx: usize,
    },
}

pub enum On<T> {
    Focus,
    Move(i32, i32),
    Resize([f32; 2]),
    DisconnectTab {
        event_id: U128Id,
        content: T,
        modeless_id: U128Id,
    },
    ConnectTab {
        event_id: U128Id,
        header_tab_idx: Option<usize>,
        modeless_id: U128Id,
    },
}

pub struct TabModeless<Content: Constructor, TabName: Constructor<Props = Content::Props>>
where
    Content::Props: Clone,
{
    size: [f32; 2],
    loc: [f32; 2],
    dragging: Option<([i32; 2], DragType)>,
    container_element: Rc<web_sys::Element>,
    _phantom_content: std::marker::PhantomData<Content>,
    _phantom_tab_name: std::marker::PhantomData<TabName>,
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

impl<Content: Constructor, TabName: Constructor<Props = Content::Props>> Component
    for TabModeless<Content, TabName>
where
    Content::Props: Clone,
{
    type Props = Props<Content::Props>;
    type Msg = Msg<Content::Props>;
    type Sub = On<Content::Props>;
}

impl<Content: Constructor, TabName: Constructor<Props = Content::Props>> Constructor
    for TabModeless<Content, TabName>
where
    Content::Props: Clone,
{
    fn constructor(props: &Props<Content::Props>) -> Self {
        let el = &props.container_element;
        let rect = el.get_bounding_client_rect();
        let client_x = props.page_x as f64 - rect.left();
        let client_y = props.page_y as f64 - rect.top();
        let loc_x = client_x / rect.width();
        let loc_y = client_y / rect.height();
        let loc = [(loc_x as f32).max(0.0), (loc_y as f32).max(0.0)];

        Self {
            loc: loc,
            size: props.size.clone(),
            dragging: None,
            container_element: Rc::clone(&props.container_element),
            _phantom_content: std::marker::PhantomData,
            _phantom_tab_name: std::marker::PhantomData,
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

impl<Content: Constructor, TabName: Constructor<Props = Content::Props>> Update
    for TabModeless<Content, TabName>
where
    Content::Props: Clone,
{
    fn update(&mut self, props: &Props<Content::Props>, msg: Msg<Content::Props>) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::sub(sub),
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
                Cmd::sub(On::Focus)
            }
            Msg::DisconnectTab { event_id, tab_idx } => {
                if let Some(content) = props.contents.borrow_mut().remove(tab_idx) {
                    Cmd::sub(On::DisconnectTab {
                        event_id,
                        content,
                        modeless_id: U128Id::clone(&props.modeless_id),
                    })
                } else {
                    Cmd::none()
                }
            }
            Msg::Drag { page_x, page_y } => {
                let cmd;
                if let Some(dragging) = self.dragging.as_mut() {
                    let mov_x = (page_x - dragging.0[0]) as f32;
                    let mov_y = (page_y - dragging.0[1]) as f32;
                    let container_element = self.container_element.get_bounding_client_rect();
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

                let rect = self.container_element.get_bounding_client_rect();
                if page_x < rect.left() as i32
                    || page_x > rect.right() as i32
                    || page_y < rect.top() as i32
                    || page_y > rect.bottom() as i32
                {
                    self.dragging = None;
                }

                cmd
            }
        }
    }
}

impl<Content: Constructor, TabName: Constructor<Props = Content::Props>> Render
    for TabModeless<Content, TabName>
where
    Content::Props: Clone,
{
    fn render(&self, props: &Props<Content::Props>, _: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .style("z-index", format!("{}", props.z_index))
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
                .on_mouseup(|e| {
                    e.stop_propagation();
                    Msg::DragEnd
                })
                .on_mousemove(on_drag!(self.dragging.is_some()))
                .on_mouseleave(on_drag!(self.dragging.is_some()))
                .on("wheel", |e| {
                    e.stop_propagation();
                    Msg::NoOp
                }),
            vec![
                Html::div(
                    Attributes::new().class(Self::class("header")),
                    Events::new()
                        .on_dragover(|e| {
                            e.prevent_default();
                            Msg::NoOp
                        })
                        .on_drop({let modeless_id = U128Id::clone(&props.modeless_id);move |e| {
                            Self::on_drop_tab(None, e,modeless_id)
                        }}),
                    props
                        .contents
                        .borrow()
                        .iter()
                        .enumerate()
                        .map(|(tab_idx, content)| {
                            TabBtn::new(
                                true,
                                tab_idx == props.contents.borrow().selected_idx(),
                                Attributes::new(),
                                Events::new()
                                .on_mousedown(|e|{ e.stop_propagation(); Msg::NoOp})
                                .on_dragstart(
                                    move |e| {
                                    e.stop_propagation();
                                    let data_transfer = unwrap_or!(e.data_transfer(); Msg::NoOp);
                                    let event_id = U128Id::new();
                                    unwrap_or!(
                                        data_transfer
                                            .set_data("text/plain", &TabBtn::id::<Self>(vec![&event_id.to_string()]))
                                            .ok();
                                        Msg::NoOp
                                    );
                                    Msg::DisconnectTab {
                                        event_id,
                                        tab_idx,
                                    }
                                }).on_drop({
                                    let modeless_id = U128Id::clone(&props.modeless_id);
                                    move |e| {
                                        Self::on_drop_tab(Some(tab_idx), e,modeless_id)
                                    }
                                }),
                                vec![TabName::empty(Clone::clone(content), Sub::none())],
                            )
                        })
                        .collect(),
                ),
                Html::div(
                    Attributes::new().class(Self::class("content")),
                    Events::new(),
                    vec![props
                        .contents
                        .borrow()
                        .selected()
                        .map(|content| Content::empty(Clone::clone(content), Sub::none()))
                        .unwrap_or(Html::none())],
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

impl<Content: Constructor, TabName: Constructor<Props = Content::Props>>
    TabModeless<Content, TabName>
where
    Content::Props: Clone,
{
    fn render_rsz(&self, drag_direction: DragDirection) -> Html<Self> {
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

    fn on_drop_tab(
        tab_idx: Option<usize>,
        e: web_sys::DragEvent,
        modeless_id: U128Id,
    ) -> Msg<Content::Props> {
        let e = unwrap_or!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
        let data_transfer = unwrap_or!(e.data_transfer(); Msg::NoOp);
        let data = unwrap_or!(data_transfer.get_data("text/plain").ok(); Msg::NoOp);
        if TabBtn::validate_prefix::<Self>(&data) {
            let suffix = TabBtn::get_suffix(&data);
            if let Some(event_id) = suffix.get(0).and_then(|x| U128Id::from_hex(x)) {
                e.prevent_default();
                e.stop_propagation();
                return Msg::Sub(On::ConnectTab {
                    event_id,
                    header_tab_idx: tab_idx,
                    modeless_id,
                });
            }
        }
        Msg::NoOp
    }
}

impl<Content: Constructor, TabName: Constructor<Props = Content::Props>> Styled
    for TabModeless<Content, TabName>
where
    Content::Props: Clone,
{
    fn style() -> Style {
        style! {
            ".base" {
                "position": "absolute";
                "overflow": "visible";
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-template-rows": "max-content 1fr";
            }

            ".content" {
                "overflow": "hidden";
                "border-radius": "2px";
                "box-shadow": format!("0 0 0.1rem 0.1rem {}", color_system::gray(255, 9));
            }

            ".rsz-top" {
                "position": "absolute";
                "top": "-0.9rem";
                "left": "0.1rem";
                "width": "calc(100% - 0.2rem)";
                "height": "1rem";
            }

            ".rsz-top:hover" {
                "cursor": "ns-resize";
            }

            ".rsz-top-left" {
                "position": "absolute";
                "top": "-0.9rem";
                "left": "-0.9rem";
                "width": "1rem";
                "height": "1rem";
            }

            ".rsz-top-left:hover" {
                "cursor": "nwse-resize";
            }

            ".rsz-left" {
                "position": "absolute";
                "top": "0.1rem";
                "left": "-0.9rem";
                "width": "1rem";
                "height": "calc(100% - 0.2rem)";
            }

            ".rsz-left:hover" {
                "cursor": "ew-resize";
            }

            ".rsz-bottom-left" {
                "position": "absolute";
                "top": "calc(100% - 0.2rem)";
                "left": "-0.9rem";
                "width": "1rem";
                "height": "1rem";
            }

            ".rsz-bottom-left:hover" {
                "cursor": "nesw-resize";
            }

            ".rsz-bottom" {
                "position": "absolute";
                "top": "calc(100% - 0.2rem)";
                "left": "0.1rem";
                "width": "calc(100% - 0.2rem)";
                "height": "1rem";
            }

            ".rsz-bottom:hover" {
                "cursor": "ns-resize";
            }

            ".rsz-bottom-right" {
                "position": "absolute";
                "top": "calc(100% - 0.2rem)";
                "left": "calc(100% - 0.2rem)";
                "width": "1rem";
                "height": "1rem";
            }

            ".rsz-bottom-right:hover" {
                "cursor": "nwse-resize";
            }

            ".rsz-right" {
                "position": "absolute";
                "top": "0.1rem";
                "left": "calc(100% - 0.2rem)";
                "width": "1rem";
                "height": "calc(100% - 0.2rem)";
            }

            ".rsz-right:hover" {
                "cursor": "ew-resize";
            }

            ".rsz-top-right" {
                "position": "absolute";
                "top": "-0.9rem";
                "left": "calc(100% - 0.2rem)";
                "width": "1rem";
                "height": "1rem";
            }

            ".rsz-top-right:hover" {
                "cursor": "nesw-resize";
            }
        }
    }
}
