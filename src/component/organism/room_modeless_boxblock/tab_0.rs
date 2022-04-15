use super::super::atom::{
    btn::Btn,
    common::Common,
    fa,
    heading::{self, Heading},
    slider::{self, Slider},
    text,
};
use crate::arena::{block, BlockMut};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {
    boxblock: BlockMut<block::Boxblock>,
}

pub enum Msg {}

pub enum On {}

pub struct Tab0 {
    boxblock: BlockMut<block::Boxblock>,
}

impl Component for Tab0 {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Tab0 {}

impl Constructor for Tab0 {
    fn constructor(props: Props) -> Self {
        Self {
            boxblock: props.boxblock,
        }
    }
}

impl Update for Tab0 {
    fn on_load(self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.boxblock = props.boxblock;
        Cmd::none()
    }
}

impl Render<Html> for Tab0 {
    type Children = Vec<Html>;
    fn render(&self, children: Self::Children) -> Html {
        Self::styled(Html::div(
            Attributes::new()
                .class(RoomModeless::class("common-base"))
                .class("pure-form"),
            Events::new(),
            vec![
                self.boxblock
                    .map(|data| self.render_header(data))
                    .unwrap_or(Common::none()),
                self.boxblock
                    .map(|data| self.render_main(data))
                    .unwrap_or(Common::none()),
            ],
        ))
    }

    fn render_header(&self, boxblock: &block::Boxblock) -> Html {
        Html::div(
            Attributes::new().class(RoomModeless::class("common-header")),
            Events::new(),
            vec![
                Html::label(
                    Attributes::new()
                        .class(RoomModeless::class("common-label"))
                        .string("for", &self.element_id.input_boxblock_name),
                    Events::new(),
                    vec![fa::i("fa-cube")],
                ),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_boxblock_name)
                        .value(boxblock.name()),
                    Events::new().on_input(Msg::SetName),
                    vec![],
                ),
                Html::label(
                    Attributes::new()
                        .class(RoomModeless::class("common-label"))
                        .string("for", &self.element_id.input_boxblock_display_name),
                    Events::new(),
                    vec![Html::text("表示名")],
                ),
                Html::input(
                    Attributes::new().value(&boxblock.display_name().1),
                    Events::new().on_input(Msg::SetDisplayName1),
                    vec![],
                ),
                text::span(""),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_boxblock_display_name)
                        .value(&boxblock.display_name().0),
                    Events::new().on_input(Msg::SetDisplayName0),
                    vec![],
                ),
            ],
        )
    }
}

impl Styled for Tab0 {
    fn style() -> Style {
        style! {}
    }
}
