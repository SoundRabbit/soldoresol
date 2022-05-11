use super::atom::btn::{self, Btn};
use super::atom::tab_btn::TabBtn;
use super::molecule::tab_modeless::{self, TabModeless};
use crate::libs::modeless_list::ModelessList;
use crate::libs::random_id::U128Id;
use crate::libs::select_list::SelectList;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

pub struct Props<
    Content: HtmlComponent + Unpin,
    TabName: HtmlComponent<Props = Content::Props> + Unpin,
> where
    Content::Props: Clone,
{
    pub modelesses: Rc<RefCell<TabModelessList<Content, TabName>>>,
}

pub enum Msg<Sub> {
    NoOp,
    Sub(On<Sub>),
    RefBase(web_sys::Node),
    ResizeBase,
    Focus(U128Id),
    Close(U128Id),
    SetMinimized(U128Id, bool),
    SetSomeDragging(Option<(i32, i32)>),
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
    DropTab {
        event_id: U128Id,
        page_x: i32,
        page_y: i32,
    },
}

pub enum On<Sub> {
    Sub(Sub),
    StartDragTab,
    EndDragTab,
}

pub struct TabModelessContainer<
    Content: HtmlComponent + Unpin,
    TabName: HtmlComponent<Props = Content::Props> + Unpin,
> where
    Content::Props: Clone,
{
    modelesses: Rc<RefCell<TabModelessList<Content, TabName>>>,
    element_rect: Option<Rc<tab_modeless::ContainerRect>>,
    floating_tab: HashMap<U128Id, FloatingTab>,
    base_node: Option<web_sys::Element>,
    some_dragging: Option<(i32, i32)>,
    _phantom_tab_name: std::marker::PhantomData<TabName>,
}

pub struct TabModelessList<
    Content: HtmlComponent + Unpin,
    TabName: HtmlComponent<Props = Content::Props> + Unpin,
> where
    Content::Props: Clone,
{
    data: ModelessList<Modeless<Content, TabName>>,
}

struct FloatingTab {
    tab_idx: usize,
    modeless_id: U128Id,
    is_selected: bool,
}

struct Modeless<
    Content: HtmlComponent + Unpin,
    TabName: HtmlComponent<Props = Content::Props> + Unpin,
> where
    Content::Props: Clone,
{
    state: Rc<RefCell<tab_modeless::State<Content, TabName>>>,
    is_minimized: bool,
}

impl<Content: HtmlComponent + Unpin, TabName: HtmlComponent<Props = Content::Props> + Unpin>
    TabModelessList<Content, TabName>
where
    Content::Props: Clone,
{
    pub fn new() -> Self {
        Self {
            data: ModelessList::new(),
        }
    }

    pub fn open_modeless(&mut self, contents: Vec<Content::Props>) {
        let m_idx = self.data.len();
        self.data.push(Modeless {
            state: tab_modeless::State::new(
                [800.0, 600.0],
                200 + (m_idx % 10) as i32 * 20,
                200 + (m_idx % 10) as i32 * 20,
                SelectList::new(contents, 0),
            ),
            is_minimized: false,
        });
    }
}

impl<Content: HtmlComponent + Unpin, TabName: HtmlComponent<Props = Content::Props> + Unpin>
    TabModelessList<Content, TabName>
where
    Content::Props: Clone + PartialEq,
{
    pub fn focus_first(&mut self, content: &Content::Props) {
        let m_id = self
            .data
            .iter()
            .filter_map(|x| x)
            .find_map(|(m_id, _, m_contents)| {
                if m_contents
                    .state
                    .borrow()
                    .contents()
                    .iter()
                    .any(|m_c| content.eq(m_c))
                {
                    Some(m_id)
                } else {
                    None
                }
            });

        if let Some(m_id) = m_id {
            self.data.focus(&m_id);
        }
    }
}

impl<Content: HtmlComponent + Unpin, TabName: HtmlComponent<Props = Content::Props> + Unpin>
    Component for TabModelessContainer<Content, TabName>
where
    Content::Props: Clone,
{
    type Props = Props<Content, TabName>;
    type Msg = Msg<Content::Event>;
    type Event = On<Content::Event>;
}

