use super::super::atom::btn::{self, Btn};
use super::super::atom::dropdown::{self, Dropdown};
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
                self.character
                    .map(|data| self.render_tab1_main(data))
                    .unwrap_or(Common::none()),
            ],
        )
    }

    fn render_tab1_header(&self, character: &block::Character) -> Html<Self> {
        Html::div(
            Attributes::new().class(RoomModeless::class("common-header")),
            Events::new(),
            vec![
                text::label("使用", ""),
                Dropdown::with_children(
                    dropdown::Props {
                        direction: dropdown::Direction::Bottom,
                        text: dropdown::Text::Text(
                            character
                                .selected_texture()
                                .map(|texture| texture.name().clone())
                                .unwrap_or(String::from("")),
                        ),
                        variant: btn::Variant::DarkLikeMenu,
                        toggle_type: dropdown::ToggleType::Click,
                    },
                    Sub::none(),
                    character
                        .textures()
                        .iter()
                        .enumerate()
                        .map(|(tex_idx, texture)| {
                            Btn::menu(
                                Attributes::new(),
                                Events::new()
                                    .on_click(move |_| Msg::SetSelectedTextureIdx(tex_idx)),
                                vec![Html::text(texture.name())],
                            )
                        })
                        .collect(),
                ),
            ],
        )
    }

    fn render_tab1_main(&self, character: &block::Character) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("tab1-main")),
            Events::new(),
            vec![
                Html::fragment(
                    character
                        .textures()
                        .iter()
                        .enumerate()
                        .map(|(tex_idx, texture)| self.render_tab1_texture(texture, tex_idx))
                        .collect(),
                ),
                self.render_tab1_new_texture(),
            ],
        )
    }

    fn render_tab1_texture(
        &self,
        texture: &block::character::StandingTexture,
        tex_idx: usize,
    ) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("tab1-texture")),
            Events::new(),
            vec![
                Html::input(
                    Attributes::new().value(texture.name()),
                    Events::new().on_input(move |name| Msg::SetTextureName(tex_idx, name)),
                    vec![],
                ),
                self.render_tab1_texture_image(texture.image(), tex_idx),
            ],
        )
    }

    fn render_tab1_texture_image(
        &self,
        image: Option<&BlockMut<resource::ImageData>>,
        tex_idx: usize,
    ) -> Html<Self> {
        image
            .and_then(|image| {
                image.map(|image| {
                    Html::img(
                        Attributes::new()
                            .src(image.url().to_string())
                            .class(Common::bg_transparent()),
                        Events::new().on_click(move |_| {
                            Msg::SetShowingModal(ShowingModal::SelectCharacterTexture(tex_idx))
                        }),
                        vec![],
                    )
                })
            })
            .unwrap_or_else(|| {
                Btn::secondary(
                    Attributes::new(),
                    Events::new().on_click(move |_| {
                        Msg::SetShowingModal(ShowingModal::SelectCharacterTexture(tex_idx))
                    }),
                    vec![Html::text("立ち絵を選択")],
                )
            })
    }

    fn render_tab1_new_texture(&self) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("tab1-texture")),
            Events::new(),
            vec![Btn::secondary(
                Attributes::new(),
                Events::new().on_click(|_| Msg::PushTexture),
                vec![Html::text("立ち絵を追加")],
            )],
        )
    }
}
