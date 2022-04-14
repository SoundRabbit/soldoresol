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

pub struct Props<T> {
    modelesses: Rc<RefCell<TabModelessList<T>>>,
}

pub enum Msg<Sub> {
    NoOp,
    Sub(On<Sub>),
    RefBase(web_sys::Node),
    ResizeBase,
    Focus(U128Id),
    Close(U128Id),
    SetMinimized(U128Id, bool),
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
    Content: HtmlComponent,
    TabName: HtmlComponent<Props = Content::Props>,
> where
    Content::Props: Clone,
{
    modelesses: Rc<RefCell<TabModelessList<Content::Props>>>,
    element_rect: Option<Rc<tab_modeless::ContainerRect>>,
    floating_tab: HashMap<U128Id, FloatingTab>,
    batch_resize: kagura::util::Batch<Cmd<Self>>,
    base_node: Option<web_sys::Element>,
    _phantom_tab_name: std::marker::PhantomData<TabName>,
}

pub struct TabModelessList<T> {
    data: ModelessList<Modeless<T>>,
}

struct FloatingTab {
    tab_idx: usize,
    modeless_id: U128Id,
    is_selected: bool,
}

struct Modeless<T> {
    data: Rc<RefCell<SelectList<T>>>,
    pos_x: Option<i32>,
    pos_y: Option<i32>,
    is_minimized: bool,
}

impl<T> TabModelessList<T> {
    pub fn open_modeless(&mut self, contents: Vec<T>) {
        self.data.push(Modeless {
            data: Rc::new(RefCell::new(SelectList::new(contents, 0))),
            pos_x: None,
            pos_y: None,
            is_minimized: false,
        });
    }
}

impl<T: PartialEq> TabModelessList<T> {
    pub fn focus_first(&mut self, content: &T) {
        let m_id = self
            .data
            .iter()
            .filter_map(|x| x)
            .find_map(|(m_id, _, m_contents)| {
                if m_contents.data.borrow().iter().any(|m_c| content.eq(m_c)) {
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

impl<Content: HtmlComponent, TabName: HtmlComponent<Props = Content::Props>> Component
    for TabModelessContainer<Content, TabName>
where
    Content::Props: Clone,
{
    type Props = Props<Content::Props>;
    type Msg = Msg<Content::Event>;
    type Event = On<Content::Event>;
}

impl<Content: HtmlComponent, TabName: HtmlComponent<Props = Content::Props>> HtmlComponent
    for TabModelessContainer<Content, TabName>
where
    Content::Props: Clone,
{
}

impl<Content: HtmlComponent, TabName: HtmlComponent<Props = Content::Props>> Constructor
    for TabModelessContainer<Content, TabName>
where
    Content::Props: Clone,
{
    fn constructor(props: Self::Props) -> Self {
        let batch_resize = kagura::util::Batch::new(|mut handle| {
            let a = Closure::wrap(
                Box::new(move || handle(Cmd::chain(Msg::ResizeBase))) as Box<dyn FnMut()>
            );
            let _ = web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("resize", a.as_ref().unchecked_ref());
            a.forget();
        });
        Self {
            modelesses: props.modelesses,
            element_rect: None,
            floating_tab: HashMap::new(),
            batch_resize,
            base_node: None,
            _phantom_tab_name: std::marker::PhantomData,
        }
    }
}

impl<Content: HtmlComponent, TabName: HtmlComponent<Props = Content::Props>> Update
    for TabModelessContainer<Content, TabName>
where
    Content::Props: Clone,
{
    fn update(self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
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
                Cmd::task(self.batch_resize.poll())
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
                        .and_then(|x| x.data.borrow_mut().remove(event.tab_idx));

                    if let Some((content, modeless)) = join_some!(
                        content,
                        self.modelesses.borrow_mut().data.get_mut(&modeless_id)
                    ) {
                        let tab_idx = if let Some(tab_idx) = header_tab_idx {
                            modeless.data.borrow_mut().insert(tab_idx, content);
                            tab_idx
                        } else {
                            modeless.data.borrow_mut().push(content);
                            modeless.data.borrow().len() - 1
                        };

                        if event.is_selected {
                            modeless.data.borrow_mut().set_selected_idx(tab_idx);
                        }
                    }

                    if self
                        .modelesses
                        .borrow()
                        .data
                        .get(&event.modeless_id)
                        .map(|x| x.data.borrow().len() < 1)
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
                        .and_then(|x| x.data.borrow_mut().remove(event.tab_idx));

                    if let Some(content) = content {
                        self.modelesses.borrow_mut().data.push(Modeless {
                            data: Rc::new(RefCell::new(SelectList::new(vec![content], 0))),
                            pos_x: Some(page_x - 10),
                            pos_y: Some(page_y - 10),
                            is_minimized: false,
                        });
                    }

                    if self
                        .modelesses
                        .borrow()
                        .data
                        .get(&event.modeless_id)
                        .map(|x| x.data.borrow().len() < 1)
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

impl<Content: HtmlComponent, TabName: HtmlComponent<Props = Content::Props>> Render<Html>
    for TabModelessContainer<Content, TabName>
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
                                            .data
                                            .borrow()
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
                            .enumerate()
                            .map(|(m_idx, m)| match m {
                                Some((m_id, z_idx, contents)) if !contents.is_minimized => {
                                    TabModeless::<Content, TabName>::empty(
                                        self,
                                        Some(m_id.to_string()),
                                        tab_modeless::Props {
                                            container_rect: Rc::clone(&element_rect),
                                            contents: Rc::clone(&contents.data),
                                            page_x: contents
                                                .pos_x
                                                .unwrap_or(200 + (m_idx % 10) as i32 * 20),
                                            page_y: contents
                                                .pos_y
                                                .unwrap_or(200 + (m_idx % 10) as i32 * 20),
                                            size: [800.0, 600.0],
                                            z_index: z_idx,
                                            modeless_id: U128Id::clone(&m_id),
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

impl<Content: HtmlComponent, TabName: HtmlComponent<Props = Content::Props>> Styled
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
