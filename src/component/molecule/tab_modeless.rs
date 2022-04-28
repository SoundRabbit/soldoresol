use super::atom::btn::{self, Btn};
use super::atom::dropdown::{self, Dropdown};
use super::atom::fa;
use super::atom::tab_btn::TabBtn;
use super::atom::text;
use crate::libs::color::color_system;
use crate::libs::random_id::U128Id;
use crate::libs::select_list::SelectList;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use nusa::v_node::v_element::VEvent;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Props<T> {
    pub size: [f64; 2],
    pub page_x: i32,
    pub page_y: i32,
    pub z_index: usize,
    pub container_rect: Rc<ContainerRect>,
    pub contents: Rc<RefCell<SelectList<T>>>,
    pub modeless_id: U128Id,
    pub some_dragging: Option<(i32, i32)>,
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
    ChangeDraggingState(U128Id, Option<(i32, i32)>),
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

pub struct TabModeless<
    Content: HtmlComponent + Unpin,
    TabName: HtmlComponent<Props = Content::Props> + Unpin,
> where
    Content::Props: Clone,
{
    size: [f64; 2],
    loc: [f64; 2],
    dragging: Option<([i32; 2], DragType)>,
    some_is_dragging: bool,
    container_rect: Rc<ContainerRect>,
    modeless_id: U128Id,
    z_index: usize,
    contents: Rc<RefCell<SelectList<Content::Props>>>,
    _phantom_content: std::marker::PhantomData<Content>,
    _phantom_tab_name: std::marker::PhantomData<TabName>,
}

#[derive(Clone, Copy)]
pub enum DragType {
    Move,
    Resize(DragDirection),
}

#[derive(Clone, Copy)]
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

impl<Content: HtmlComponent + Unpin, TabName: HtmlComponent<Props = Content::Props> + Unpin>
    Component for TabModeless<Content, TabName>
where
    Content::Props: Clone,
{
    type Props = Props<Content::Props>;
    type Msg = Msg<Content::Event>;
    type Event = On<Content::Event>;
}

impl<Content: HtmlComponent + Unpin, TabName: HtmlComponent<Props = Content::Props> + Unpin>
    HtmlComponent for TabModeless<Content, TabName>
where
    Content::Props: Clone,
{
}

impl<Content: HtmlComponent + Unpin, TabName: HtmlComponent<Props = Content::Props> + Unpin>
    Constructor for TabModeless<Content, TabName>
where
    Content::Props: Clone,
{
    fn constructor(props: Props<Content::Props>) -> Self {
        let client_x = props.page_x as f64 - props.container_rect.left;
        let client_y = props.page_y as f64 - props.container_rect.top;
        let loc = [client_x.max(0.0), client_y.max(0.0)];

        Self {
            loc: loc,
            size: props.size.clone(),
            dragging: None,
            some_is_dragging: props.some_dragging.is_some(),
            container_rect: Rc::clone(&props.container_rect),
            modeless_id: props.modeless_id,
            contents: props.contents,
            z_index: props.z_index,
            _phantom_content: std::marker::PhantomData,
            _phantom_tab_name: std::marker::PhantomData,
        }
    }
}

impl<Content: HtmlComponent + Unpin, TabName: HtmlComponent<Props = Content::Props> + Unpin> Update
    for TabModeless<Content, TabName>
where
    Content::Props: Clone,
{
    fn on_load(mut self: Pin<&mut Self>, props: Props<Content::Props>) -> Cmd<Self> {
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

        self.container_rect = props.container_rect;
        self.modeless_id = props.modeless_id;
        self.some_is_dragging = props.some_dragging.is_some();
        self.contents = props.contents;
        self.z_index = props.z_index;

        let mut cmds = vec![];

        if self.dragging.is_some() {
            if let Some((page_x, page_y)) = props.some_dragging {
                cmds.push(Cmd::chain(Msg::Drag { page_x, page_y }));
            } else {
                cmds.push(Cmd::chain(Msg::DragEnd));
            }
        }

        Cmd::list(cmds)
    }

    fn update(mut self: Pin<&mut Self>, msg: Msg<Content::Event>) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::submit(sub),
            Msg::CloseSelf => Cmd::submit(On::Close(U128Id::clone(&self.modeless_id))),
            Msg::SetMinimizedSelf(is_minimized) => Cmd::submit(On::SetMinimized(
                U128Id::clone(&self.modeless_id),
                is_minimized,
            )),
            Msg::CloneTab(tab_idx) => {
                let mut contents = self.contents.borrow_mut();
                if let Some(tab) = contents.get(tab_idx) {
                    let tab = tab.clone();
                    contents.insert(tab_idx + 1, tab);
                }
                Cmd::none()
            }
            Msg::CloseTab(tab_idx) => {
                self.contents.borrow_mut().remove(tab_idx);
                if self.contents.borrow().len() == 0 {
                    Cmd::submit(On::Close(U128Id::clone(&self.modeless_id)))
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
                Cmd::list(vec![
                    Cmd::submit(On::Focus(U128Id::clone(&self.modeless_id))),
                    Cmd::submit(On::ChangeDraggingState(
                        U128Id::clone(&self.modeless_id),
                        Some((page_x, page_y)),
                    )),
                ])
            }
            Msg::DragEnd => {
                self.dragging = None;
                Cmd::list(vec![
                    Cmd::submit(On::Focus(U128Id::clone(&self.modeless_id))),
                    Cmd::submit(On::ChangeDraggingState(
                        U128Id::clone(&self.modeless_id),
                        None,
                    )),
                ])
            }
            Msg::DisconnectTab {
                event_id,
                tab_idx,
                is_selected,
            } => Cmd::submit(On::DisconnectTab {
                event_id,
                tab_idx,
                modeless_id: U128Id::clone(&self.modeless_id),
                is_selected,
            }),
            Msg::SetSelectedTabIdx(tab_idx) => {
                self.contents.borrow_mut().set_selected_idx(tab_idx);
                Cmd::none()
            }
            Msg::Drag { page_x, page_y } => {
                let mut cmds = vec![];
                if let Some(mut dragging) = self.dragging {
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
                    self.dragging = Some(dragging);

                    cmds.push(match &dragging.1 {
                        DragType::Move => {
                            let page_x = (self.loc[0] + self.container_rect.left).round() as i32;
                            let page_y = (self.loc[1] + self.container_rect.top).round() as i32;
                            Cmd::submit(On::Move(page_x, page_y))
                        }
                        _ => Cmd::submit(On::Resize(self.size.clone())),
                    })
                }

                if page_x < self.container_rect.left as i32
                    || page_x > (self.container_rect.left + self.container_rect.width) as i32
                    || page_y < self.container_rect.top as i32
                    || page_y > (self.container_rect.top + self.container_rect.height) as i32
                {
                    self.dragging = None;
                    cmds.push(Cmd::submit(On::ChangeDraggingState(
                        U128Id::clone(&self.modeless_id),
                        None,
                    )));
                }

                Cmd::list(cmds)
            }
        }
    }
}

