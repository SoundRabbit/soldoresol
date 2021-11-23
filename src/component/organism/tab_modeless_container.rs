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
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

pub struct Props {}

pub enum Msg<Sub> {
    NoOp,
    Sub(On<Sub>),
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

pub struct TabModelessContainer<Content: Constructor, TabName: Constructor<Props = Content::Props>>
where
    Content::Props: Clone,
{
    modelesses: ModelessList<Modeless<Content::Props>>,
    element_rect: Option<Rc<tab_modeless::ContainerRect>>,
    floating_tab: HashMap<U128Id, FloatingTab>,
    _phantom_tab_name: std::marker::PhantomData<TabName>,
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

impl<Content: Constructor, TabName: Constructor<Props = Content::Props>>
    TabModelessContainer<Content, TabName>
where
    Content::Props: Clone,
{
    pub fn open_modeless(&mut self, contents: Vec<Content::Props>) {
        self.modelesses.push(Modeless {
            data: Rc::new(RefCell::new(SelectList::new(contents, 0))),
            pos_x: None,
            pos_y: None,
            is_minimized: false,
        });
    }
}

impl<Content: Constructor, TabName: Constructor<Props = Content::Props>>
    TabModelessContainer<Content, TabName>
where
    Content::Props: Clone + PartialEq,
{
    pub fn focus_first(&mut self, content: &Content::Props) {
        let m_id = self
            .modelesses
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
            self.modelesses.focus(&m_id);
        }
    }
}

impl<Content: Constructor, TabName: Constructor<Props = Content::Props>> Component
    for TabModelessContainer<Content, TabName>
where
    Content::Props: Clone,
{
    type Props = Props;
    type Msg = Msg<Content::Sub>;
    type Sub = On<Content::Sub>;
}

impl<Content: Constructor, TabName: Constructor<Props = Content::Props>>
    TabModelessContainer<Content, TabName>
where
    Content::Props: Clone,
{
    pub fn new() -> PrepackedComponent<Self> {
        PrepackedComponent::new(Self {
            modelesses: ModelessList::new(),
            element_rect: None,
            floating_tab: HashMap::new(),
            _phantom_tab_name: std::marker::PhantomData,
        })
    }
}

impl<Content: Constructor, TabName: Constructor<Props = Content::Props>> Update
    for TabModelessContainer<Content, TabName>
where
    Content::Props: Clone,
{
    fn on_assemble(&mut self, _: &Props) -> Cmd<Self> {
        Cmd::batch(move |mut handle| {
            let a = Closure::wrap(Box::new(move || handle(Msg::NoOp)) as Box<dyn FnMut()>);
            let _ = web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("resize", a.as_ref().unchecked_ref());
            a.forget();
        })
    }

    fn ref_node(&mut self, _: &Props, ref_name: String, node: web_sys::Node) -> Cmd<Self> {
        if ref_name == "base" {
            if let Ok(node) = node.dyn_into::<web_sys::Element>() {
                let node = node;
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
                    return Cmd::chain(Msg::NoOp);
                }
            }
        }
        Cmd::none()
    }

