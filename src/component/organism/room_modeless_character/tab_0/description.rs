use super::super::super::atom::{btn::Btn, chat_message};
use crate::arena::{block, BlockMut};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {
    character: BlockMut<block::Character>,
}

pub enum Msg {
    SetDescriptionAsEdit,
    SetDescriptionAsView,
    SetEditingDescription(String),
}

pub enum On {
    SetDescription(String),
}

pub struct Description {
    character: BlockMut<block::Character>,
    description: DescriptionKind,
}

enum DescriptionKind {
    View(block::chat_message::Message),
    Edit(String),
}

impl Component for Description {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Description {}

impl Constructor for Description {
    fn constructor(props: Props) -> Self {
        let description = DescriptionKind::View(Self::description(&props.character));
        Self {
            character: props.character,
            description,
        }
    }
}

impl Update for Description {
    fn on_load(self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        if self.character.id() != props.character.id() {
            self.description = DescriptionKind::View(Self::description(&props.character));
            self.character = props.character;
            Cmd::none()
        } else {
            Cmd::none()
        }
    }

    fn update(self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::SetDescriptionAsEdit => {
                self.description = DescriptionKind::Edit(
                    self.character
                        .map(|character| character.description().raw().clone())
                        .unwrap_or_default(),
                );
                Cmd::none()
            }
            Msg::SetDescriptionAsView => {
                let description = match &mut self.description {
                    DescriptionKind::Edit(description) => Some(description.drain(..).collect()),
                    _ => None,
                };
                self.description = DescriptionKind::View(Self::description(&self.character));
                match description {
                    Some(description) => Cmd::submit(On::SetDescription(description)),
                    None => Cmd::none(),
                }
            }
            Msg::SetEditingDescription(desc) => {
                if let DescriptionKind::Edit(description) = &mut self.description {
                    *description = desc;
                }
                Cmd::none()
            }
        }
    }
}

impl Render<Html> for Description {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            match &self.description {
                DescriptionKind::Edit(description) => self.render_edit(description),
                DescriptionKind::View(description) => self.render_view(description),
            },
        ))
    }
}

impl Description {
    fn description(character: &BlockMut<block::Character>) -> block::chat_message::Message {
        let description = character
            .map(|character| character.description().data().clone())
            .unwrap_or_else(|| block::chat_message::Message::from(vec![]));
        let (description, _) = block::chat_message::map(character.chat_ref(), description);
        description
    }

    fn render_edit(&self, description: &String) -> Vec<Html> {
        vec![
            Btn::primary(
                Attributes::new(),
                Events::new().on_click(self, |_| Msg::SetDescriptionAsView),
                vec![Html::text("保存")],
            ),
            Html::textarea(
                Attributes::new()
                    .value(description)
                    .class(Self::class("base")),
                Events::new().on_input(self, |desc| Msg::SetEditingDescription(desc)),
                vec![],
            ),
        ]
    }

    fn render_view(&self, description: &block::chat_message::Message) -> Vec<Html> {
        vec![
            Btn::secondary(
                Attributes::new(),
                Events::new().on_click(self, |_| Msg::SetDescriptionAsEdit),
                vec![Html::text("編集")],
            ),
            chat_message::div(Attributes::new(), Events::new(), description),
        ]
    }
}

impl Styled for Description {
    fn style() -> Style {
        style! {}
    }
}
