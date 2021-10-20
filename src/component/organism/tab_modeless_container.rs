use super::molecule::tab_modeless::{self, TabModeless};
use crate::libs::modeless_list::ModelessList;
use crate::libs::random_id::U128Id;
use crate::libs::select_list::SelectList;
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Props {}

pub enum Msg<T> {
    NoOp,
    DisconnectTab {
        event_id: U128Id,
        content: T,
    },
    ConnectTab {
        event_id: U128Id,
        header_tab_idx: Option<usize>,
        modeless_id: U128Id,
    },
}

pub enum On {}

pub struct TabModelessContainer<Content: Constructor, TabName: Constructor<Props = Content::Props>>
where
    Content::Props: Clone,
{
    modelesses: ModelessList<Rc<RefCell<SelectList<Content::Props>>>>,
    element: Option<Rc<web_sys::Element>>,
    floating_tab: HashMap<U128Id, Content::Props>,
    _phantom_tab_name: std::marker::PhantomData<TabName>,
}

impl<Content: Constructor, TabName: Constructor<Props = Content::Props>>
    TabModelessContainer<Content, TabName>
where
    Content::Props: Clone,
{
    pub fn open_modeless(&mut self, contents: Vec<Content::Props>) {
        self.modelesses
            .push(Rc::new(RefCell::new(SelectList::new(contents, 0))));
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
                if m_contents.borrow().iter().any(|m_c| content.eq(m_c)) {
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
    type Msg = Msg<Content::Props>;
    type Sub = On;
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
    fn update(&mut self, props: &Props, msg: Msg<Content::Props>) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::DisconnectTab { event_id, content } => {
                self.floating_tab.insert(event_id, content);
                Cmd::none()
            }
            Msg::ConnectTab {
                event_id,
                header_tab_idx,
                modeless_id,
            } => {
                if let Some((content, modeless)) = join_some!(
                    self.floating_tab.remove(&event_id),
                    self.modelesses.get_mut(&modeless_id)
                ) {
                    if let Some(tab_idx) = header_tab_idx {
                        modeless.borrow_mut().insert(tab_idx, content);
                    } else {
                        modeless.borrow_mut().push(content);
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
    fn render(&self, _: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        if let Some(element) = self.element.as_ref() {
            Html::div(
                Attributes::new(),
                Events::new(),
                self.modelesses
                    .iter()
                    .enumerate()
                    .map(|(m_idx, m)| match m {
                        None => Html::div(Attributes::new(), Events::new(), vec![]),
                        Some((m_id, z_idx, contents)) => TabModeless::<Content, TabName>::empty(
                            tab_modeless::Props {
                                container_element: Rc::clone(&element),
                                contents: Rc::clone(contents),
                                page_x: 200 + (m_idx % 10) as i32 * 20,
                                page_y: 200 + (m_idx % 10) as i32 * 20,
                                size: [30.0, 30.0],
                                z_index: z_idx,
                                modeless_id: U128Id::clone(&m_id),
                            },
                            Sub::map(|sub| match sub {
                                tab_modeless::On::DisconnectTab {
                                    event_id, content, ..
                                } => Msg::DisconnectTab { event_id, content },
                                tab_modeless::On::ConnectTab {
                                    event_id,
                                    header_tab_idx,
                                    modeless_id,
                                } => Msg::ConnectTab {
                                    event_id,
                                    header_tab_idx,
                                    modeless_id,
                                },
                                _ => Msg::NoOp,
                            }),
                        ),
                    })
                    .collect(),
            )
        } else {
            Html::div(Attributes::new(), Events::new(), vec![])
        }
    }
}