impl<Content: HtmlComponent + Unpin, TabName: HtmlComponent<Props = Content::Props> + Unpin>
    Render<Html> for TabModeless<Content, TabName>
where
    Content::Props: Clone,
{
    type Children = ();
    fn render(&self, _: ()) -> Html {
        Self::styled(Html::div(
            Attributes::new()
                .index_id(self.modeless_id.to_string())
                .string("data-modeless-id", self.modeless_id.to_string())
                .class(Self::class("base"))
                .style("z-index", format!("{}", self.z_index))
                .style("left", format!("{}px", self.loc[0].round() as i32))
                .style("top", format!("{}px", self.loc[1].round() as i32))
                .style("width", format!("{}px", self.size[0].round() as i32))
                .style("height", format!("{}px", self.size[1].round() as i32)),
            {
                let mut events = Events::new();

                if !self.some_is_dragging {
                    events = events.on_mousedown(self, |e| {
                        let e = unwrap!(e.dyn_into::<web_sys::MouseEvent>().ok(); Msg::NoOp);
                        e.stop_propagation();
                        Msg::DragStart {
                            page_x: e.page_x(),
                            page_y: e.page_y(),
                            drag_type: DragType::Move,
                        }
                    });
                }

                events = events.on("wheel",self, |e| {
                    e.stop_propagation();
                    Msg::NoOp
                });

                events
            },
            vec![
                Html::div(
                    Attributes::new().class(Self::class("header")),
                    Events::new()
                        .on_dragover(self, |e| {
                            e.prevent_default();
                            Msg::NoOp
                        })
                        .on_drop(self, {
                            let modeless_id = U128Id::clone(&self.modeless_id);
                            move |e| {
                                let e = unwrap!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
                                Self::on_drop_tab(None, e,modeless_id)
                            }
                        }),
                    vec![
                        Html::div(Attributes::new().class(Self::class("header-tabs")),Events::new(),
                        self
                            .contents
                            .borrow()
                            .iter()
                            .enumerate()
                            .map(|(tab_idx, content)| {
                                let is_selected = tab_idx == self.contents.borrow().selected_idx();
                                TabBtn::new(
                                    true,
                                    is_selected,
                                    Attributes::new(),
                                    Events::new()
                                    .on_mousedown(self, |e|{ e.stop_propagation(); Msg::NoOp})
                                    .on_dragstart(self,
                                        move |e| {
                                        let e = unwrap!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
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
                                    }).on_drop(self, {
                                        let modeless_id = U128Id::clone(&self.modeless_id);
                                        move |e| {
                                            let e = unwrap!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
                                            Self::on_drop_tab(Some(tab_idx), e,modeless_id)
                                        }
                                    }).on_click(self, move |_| Msg::SetSelectedTabIdx(tab_idx)),
                                    vec![TabName::empty(self, None, Clone::clone(content), Sub::none())],
                                )
                            })
                            .collect()
                        ),
                        Dropdown::new(
                            self,
                            None,
                            dropdown::Props {
                                direction: dropdown::Direction::BottomLeft,
                                toggle_type: dropdown::ToggleType::Click,
                                variant: btn::Variant::TransparentDark,
                            },
                            Sub::none(),
                            (vec![fa::fas_i("fa-ellipsis-vertical")],vec![
                                Html::span(
                                    Attributes::new()
                                        .class(Dropdown::class("menu-heading"))
                                        .class(Btn::class_name(&btn::Variant::SecondaryLikeMenu)),
                                    Events::new(),
                                    vec![text::span("タブ")]
                                ),
                                Btn::menu_as_secondary(
                                    Attributes::new(),
                                    Events::new().on_click(self,{
                                        let selected_tab_idx = self.contents.borrow().selected_idx();
                                        move |_| Msg::CloneTab(selected_tab_idx)
                                    }),
                                    vec![Html::text("現在のタブを複製")]
                                ),
                                Btn::menu_as_secondary(
                                    Attributes::new(),
                                    Events::new().on_click(self,{
                                        let selected_tab_idx = self.contents.borrow().selected_idx();
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
                                    Events::new().on_click(self,|_| Msg::SetMinimizedSelf(true)),
                                    vec![Html::text("ウィンドウを最小化")]
                                ),
                                Btn::menu_as_secondary(
                                    Attributes::new(),
                                    Events::new().on_click(self,|_| Msg::CloseSelf),
                                    vec![Html::text("ウィンドウを閉じる")]
                                ),
                            ])
                        )
                    ]
                ),
                Html::div(
                    Attributes::new().class(Self::class("content")),
                    Events::new().on_mousedown(self,|e| {
                        e.stop_propagation();
                        Msg::NoOp
                    }),
                    vec![self
                        .contents
                        .borrow()
                        .selected()
                        .map(|content| Content::empty(self, None, Clone::clone(content), Sub::map(|sub| Msg::Sub(On::Sub(sub)))))
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

impl<Content: HtmlComponent + Unpin, TabName: HtmlComponent<Props = Content::Props> + Unpin>
    TabModeless<Content, TabName>
where
    Content::Props: Clone,
{
    fn render_rsz(&self, drag_direction: DragDirection) -> Html {
        Html::div(
            Attributes::new().class(Self::class(&format!("rsz-{}", &drag_direction))),
            Events::new()
                .on_mousedown(self, move |e| {
                    let e = unwrap!(e.dyn_into::<web_sys::MouseEvent>().ok(); Msg::NoOp);
                    e.stop_propagation();
                    Msg::DragStart {
                        page_x: e.page_x(),
                        page_y: e.page_y(),
                        drag_type: DragType::Resize(drag_direction),
                    }
                })
                .on_mouseup(self, |_| Msg::DragEnd),
            vec![],
        )
    }

    fn on_drop_tab(
        tab_idx: Option<usize>,
        e: VEvent<web_sys::DragEvent>,
        modeless_id: U128Id,
    ) -> Msg<Content::Event> {
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

impl<Content: HtmlComponent + Unpin, TabName: HtmlComponent<Props = Content::Props> + Unpin> Styled
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
