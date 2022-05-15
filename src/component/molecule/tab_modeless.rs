use super::atom::btn::{self, Btn};
use super::atom::dropdown::{self, Dropdown};
use super::atom::fa;
use super::atom::tab_btn::TabBtn;
use super::atom::text;
use crate::libs::color::color_system;
use crate::libs::random_id::U128Id;
use crate::libs::select_list::SelectList;
use crate::libs::type_id::type_id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use nusa::v_node::v_element::VEvent;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Props<
    Content: HtmlComponent + Unpin,
    TabName: HtmlComponent<Props = Content::Props> + Unpin,
> where
    Content::Props: Clone,
{
    pub state: Rc<RefCell<State<Content, TabName>>>,
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
    state: Rc<RefCell<State<Content, TabName>>>,
}

pub struct State<
    Content: HtmlComponent + Unpin,
    TabName: HtmlComponent<Props = Content::Props> + Unpin,
> where
    Content::Props: Clone,
{
    size: [f64; 2],
    loc: [f64; 2],
    dragging: Option<([i32; 2], DragType)>,
    some_is_dragging: bool,
    container_rect: Option<Rc<ContainerRect>>,
    modeless_id: U128Id,
    z_index: usize,
    contents: SelectList<Content::Props>,
    cmds: Vec<Cmd<TabModeless<Content, TabName>>>,
    _phantom_content: std::marker::PhantomData<Content>,
    _phantom_tab_name: std::marker::PhantomData<TabName>,
}

impl<Content: HtmlComponent + Unpin, TabName: HtmlComponent<Props = Content::Props> + Unpin>
    State<Content, TabName>
where
    Content::Props: Clone,
{
    pub fn new(
        size: [f64; 2],
        page_x: i32,
        page_y: i32,
        contents: SelectList<Content::Props>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            loc: [page_x as f64, page_y as f64],
            size: size,
            dragging: None,
            some_is_dragging: false,
            container_rect: None,
            modeless_id: U128Id::none(),
            contents: contents,
            z_index: 0,
            cmds: vec![],
            _phantom_content: std::marker::PhantomData,
            _phantom_tab_name: std::marker::PhantomData,
        }))
    }

    pub fn set(
        &mut self,
        container_rect: Rc<ContainerRect>,
        modeless_id: U128Id,
        some_dragging: Option<(i32, i32)>,
        z_index: usize,
    ) {
        if let Some(self_container_rect) = self.container_rect.as_ref() {
            if container_rect.width != self_container_rect.width {
                let margin_ratio = self.loc[0] / (self_container_rect.width - self.size[0]);
                let margin = container_rect.width - self.size[0];
                self.loc[0] = margin_ratio * margin;
            }

            if container_rect.height != self_container_rect.height {
                let margin_ratio = self.loc[1] / (self_container_rect.height - self.size[1]);
                let margin = container_rect.height - self.size[1];
                self.loc[1] = margin_ratio * margin;
            }
        } else {
            let client_x = self.loc[0] as f64 - container_rect.left;
            let client_y = self.loc[1] as f64 - container_rect.top;
            let loc = [client_x.max(0.0), client_y.max(0.0)];
            self.loc = loc;
        }

        self.container_rect = Some(container_rect);
        self.modeless_id = modeless_id;
        self.some_is_dragging = some_dragging.is_some();
        self.z_index = z_index;

        if self.dragging.is_some() {
            if let Some((page_x, page_y)) = some_dragging {
                self.cmds.push(Cmd::chain(Msg::Drag { page_x, page_y }));
            } else {
                self.cmds.push(Cmd::chain(Msg::DragEnd));
            }
        }
    }

    pub fn contents(&self) -> &SelectList<Content::Props> {
        &self.contents
    }

    pub fn contents_mut(&mut self) -> &mut SelectList<Content::Props> {
        &mut self.contents
    }
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
    type Props = Props<Content, TabName>;
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
    fn constructor(props: Self::Props) -> Self {
        Self { state: props.state }
    }
}

impl<Content: HtmlComponent + Unpin, TabName: HtmlComponent<Props = Content::Props> + Unpin> Update
    for TabModeless<Content, TabName>
