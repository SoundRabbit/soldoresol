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
use wasm_bindgen::JsCast;

pub struct Props {}

pub enum Msg<Sub> {
    NoOp,
    Sub(On<Sub>),
    DisconnectTab {
        event_id: U128Id,
        tab_idx: usize,
        modeless_id: U128Id,
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
}

pub struct TabModelessContainer<Content: Constructor, TabName: Constructor<Props = Content::Props>>
where
    Content::Props: Clone,
{
    modelesses: ModelessList<Modeless<Content::Props>>,
    element: Option<Rc<web_sys::Element>>,
    floating_tab: HashMap<U128Id, FloatingTab>,
    _phantom_tab_name: std::marker::PhantomData<TabName>,
}

struct FloatingTab {
    tab_idx: usize,
    modeless_id: U128Id,
}

struct Modeless<T> {
    data: Rc<RefCell<SelectList<T>>>,
    pos_x: Option<i32>,
    pos_y: Option<i32>,
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
            element: None,
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
    fn ref_node(&mut self, _: &Props, name: String, node: web_sys::Node) -> Cmd<Self> {
        if self.element.is_none() && name == "base" {
            if let Ok(node) = node.dyn_into::<web_sys::Element>() {
                self.element = Some(Rc::new(node));
                return Cmd::chain(Msg::NoOp);
            }
        }
        Cmd::none()
    }

    fn update(&mut self, props: &Props, msg: Msg<Content::Sub>) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::sub(sub),
            Msg::DisconnectTab {
                event_id,
                tab_idx,
                modeless_id,
            } => {
                self.floating_tab.insert(
                    event_id,
                    FloatingTab {
                        tab_idx,
                        modeless_id,
                    },
                );
                Cmd::none()
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
                        if let Some(tab_idx) = header_tab_idx {
                            modeless.data.borrow_mut().insert(tab_idx, content);
                        } else {
                            modeless.data.borrow_mut().push(content);
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
                Cmd::none()
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
                Cmd::none()
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
                        let e = unwrap_or!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
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
                    if let Some(element) = self.element.as_ref() {
                        Html::fragment(
                            self.modelesses
                                .iter()
                                .enumerate()
                                .map(|(m_idx, m)| match m {
                                    None => Html::div(Attributes::new(), Events::new(), vec![]),
                                    Some((m_id, z_idx, contents)) => {
                                        TabModeless::<Content, TabName>::empty(
                                            tab_modeless::Props {
                                                container_element: Rc::clone(&element),
                                                contents: Rc::clone(&contents.data),
                                                page_x: contents
                                                    .pos_x
                                                    .unwrap_or(200 + (m_idx % 10) as i32 * 20),
                                                page_y: contents
                                                    .pos_y
                                                    .unwrap_or(200 + (m_idx % 10) as i32 * 20),
                                                size: [0.4, 0.6],
                                                z_index: z_idx,
                                                modeless_id: U128Id::clone(&m_id),
                                            },
                                            Sub::map(|sub| match sub {
                                                tab_modeless::On::DisconnectTab {
                                                    event_id,
                                                    tab_idx,
                                                    modeless_id,
                                                } => Msg::DisconnectTab {
                                                    event_id,
                                                    tab_idx,
                                                    modeless_id,
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
                                                tab_modeless::On::Sub(sub) => {
                                                    Msg::Sub(On::Sub(sub))
                                                }
                                                _ => Msg::NoOp,
                                            }),
                                        )
                                    }
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
        }
    }
}
