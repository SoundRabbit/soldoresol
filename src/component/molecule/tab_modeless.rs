use super::atom::btn::{self, Btn};
use super::atom::dropdown::{self, Dropdown};
use super::atom::tab_btn::{self, TabBtn};
use super::atom::text;
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
    pub size: [f64; 2],
    pub page_x: i32,
    pub page_y: i32,
    pub z_index: usize,
    pub container_rect: Rc<ContainerRect>,
    pub contents: Rc<RefCell<SelectList<T>>>,
    pub modeless_id: U128Id,
}

#[derive(PartialEq)]
pub struct ContainerRect {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

pub enum Msg<Sub> {
    NoOp,
    Sub(On<Sub>),
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
        is_selected: bool,
    },
    SetSelectedTabIdx(usize),
    CloseSelf,
    SetMinimizedSelf(bool),
    CloneTab(usize),
    CloseTab(usize),
}

pub enum On<Sub> {
    Focus(U128Id),
    Close(U128Id),
    SetMinimized(U128Id, bool),
    Move(i32, i32),
    Resize([f64; 2]),
    DisconnectTab {
        event_id: U128Id,
        tab_idx: usize,
        modeless_id: U128Id,
        is_selected: bool,
    },
    ConnectTab {
        event_id: U128Id,
        header_tab_idx: Option<usize>,
        modeless_id: U128Id,
    },
    Sub(Sub),
}

pub struct TabModeless<Content: Constructor, TabName: Constructor<Props = Content::Props>>
where
    Content::Props: Clone,
{
    size: [f64; 2],
    loc: [f64; 2],
    dragging: Option<([i32; 2], DragType)>,
    container_rect: Rc<ContainerRect>,
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
    type Msg = Msg<Content::Sub>;
    type Sub = On<Content::Sub>;
}

impl<Content: Constructor, TabName: Constructor<Props = Content::Props>> Constructor
    for TabModeless<Content, TabName>