where
    Content::Props: Clone,
{
    fn on_assemble(self: Pin<&mut Self>) -> Cmd<Self> {
        Cmd::list(self.state.borrow_mut().cmds.drain(..).collect())
    }

    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.state = props.state;
        Cmd::list(self.state.borrow_mut().cmds.drain(..).collect())
    }

    fn update(self: Pin<&mut Self>, msg: Msg<Content::Event>) -> Cmd<Self> {
        let mut this = self.state.borrow_mut();
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::submit(sub),
            Msg::CloseSelf => Cmd::submit(On::Close(U128Id::clone(&this.modeless_id))),
            Msg::SetMinimizedSelf(is_minimized) => Cmd::submit(On::SetMinimized(
                U128Id::clone(&this.modeless_id),
                is_minimized,
            )),
            Msg::CloneTab(tab_idx) => {
                if let Some(tab) = this.contents.get(tab_idx) {
                    let tab = tab.clone();
                    this.contents.insert(tab_idx + 1, tab);
                }
                Cmd::none()
            }
            Msg::CloseTab(tab_idx) => {
                this.contents.remove(tab_idx);
                if this.contents.len() == 0 {
                    Cmd::submit(On::Close(U128Id::clone(&this.modeless_id)))
                } else {
                    Cmd::none()
                }
            }
            Msg::DragStart {
                page_x,
                page_y,
                drag_type,
            } => {
                this.dragging = Some(([page_x, page_y], drag_type));
                Cmd::list(vec![
                    Cmd::submit(On::Focus(U128Id::clone(&this.modeless_id))),
                    Cmd::submit(On::ChangeDraggingState(
                        U128Id::clone(&this.modeless_id),
                        Some((page_x, page_y)),
                    )),
                ])
            }
            Msg::DragEnd => {
                this.dragging = None;
                Cmd::list(vec![
                    Cmd::submit(On::Focus(U128Id::clone(&this.modeless_id))),
                    Cmd::submit(On::ChangeDraggingState(
                        U128Id::clone(&this.modeless_id),
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
                modeless_id: U128Id::clone(&this.modeless_id),
                is_selected,
            }),
            Msg::SetSelectedTabIdx(tab_idx) => {
                this.contents.set_selected_idx(tab_idx);
                Cmd::none()
            }
            Msg::Drag { page_x, page_y } => {
                let mut cmds = vec![];
                if let Some(mut dragging) = this.dragging {
                    let mov_x = (page_x - dragging.0[0]) as f64;
                    let mov_y = (page_y - dragging.0[1]) as f64;

                    match &dragging.1 {
                        DragType::Move => {
                            this.loc[0] += mov_x;
                            this.loc[1] += mov_y;
                        }
                        DragType::Resize(DragDirection::Top) => {
                            this.loc[1] += mov_y;
                            this.size[1] -= mov_y;
                        }
                        DragType::Resize(DragDirection::Left) => {
                            this.loc[0] += mov_x;
                            this.size[0] -= mov_x;
                        }
                        DragType::Resize(DragDirection::Bottom) => {
                            this.size[1] += mov_y;
                        }
                        DragType::Resize(DragDirection::Right) => {
                            this.size[0] += mov_x;
                        }
                        DragType::Resize(DragDirection::TopLeft) => {
                            this.loc[0] += mov_x;
                            this.loc[1] += mov_y;
                            this.size[0] -= mov_x;
                            this.size[1] -= mov_y;
                        }
                        DragType::Resize(DragDirection::TopRight) => {
                            this.loc[1] += mov_y;
                            this.size[0] += mov_x;
                            this.size[1] -= mov_y;
                        }
                        DragType::Resize(DragDirection::BottomLeft) => {
                            this.loc[0] += mov_x;
                            this.size[0] -= mov_x;
                            this.size[1] += mov_y;
                        }
                        DragType::Resize(DragDirection::BottomRight) => {
                            this.size[0] += mov_x;
                            this.size[1] += mov_y;
                        }
                    }

                    if this.loc[0] < 0.0 {
                        this.loc[0] = 0.0;
                    }
                    if this.loc[1] < 0.0 {
                        this.loc[1] = 0.0;
                    }

                    if let Some(this_container_rect) = this.container_rect.clone() {
                        if this.size[0] > this_container_rect.width {
                            this.size[0] = this_container_rect.width;
                        }
                        if this.size[1] > this_container_rect.height {
                            this.size[1] = this_container_rect.height;
                        }
                        if this.size[0] < this_container_rect.width * 0.1 {
                            this.size[0] = this_container_rect.width * 0.1;
                        }
                        if this.size[1] < this_container_rect.height * 0.1 {
                            this.size[1] = this_container_rect.height * 0.1;
                        }
                        if this.loc[0] + this.size[0] > this_container_rect.width {
                            this.loc[0] = this_container_rect.width - this.size[0];
                        }
                        if this.loc[1] + this.size[1] > this_container_rect.height {
                            this.loc[1] = this_container_rect.height - this.size[1];
                        }
                    }

                    dragging.0[0] = page_x;
                    dragging.0[1] = page_y;
                    this.dragging = Some(dragging);

                    cmds.push(match (&dragging.1, this.container_rect.as_ref()) {
                        (DragType::Move, Some(this_container_rect)) => {
                            let page_x = (this.loc[0] + this_container_rect.left).round() as i32;
                            let page_y = (this.loc[1] + this_container_rect.top).round() as i32;
                            Cmd::submit(On::Move(page_x, page_y))
                        }
                        _ => Cmd::submit(On::Resize(this.size.clone())),
                    })
                }

                if let Some(this_container_rect) = this.container_rect.as_ref() {
                    if page_x < this_container_rect.left as i32
                        || page_x > (this_container_rect.left + this_container_rect.width) as i32
                        || page_y < this_container_rect.top as i32
                        || page_y > (this_container_rect.top + this_container_rect.height) as i32
                    {
                        this.dragging = None;
                        cmds.push(Cmd::submit(On::ChangeDraggingState(
                            U128Id::clone(&this.modeless_id),
                            None,
                        )));
                    }
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
                .index_id(self.state.borrow().modeless_id.to_string())
                .string("data-modeless-id", self.state.borrow().modeless_id.to_string())
                .class(Self::class("base"))
                .style("z-index", format!("{}", self.state.borrow().z_index))
                .style("left", format!("{}px", self.state.borrow().loc[0].round() as i32))
                .style("top", format!("{}px", self.state.borrow().loc[1].round() as i32))
                .style("width", format!("{}px", self.state.borrow().size[0].round() as i32))
                .style("height", format!("{}px", self.state.borrow().size[1].round() as i32)),
            {
                let mut events = Events::new();

                if !self.state.borrow().some_is_dragging {
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
                }).on_click(self, {
                    let modeless_id = U128Id::clone(&self.state.borrow().modeless_id);
                    |_| Msg::Sub(On::Focus(modeless_id))
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
                            let modeless_id = U128Id::clone(&self.state.borrow().modeless_id);
                            move |e| {
                                let e = unwrap!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
                                Self::on_drop_tab(None, e,modeless_id)
                            }
                        }),
                    vec![
                        Html::div(Attributes::new().class(Self::class("header-tabs")),Events::new(),
                        self.state.borrow()
                            .contents
                            .iter()
                            .enumerate()
                            .map(|(tab_idx, content)| {
                                let is_selected = tab_idx == self.state.borrow().contents.selected_idx();
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
                                                .set_data("text/plain", &(type_id::<Self>() + ";" + &event_id.to_string()))
                                                .ok();
                                            Msg::NoOp
                                        );
                                        Msg::DisconnectTab {
                                            event_id,
                                            tab_idx,
                                            is_selected
                                        }
                                    }).on_drop(self, {
                                        let modeless_id = U128Id::clone(&self.state.borrow().modeless_id);
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
                                        let selected_tab_idx = self.state.borrow().contents.selected_idx();
                                        move |_| Msg::CloneTab(selected_tab_idx)
                                    }),
                                    vec![Html::text("現在のタブを複製")]
                                ),
                                Btn::menu_as_secondary(
                                    Attributes::new(),
                                    Events::new().on_click(self,{
                                        let selected_tab_idx = self.state.borrow().contents.selected_idx();
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
                    vec![self.state.borrow()
                        .contents
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

        let payload = data.split(";").collect::<Vec<_>>();
        if payload.len() < 2 {
            return Msg::NoOp;
        }

        if type_id::<Self>() != payload[0] {
            return Msg::NoOp;
        }

        if let Some(event_id) = U128Id::from_hex(&payload[1]) {
            e.prevent_default();
            e.stop_propagation();
            return Msg::Sub(On::ConnectTab {
                event_id,
                header_tab_idx: tab_idx,
                modeless_id,
            });
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
