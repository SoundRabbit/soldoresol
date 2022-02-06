use super::super::atom::btn::Btn;
use super::super::atom::fa;
use super::super::atom::slider::{self, Slider};
use super::super::atom::text;
use super::super::organism::popup_color_pallet::{self, PopupColorPallet};
use super::*;

impl RoomModelessCharacter {
    pub fn render_tab0(&self) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(RoomModeless::class("common-base"))
                .class("pure-form"),
            Events::new(),
            vec![
                self.character
                    .map(|data| self.render_tab0_header(data))
                    .unwrap_or(Common::none()),
                self.character
                    .map(|data| self.render_tab0_main(data))
                    .unwrap_or(Common::none()),
            ],
        )
    }

    fn render_tab0_header(&self, character: &block::Character) -> Html<Self> {
        Html::div(
            Attributes::new().class(RoomModeless::class("common-header")),
            Events::new(),
            vec![
                Html::label(
                    Attributes::new()
                        .class(RoomModeless::class("common-label"))
                        .string("for", &self.element_id.input_character_name),
                    Events::new(),
                    vec![fa::i("fa-user")],
                ),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_character_name)
                        .value(character.name()),
                    Events::new(),
                    vec![],
                ),
                Html::label(
                    Attributes::new()
                        .class(RoomModeless::class("common-label"))
                        .string("for", &self.element_id.input_character_display_name),
                    Events::new(),
                    vec![Html::text("表示名")],
                ),
                Html::input(
                    Attributes::new().value(&character.display_name().1),
                    Events::new().on_input(Msg::SetDisplayName1),
                    vec![],
                ),
                text::span(""),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_character_display_name)
                        .value(&character.display_name().0),
                    Events::new().on_input(Msg::SetDisplayName0),
                    vec![],
                ),
            ],
        )
    }

    fn render_tab0_main(&self, character: &block::Character) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("tab0-main")),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new().class(Common::keyvalue()),
                    Events::new(),
                    vec![
                        text::span("サイズ"),
                        Slider::empty(
                            slider::Props {
                                position: slider::Position::Linear {
                                    min: 0.1,
                                    max: 10.0,
                                    val: character.size(),
                                    step: 0.1,
                                },
                                range_is_editable: false,
                                theme: slider::Theme::Light,
                            },
                            Sub::map(move |sub| match sub {
                                slider::On::Input(x) => Msg::SetSize(x),
                                _ => Msg::NoOp,
                            }),
                        ),
                        text::span("立ち絵サイズ"),
                        Slider::empty(
                            slider::Props {
                                position: slider::Position::Linear {
                                    min: 0.1,
                                    max: 10.0,
                                    val: character.tex_size(),
                                    step: 0.1,
                                },
                                range_is_editable: false,
                                theme: slider::Theme::Light,
                            },
                            Sub::map(move |sub| match sub {
                                slider::On::Input(x) => Msg::SetTexSize(x),
                                _ => Msg::NoOp,
                            }),
                        ),
                    ],
                ),
                Html::div(
                    Attributes::new().class(Common::keyvalue()),
                    Events::new(),
                    vec![
                        text::span("色"),
                        PopupColorPallet::empty(
                            popup_color_pallet::Props {
                                direction: popup_color_pallet::Direction::Bottom,
                                default_selected: character.color().clone(),
                            },
                            Sub::map(|sub| match sub {
                                popup_color_pallet::On::SelectColor(color) => Msg::SetColor(color),
                            }),
                        ),
                        text::span("立ち絵"),
                        character
                            .texture()
                            .as_ref()
                            .map(|texture| {
                                texture.map(|texture| {
                                    Html::img(
                                        Attributes::new()
                                            .src(texture.url().to_string())
                                            .class(Common::bg_transparent()),
                                        Events::new().on_click(|_| {
                                            Msg::SetShowingModal(
                                                ShowingModal::SelectCharacterTexture,
                                            )
                                        }),
                                        vec![],
                                    )
                                })
                            })
                            .unwrap_or(None)
                            .unwrap_or_else(|| {
                                Btn::secondary(
                                    Attributes::new(),
                                    Events::new().on_click(|_| {
                                        Msg::SetShowingModal(ShowingModal::SelectCharacterTexture)
                                    }),
                                    vec![Html::text("立ち絵を選択")],
                                )
                            }),
                    ],
                ),
            ],
        )
    }
}
