use super::molecule::modal::{self, Modal};
use crate::arena::resource::{self, ResourceId};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;

pub struct Props {
    pub resource_arena: resource::ArenaRef,
}

pub enum Msg {
    Sub(On),
}

pub enum On {
    Close,
    SelectFile(ResourceId),
}

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
        match msg {
            Msg::Sub(sub) => Cmd::sub(sub),
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(Modal::with_child(
            modal::Props {
                header_title: String::from("ファイル"),
                footer_message: String::from(""),
            },
            Subscription::new(|sub| match sub {
                modal::On::Close => Msg::Sub(On::Close),
            }),
            Html::div(
                Attributes::new(),
                Events::new(),
                self.resource_arena
                    .all_of::<resource::ImageData>()
                    .map(|(r_id, img)| {
                        Html::img(
                            Attributes::new()
                                .draggable(false)
                                .src(&img.url() as &String),
                            Events::new().on_click({
                                let r_id = ResourceId::clone(&r_id);
                                move |_| Msg::Sub(On::SelectFile(r_id))
                            }),
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
