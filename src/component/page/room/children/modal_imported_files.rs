use super::molecule::modal::{self, Modal};
use super::util::styled::{Style, Styled};
use crate::arena::resource::{self};
use kagura::prelude::*;

pub struct Props {
    pub resource_arena: resource::ArenaRef,
}

pub enum Msg {}

pub enum On {}

pub struct ModalImportedFiles {
    resource_arena: resource::ArenaRef,
}

impl Constructor for ModalImportedFiles {
    fn constructor(
        props: Self::Props,
        builder: &mut ComponentBuilder<Self::Msg, Self::Sub>,
    ) -> Self {
        Self {
            resource_arena: props.resource_arena,
        }
    }
}

impl Component for ModalImportedFiles {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, builder: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        Cmd::none()
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(Modal::with_child(
            modal::Props {
                header_title: String::from("ファイル"),
                footer_message: String::from(""),
            },
            Subscription::none(),
            Html::div(
                Attributes::new(),
                Events::new(),
                self.resource_arena
                    .all_of::<resource::ImageData>()
                    .map(|(_, img)| {
                        Html::img(
                            Attributes::new().src(&img.url() as &String),
                            Events::new(),
                            vec![],
                        )
                    })
                    .collect(),
            ),
        ))
    }
}

impl Styled for ModalImportedFiles {
    fn style() -> Style {
        style! {}
    }
}