impl<Content: HtmlComponent + Unpin, TabName: HtmlComponent<Props = Content::Props> + Unpin>
    HtmlComponent for TabModelessContainer<Content, TabName>
where
    Content::Props: Clone,
{
}

impl<Content: HtmlComponent + Unpin, TabName: HtmlComponent<Props = Content::Props> + Unpin>
    Constructor for TabModelessContainer<Content, TabName>
where
    Content::Props: Clone,
{
    fn constructor(props: Self::Props) -> Self {
        Self {
            modelesses: props.modelesses,
            element_rect: None,
            floating_tab: HashMap::new(),
            base_node: None,
            some_dragging: None,
            _phantom_tab_name: std::marker::PhantomData,
        }
    }
}

impl<Content: HtmlComponent + Unpin, TabName: HtmlComponent<Props = Content::Props> + Unpin> Update
    for TabModelessContainer<Content, TabName>
where
    Content::Props: Clone,
{
    fn on_assemble(self: Pin<&mut Self>) -> Cmd<Self> {
        Cmd::batch(kagura::util::Batch::new(|mut handle| {
            let a = Closure::wrap(
                Box::new(move || handle(Cmd::chain(Msg::ResizeBase))) as Box<dyn FnMut()>
            );
            let _ = web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("resize", a.as_ref().unchecked_ref());
            a.forget();
        }))
    }

    fn update(mut self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::submit(sub),
            Msg::RefBase(node) => {
                self.base_node = node.dyn_into::<web_sys::Element>().ok();
                Cmd::chain(Msg::ResizeBase)
            }
            Msg::ResizeBase => {
                if let Some(node) = &self.base_node {
                    let rect = node.get_bounding_client_rect();
                    let rect = tab_modeless::ContainerRect {
                        left: rect.left(),
                        top: rect.top(),
                        width: rect.width(),
                        height: rect.height(),
                    };

                    if self
                        .element_rect
                        .as_ref()
                        .map(|el_rect| *(el_rect.as_ref()) != rect)
                        .unwrap_or(true)
                    {
                        self.element_rect = Some(Rc::new(rect));
                    }
                }
                Cmd::none()
            }
            Msg::Focus(modeless_id) => {
                self.modelesses.borrow_mut().data.focus(&modeless_id);
                Cmd::none()
            }
            Msg::Close(modeless_id) => {
                self.modelesses.borrow_mut().data.remove(&modeless_id);
                Cmd::none()
            }
            Msg::SetMinimized(modeless_id, is_minimized) => {
                if let Some(modeless) = self.modelesses.borrow_mut().data.get_mut(&modeless_id) {
                    modeless.is_minimized = is_minimized;
                }
                Cmd::none()
            }
            Msg::SetSomeDragging(some_dragging) => {
                self.some_dragging = some_dragging;
                Cmd::none()
            }
            Msg::DisconnectTab {
                event_id,
                tab_idx,
                modeless_id,
                is_selected,
            } => {
                self.floating_tab.insert(
                    event_id,
                    FloatingTab {
                        tab_idx,
                        modeless_id,
                        is_selected,
                    },
                );
                Cmd::submit(On::StartDragTab)
            }
            Msg::ConnectTab {
                event_id,
                header_tab_idx,
                modeless_id,
            } => {
                if let Some(event) = self.floating_tab.remove(&event_id) {
                    let content = self
                        .modelesses
                        .borrow_mut()
                        .data
                        .get_mut(&event.modeless_id)
                        .and_then(|x| x.state.borrow_mut().contents_mut().remove(event.tab_idx));

                    {
                        let mut modelesses = self.modelesses.borrow_mut();
                        if let Some((content, modeless)) =
                            join_some!(content, modelesses.data.get_mut(&modeless_id))
                        {
                            let tab_idx = if let Some(tab_idx) = header_tab_idx {
                                modeless
                                    .state
                                    .borrow_mut()
                                    .contents_mut()
                                    .insert(tab_idx, content);
                                tab_idx
                            } else {
                                modeless.state.borrow_mut().contents_mut().push(content);
                                modeless.state.borrow().contents().len() - 1
                            };

                            if event.is_selected {
                                modeless
                                    .state
                                    .borrow_mut()
                                    .contents_mut()
                                    .set_selected_idx(tab_idx);
                            }
                        }
                    }

                    if self
                        .modelesses
                        .borrow()
                        .data
                        .get(&event.modeless_id)
                        .map(|x| x.state.borrow().contents().len() < 1)
                        .unwrap_or(false)
                    {
                        self.modelesses.borrow_mut().data.remove(&event.modeless_id);
                    }
                }

                if self.floating_tab.is_empty() {
                    Cmd::submit(On::EndDragTab)
                } else {
                    Cmd::none()
                }
            }
            Msg::DropTab {
                event_id,
                page_x,
                page_y,
            } => {
                if let Some(event) = self.floating_tab.remove(&event_id) {
                    let content = self
                        .modelesses
                        .borrow_mut()
                        .data
                        .get_mut(&event.modeless_id)
                        .and_then(|x| x.state.borrow_mut().contents_mut().remove(event.tab_idx));

                    if let Some(content) = content {
                        self.modelesses.borrow_mut().data.push(Modeless {
                            state: tab_modeless::State::new(
                                [800.0, 600.0],
                                page_x - 10,
                                page_y - 10,
                                SelectList::new(vec![content], 0),
                            ),
                            is_minimized: false,
                        });
                    }

                    if self
                        .modelesses
                        .borrow()
                        .data
                        .get(&event.modeless_id)
                        .map(|x| x.state.borrow().contents().len() < 1)
                        .unwrap_or(false)
                    {
                        self.modelesses.borrow_mut().data.remove(&event.modeless_id);
                    }
                }
                if self.floating_tab.is_empty() {
                    Cmd::submit(On::EndDragTab)
                } else {
                    Cmd::none()
                }
            }
        }
    }
}