    fn update(&mut self, props: &Props, msg: Msg<Content::Sub>) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::sub(sub),
            Msg::Focus(modeless_id) => {
                self.modelesses.focus(&modeless_id);
                Cmd::none()
            }
            Msg::Close(modeless_id) => {
                self.modelesses.remove(&modeless_id);
                Cmd::none()
            }
            Msg::SetMinimized(modeless_id, is_minimized) => {
                if let Some(modeless) = self.modelesses.get_mut(&modeless_id) {
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
                Cmd::sub(On::StartDragTab)
            }
            Msg::ConnectTab {
                event_id,
                header_tab_idx,
                modeless_id,
            } => {
                if let Some(event) = self.floating_tab.remove(&event_id) {
                    let content = self
                        .modelesses
                        .get_mut(&event.modeless_id)
                        .and_then(|x| x.data.borrow_mut().remove(event.tab_idx));

                    if let Some((content, modeless)) =
                        join_some!(content, self.modelesses.get_mut(&modeless_id))
                    {
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
                        .get(&event.modeless_id)
                        .map(|x| x.data.borrow().len() < 1)
                        .unwrap_or(false)
                    {
                        self.modelesses.remove(&event.modeless_id);
                    }
                }

                if self.floating_tab.is_empty() {
                    Cmd::sub(On::EndDragTab)
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
                        .get_mut(&event.modeless_id)
                        .and_then(|x| x.data.borrow_mut().remove(event.tab_idx));

                    if let Some(content) = content {
                        self.modelesses.push(Modeless {
                            data: Rc::new(RefCell::new(SelectList::new(vec![content], 0))),
                            pos_x: Some(page_x - 10),
                            pos_y: Some(page_y - 10),
                            is_minimized: false,
                        });
                    }

                    if self
                        .modelesses
                        .get(&event.modeless_id)
                        .map(|x| x.data.borrow().len() < 1)
                        .unwrap_or(false)
                    {
                        self.modelesses.remove(&event.modeless_id);
                    }
                }
                if self.floating_tab.is_empty() {
                    Cmd::sub(On::EndDragTab)
                } else {
                    Cmd::none()
                }
            }
        }
    }
}

impl<Content: Constructor, TabName: Constructor<Props = Content::Props>> Render
    for TabModelessContainer<Content, TabName>
where
    Content::Props: Clone,
{
    fn render(&self, _: &Props, children: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(
            Html::div(
                Attributes::new().class(Self::class("base")),
                Events::new()
                    .on_dragover(|e| {
                        e.prevent_default();
                        Msg::NoOp
                    })
                    .on_drop(move |e| {
                        let data_transfer = unwrap_or!(e.data_transfer(); Msg::NoOp);
                        let data = unwrap_or!(data_transfer.get_data("text/plain").ok(); Msg::NoOp);
                        if TabBtn::validate_prefix::<TabModeless<Content, TabName>>(&data) {
                            let suffix = TabBtn::get_suffix(&data);
                            if let Some(event_id) = suffix.get(0).and_then(|x| U128Id::from_hex(x))
                            {
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
                            .iter()
                            .map(|m| match m {
                                Some((m_id, _, contents)) if contents.is_minimized => Html::div(
                                    Attributes::new().class(Self::class("minimized-item")),
                                    Events::new(),
                                    vec![
                                        Html::div(
                                            Attributes::new()
                                                .class(Self::class("minimized-item-tabs")),
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
                                                            Clone::clone(&content),
                                                            Sub::none(),
                                                        )],
                                                    )
                                                })
                                                .collect(),
                                        ),
                                        Btn::secondary(
                                            Attributes::new()
                                                .class(Self::class("minimized-item-open")),
                                            Events::new().on_click({
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
                                .iter()
                                .enumerate()
                                .map(|(m_idx, m)| match m {
                                    Some((m_id, z_idx, contents)) if !contents.is_minimized => {
                                        TabModeless::<Content, TabName>::empty(
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
                                            Self::modeless_sub(),
                                        )
                                    }
                                    _ => Html::div(Attributes::new(), Events::new(), vec![]),
                                })
                                .collect(),
                        )
                    } else {
                        Html::none()
                    },
                ],
            )
            .ref_name("base"),
        )
    }
}

impl<Content: Constructor, TabName: Constructor<Props = Content::Props>>
    TabModelessContainer<Content, TabName>
where
    Content::Props: Clone,
{
    fn modeless_sub() -> Sub<tab_modeless::On<Content::Sub>, Msg<Content::Sub>> {
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
            tab_modeless::On::Focus(modeless_id) => Msg::Focus(modeless_id),
            tab_modeless::On::Close(modeless_id) => Msg::Close(modeless_id),
            tab_modeless::On::SetMinimized(modeless_id, is_minimized) => {
                Msg::SetMinimized(modeless_id, is_minimized)
            }
            tab_modeless::On::Sub(sub) => Msg::Sub(On::Sub(sub)),
            _ => Msg::NoOp,
        })
    }
}

impl<Content: Constructor, TabName: Constructor<Props = Content::Props>> Styled
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
