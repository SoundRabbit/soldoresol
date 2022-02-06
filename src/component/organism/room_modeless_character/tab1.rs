use super::super::atom::btn::{self, Btn};
use super::super::atom::dropdown::{self, Dropdown};
use super::super::atom::heading::{self, Heading};
use super::super::atom::text;
use super::*;

impl RoomModelessCharacter {
    pub fn render_tab1(&self) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(RoomModeless::class("common-base"))
                .class("pure-form"),
            Events::new(),
            vec![
                self.character
                    .map(|data| self.render_tab1_header(data))
                    .unwrap_or(Common::none()),
                self.render_tab1_main(),
            ],
        )
    }

    fn render_tab1_header(&self, character: &block::Character) -> Html<Self> {
        Html::div(
            Attributes::new().class(RoomModeless::class("common-header")),
            Events::new(),
            vec![
                text::span("使用"),
                Dropdown::with_children(
                    dropdown::Props {
                        direction: dropdown::Direction::Bottom,
                        text: dropdown::Text::Text(String::from("[default]]")),
                        variant: btn::Variant::DarkLikeMenu,
                        toggle_type: dropdown::ToggleType::Click,
                    },
                    Sub::none(),
                    vec![],
                ),
            ],
        )
    }

    fn render_tab1_main(&self) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("tab1-main")),
            Events::new(),
            vec![self.render_tab1_texture_block("PZ（上）", 2)],
        )
    }

    fn render_tab1_texture_block(&self, name: impl Into<String>, tex_idx: usize) -> Html<Self> {
        Html::div(
            Attributes::new(),
            Events::new(),
            vec![
                Heading::h5(
                    heading::Variant::Light,
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text(name)],
                ),
                self.character
                    .map(|data| self.render_tab1_texture(data, tex_idx))
                    .unwrap_or(Common::none()),
            ],
        )
    }

    fn render_tab1_texture(&self, character: &block::Character, tex_idx: usize) -> Html<Self> {
        character
            .texture()
            .map(|texture| {
                texture.map(|texture| {
                    Html::img(
                        Attributes::new()
                            .src(texture.url().to_string())
                            .class(Common::bg_transparent()),
                        Events::new().on_click(move |_| {
                            Msg::SetShowingModal(ShowingModal::SelectCharacterTexture)
                        }),
                        vec![],
                    )
                })
            })
            .unwrap_or(None)
            .unwrap_or_else(|| {
                Btn::secondary(
                    Attributes::new(),
                    Events::new().on_click(move |_| {
                        Msg::SetShowingModal(ShowingModal::SelectCharacterTexture)
                    }),
                    vec![Html::text("立ち絵を選択")],
                )
            })
    }
}