impl<Content: HtmlComponent + Unpin, TabName: HtmlComponent<Props = Content::Props> + Unpin>
    Render<Html> for TabModelessContainer<Content, TabName>
where
    Content::Props: Clone,
{
    type Children = Vec<Html>;
    fn render(&self, children: Self::Children) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new()
                .refer(self, |node| Msg::RefBase(node))
                .on_dragover(self, |e| {
                    e.prevent_default();
                    Msg::NoOp
                })
                .on_drop(self, move |e| {
                    let e = unwrap!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
                    let data_transfer = unwrap!(e.data_transfer(); Msg::NoOp);
                    let data = unwrap!(data_transfer.get_data("text/plain").ok(); Msg::NoOp);
                    if TabBtn::validate_prefix::<TabModeless<Content, TabName>>(&data) {
                        let suffix = TabBtn::get_suffix(&data);
                        if let Some(event_id) = suffix.get(0).and_then(|x| U128Id::from_hex(x)) {
                            e.prevent_default();
                            e.stop_propagation();
                            return Msg::DropTab {
                                event_id,
                                page_x: e.page_x(),
                                page_y: e.page_y(),
                            };
                        }
                    }
                    Msg::NoOp
                })
                .capture_on_mousemove(self, {
                    let some_is_dragging = self.some_dragging.is_some();
                    move |e| {
                        if !some_is_dragging {
                            Msg::NoOp
                        } else {
                            let e = unwrap!(e.dyn_into::<web_sys::MouseEvent>().ok(); Msg::NoOp);
                            e.stop_propagation();
                            Msg::SetSomeDragging(Some((e.page_x(), e.page_y())))
                        }
                    }
                })
                .capture_on_mouseup(self, {
                    let some_is_dragging = self.some_dragging.is_some();
                    move |e| {
                        if !some_is_dragging {
                            Msg::NoOp
                        } else {
                            e.stop_propagation();
                            Msg::SetSomeDragging(None)
                        }
                    }
                }),
            vec![
                Html::div(Attributes::new(), Events::new(), children),
                Html::div(
                    Attributes::new().class(Self::class("minimized-list")),
                    Events::new(),
                    self.modelesses
                        .borrow()
                        .data
                        .iter()
                        .map(|m| match m {
                            Some((m_id, _, contents)) if contents.is_minimized => Html::div(
                                Attributes::new().class(Self::class("minimized-item")),
                                Events::new(),
                                vec![
                                    Html::div(
                                        Attributes::new().class(Self::class("minimized-item-tabs")),
                                        Events::new(),
                                        contents
                                            .state
                                            .borrow()
                                            .contents()
                                            .iter()
                                            .map(|content| {
                                                Html::span(
                                                    Attributes::new()
                                                        .class(Btn::class_name(
                                                            &btn::Variant::DarkLikeMenu,
                                                        ))
                                                        .class(Self::class(
                                                            "minimized-item-tabs-tab",
                                                        )),
                                                    Events::new(),
                                                    vec![TabName::empty(
                                                        self,
                                                        None,
                                                        Clone::clone(&content),
                                                        Sub::none(),
                                                    )],
                                                )
                                            })
                                            .collect(),
                                    ),
                                    Btn::secondary(
                                        Attributes::new().class(Self::class("minimized-item-open")),
                                        Events::new().on_click(self, {
                                            let m_id = U128Id::clone(&m_id);
                                            move |_| Msg::SetMinimized(m_id, false)
                                        }),
                                        vec![Html::text("開く")],
                                    ),
                                ],
                            ),
                            _ => Html::none(),
                        })
                        .collect(),
                ),
                if let Some(element_rect) = self.element_rect.as_ref() {
                    Html::fragment(
                        self.modelesses
                            .borrow()
                            .data
                            .iter()
                            .map(|m| match m {
                                Some((m_id, z_idx, modeless)) if !modeless.is_minimized => {
                                    modeless.state.borrow_mut().set(
                                        Rc::clone(&element_rect),
                                        U128Id::clone(&m_id),
                                        self.some_dragging.clone(),
                                        z_idx,
                                    );
                                    TabModeless::<Content, TabName>::empty(
                                        self,
                                        None,
                                        tab_modeless::Props {
                                            state: Rc::clone(&modeless.state),
                                        },
                                        Sub::map(|sub| match sub {
                                            tab_modeless::On::DisconnectTab {
                                                event_id,
                                                tab_idx,
                                                modeless_id,
                                                is_selected,
                                            } => Msg::DisconnectTab {
                                                event_id,
                                                tab_idx,
                                                modeless_id,
                                                is_selected,
                                            },
                                            tab_modeless::On::ConnectTab {
                                                event_id,
                                                header_tab_idx,
                                                modeless_id,
                                            } => Msg::ConnectTab {
                                                event_id,
                                                header_tab_idx,
                                                modeless_id,
                                            },
                                            tab_modeless::On::Focus(modeless_id) => {
                                                Msg::Focus(modeless_id)
                                            }
                                            tab_modeless::On::Close(modeless_id) => {
                                                Msg::Close(modeless_id)
                                            }
                                            tab_modeless::On::SetMinimized(
                                                modeless_id,
                                                is_minimized,
                                            ) => Msg::SetMinimized(modeless_id, is_minimized),
                                            tab_modeless::On::ChangeDraggingState(
                                                _,
                                                some_dragging,
                                            ) => Msg::SetSomeDragging(some_dragging),
                                            tab_modeless::On::Sub(sub) => Msg::Sub(On::Sub(sub)),
                                            _ => Msg::NoOp,
                                        }),
                                    )
                                }
                                _ => Html::none(),
                            })
                            .collect(),
                    )
                } else {
                    Html::none()
                },
            ],
        ))
    }
}

impl<Content: HtmlComponent + Unpin, TabName: HtmlComponent<Props = Content::Props> + Unpin> Styled
    for TabModelessContainer<Content, TabName>
where
    Content::Props: Clone,
{
    fn style() -> Style {
        style! {
            ".base" {
                "width": "100%";
                "height": "100%";
                "overflow": "hidden";
                "position": "relative";
            }

            ".minimized-list" {
                "width": "100%";
                "bottom": "0";
                "min-height": "max-content";
                "display": "flex";
                "position": "absolute";
            }

            ".minimized-item" {
                "border-radius": "2px 2px 0 0";
                "box-shadow": format!("0 0 0.1rem 0.1rem {}", crate::libs::color::color_system::gray(255, 5));
                "min-width": "5rem";
            }

            ".minimized-item-tabs" {
                "border-radius": "2px 2px 0 0";
                "display": "flex";
                "flex-direction": "column";
                "margin-bottom": ".35rem";
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".minimized-item-tabs-tab" {
                "width": "100%";
                "padding": ".5em 1em";
            }

            ".minimized-item-open" {
                "width": "100%";
            }
        }
    }
}