where
    Content::Props: Clone,
{
    fn constructor(props: &Props<Content::Props>) -> Self {
        let client_x = props.page_x as f64 - props.container_rect.left;
        let client_y = props.page_y as f64 - props.container_rect.top;
        let loc = [client_x.max(0.0), client_y.max(0.0)];

        Self {
            loc: loc,
            size: props.size.clone(),
            dragging: None,
            container_rect: Rc::clone(&props.container_rect),
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
    fn on_load(&mut self, props: &Props<Content::Props>) -> Cmd<Self> {
        if props.container_rect.width != self.container_rect.width {
            let margin_ratio = self.loc[0] / (self.container_rect.width - self.size[0]);
            let margin = props.container_rect.width - self.size[0];
            self.loc[0] = margin_ratio * margin;
        }

        if props.container_rect.height != self.container_rect.height {
            let margin_ratio = self.loc[1] / (self.container_rect.height - self.size[1]);
            let margin = props.container_rect.height - self.size[1];
            self.loc[1] = margin_ratio * margin;
        }

        self.container_rect = Rc::clone(&props.container_rect);

        Cmd::none()
    }

    fn update(&mut self, props: &Props<Content::Props>, msg: Msg<Content::Sub>) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::sub(sub),
            Msg::CloseSelf => Cmd::sub(On::Close(U128Id::clone(&props.modeless_id))),
            Msg::SetMinimizedSelf(is_minimized) => Cmd::sub(On::SetMinimized(
                U128Id::clone(&props.modeless_id),
                is_minimized,
            )),
            Msg::CloneTab(tab_idx) => {
                let mut contents = props.contents.borrow_mut();
                if let Some(tab) = contents.get(tab_idx) {
                    let tab = tab.clone();
                    contents.insert(tab_idx + 1, tab);
                }
                Cmd::none()
            }
            Msg::CloseTab(tab_idx) => {
                props.contents.borrow_mut().remove(tab_idx);
                if props.contents.borrow().len() == 0 {
                    Cmd::sub(On::Close(U128Id::clone(&props.modeless_id)))
                } else {
                    Cmd::none()
                }
            }
            Msg::DragStart {
                page_x,
                page_y,
                drag_type,
            } => {
                self.dragging = Some(([page_x, page_y], drag_type));
                Cmd::sub(On::Focus(U128Id::clone(&props.modeless_id)))
            }
            Msg::DragEnd => {
                self.dragging = None;
                Cmd::sub(On::Focus(U128Id::clone(&props.modeless_id)))
            }
            Msg::DisconnectTab {
                event_id,
                tab_idx,
                is_selected,
            } => Cmd::sub(On::DisconnectTab {
                event_id,
                tab_idx,
                modeless_id: U128Id::clone(&props.modeless_id),
                is_selected,
            }),
            Msg::SetSelectedTabIdx(tab_idx) => {
                props.contents.borrow_mut().set_selected_idx(tab_idx);
                Cmd::none()
            }
            Msg::Drag { page_x, page_y } => {
                let cmd;
                if let Some(dragging) = self.dragging.as_mut() {
                    let mov_x = (page_x - dragging.0[0]) as f64;
                    let mov_y = (page_y - dragging.0[1]) as f64;

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
                    if self.size[0] > self.container_rect.width {
                        self.size[0] = self.container_rect.width;
                    }
                    if self.size[1] > self.container_rect.height {
                        self.size[1] = self.container_rect.height;
                    }
                    if self.size[0] < self.container_rect.width * 0.1 {
                        self.size[0] = self.container_rect.width * 0.1;
                    }
                    if self.size[1] < self.container_rect.height * 0.1 {
                        self.size[1] = self.container_rect.height * 0.1;
                    }
                    if self.loc[0] + self.size[0] > self.container_rect.width {
                        self.loc[0] = self.container_rect.width - self.size[0];
                    }
                    if self.loc[1] + self.size[1] > self.container_rect.height {
                        self.loc[1] = self.container_rect.height - self.size[1];
                    }

                    dragging.0[0] = page_x;
                    dragging.0[1] = page_y;

                    cmd = match &dragging.1 {
                        DragType::Move => {
                            let page_x = (self.loc[0] + self.container_rect.left).round() as i32;
                            let page_y = (self.loc[1] + self.container_rect.top).round() as i32;
                            Cmd::sub(On::Move(page_x, page_y))
                        }
                        _ => Cmd::sub(On::Resize(self.size.clone())),
                    }
                } else {
                    cmd = Cmd::none();
                }

                if page_x < self.container_rect.left as i32
                    || page_x > (self.container_rect.left + self.container_rect.width) as i32
                    || page_y < self.container_rect.top as i32
                    || page_y > (self.container_rect.top + self.container_rect.height) as i32
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
                .style("left", format!("{}px", self.loc[0].round() as i32))
                .style("top", format!("{}px", self.loc[1].round() as i32))
                .style("width", format!("{}px", self.size[0].round() as i32))
                .style("height", format!("{}px", self.size[1].round() as i32)),
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
                    vec![
                        Html::div(Attributes::new().class(Self::class("header-tabs")),Events::new(),
                        props
                            .contents
                            .borrow()
                            .iter()
                            .enumerate()
                            .map(|(tab_idx, content)| {
                                let is_selected = tab_idx == props.contents.borrow().selected_idx();
                                TabBtn::new(
                                    true,
                                    is_selected,
                                    Attributes::new(),
                                    Events::new()
                                    .on_mousedown(|e|{ e.stop_propagation(); Msg::NoOp})
                                    .on_dragstart(
                                        move |e| {
                                        e.stop_propagation();
                                        let data_transfer = unwrap!(e.data_transfer(); Msg::NoOp);
                                        let event_id = U128Id::new();
                                        unwrap!(
                                            data_transfer
                                                .set_data("text/plain", &TabBtn::id::<Self>(vec![&event_id.to_string()]))
                                                .ok();
                                            Msg::NoOp
                                        );
                                        Msg::DisconnectTab {
                                            event_id,
                                            tab_idx,
                                            is_selected
                                        }
                                    }).on_drop({
                                        let modeless_id = U128Id::clone(&props.modeless_id);
                                        move |e| {
                                            Self::on_drop_tab(Some(tab_idx), e,modeless_id)
                                        }
                                    }).on_click(move |_| Msg::SetSelectedTabIdx(tab_idx)),
                                    vec![TabName::empty(Clone::clone(content), Sub::none())],
                                )
                            })
                            .collect()
                        ),
                        Dropdown::with_children(
                            dropdown::Props {
                                direction: dropdown::Direction::BottomLeft,
                                text: dropdown::Text::Menu,
                                toggle_type: dropdown::ToggleType::Click,
                                variant: btn::Variant::TransparentDark,
                            },
                            Sub::none(),
                            vec![
                                Html::span(
                                    Attributes::new()
                                        .class(Dropdown::class("menu-heading"))
                                        .class(Btn::class_name(&btn::Variant::SecondaryLikeMenu)),
                                    Events::new(),
                                    vec![text::span("タブ")]
                                ),
                                Btn::menu_as_secondary(
                                    Attributes::new(),
                                    Events::new().on_click({
                                        let selected_tab_idx = props.contents.borrow().selected_idx();
                                        move |_| Msg::CloneTab(selected_tab_idx)
                                    }),
                                    vec![Html::text("現在のタブを複製")]
                                ),
                                Btn::menu_as_secondary(
                                    Attributes::new(),
                                    Events::new().on_click({
                                        let selected_tab_idx = props.contents.borrow().selected_idx();
                                        move |_| Msg::CloseTab(selected_tab_idx)
                                    }),
                                    vec![Html::text("現在のタブを閉じる")]
                                ),
                                Html::span(
                                    Attributes::new()
                                        .class(Dropdown::class("menu-heading"))
                                        .class(Btn::class_name(&btn::Variant::SecondaryLikeMenu)),
                                    Events::new(),
                                    vec![text::span("ウィンドウ")]
                                ),
                                Btn::menu_as_secondary(
                                    Attributes::new(),
                                    Events::new().on_click(|_| Msg::SetMinimizedSelf(true)),
                                    vec![Html::text("ウィンドウを最小化")]
                                ),
                                Btn::menu_as_secondary(
                                    Attributes::new(),
                                    Events::new().on_click(|_| Msg::CloseSelf),
                                    vec![Html::text("ウィンドウを閉じる")]
                                ),
                            ]
                        )
                    ]
                ),
                Html::div(
                    Attributes::new().class(Self::class("content")),
                    Events::new().on_mousedown(|e| {
                        e.stop_propagation();
                        Msg::NoOp
                    }),
                    vec![props
                        .contents
                        .borrow()
                        .selected()
                        .map(|content| Content::empty(Clone::clone(content), Sub::map(|sub| Msg::Sub(On::Sub(sub)))))
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
    ) -> Msg<Content::Sub> {
        let e = unwrap!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
        let data_transfer = unwrap!(e.data_transfer(); Msg::NoOp);
        let data = unwrap!(data_transfer.get_data("text/plain").ok(); Msg::NoOp);
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
                "border-radius": "2px";
                "box-shadow": format!("0 0 0.1rem 0.1rem {}", color_system::gray(255, 9));
                "background-color": format!("{}", crate::libs::color::Pallet::gray(0));
            }
            ".header" {
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "background-color": crate::libs::color::Pallet::gray(7);
            }

            ".header-tabs" {
                "display": "flex";
                "felx-wrap": "wrap";
            }

            ".content" {
                "overflow": "hidden";
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
