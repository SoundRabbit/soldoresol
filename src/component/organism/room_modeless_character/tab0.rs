use super::super::atom::btn::{self, Btn};
use super::super::atom::chat_message;
use super::super::atom::dropdown::{self, Dropdown};
use super::super::atom::fa;
use super::super::atom::heading::{self, Heading};
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
                    Events::new().on_input(Msg::SetName),
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
                self.render_tab0_props(character),
                Heading::h3(
                    heading::Variant::Light,
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text("概要")],
                ),
                self.render_tab0_description(),
                Heading::h3(
                    heading::Variant::Light,
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text("立ち絵")],
                ),
                self.render_tab0_textures(character),
            ],
        )
    }

    fn render_tab0_props(&self, character: &block::Character) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("tab0-content")),
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
                        text::label("立ち絵", ""),
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
                ),
            ],
        )
    }

    fn render_tab0_description(&self) -> Html<Self> {
        TabMenu::with_children(
            tab_menu::Props {
                selected: match &self.description_view {
                    DescriptionView::Edit(..) => 0,
                    DescriptionView::View(..) => 1,
                },
                tabs: vec![String::from("編集"), String::from("表示")],
                controlled: true,
            },
            Sub::map(|sub| match sub {
                tab_menu::On::ChangeSelectedTab(0) => Msg::SetDescriptionViewAsEdit(None),
                tab_menu::On::ChangeSelectedTab(1) => Msg::SetDescriptionViewAsView,
                tab_menu::On::ChangeSelectedTab(_) => Msg::NoOp,
            }),
            vec![
                match &self.description_view {
                    DescriptionView::Edit(description) => {
                        self.render_tab0_description_edit(description)
                    }
                    _ => Html::none(),
                },
                match &self.description_view {
                    DescriptionView::View(description) => {
                        self.render_tab0_description_view(description)
                    }
                    _ => Html::none(),
                },
            ],
        )
    }

    fn render_tab0_description_edit(&self, description: &String) -> Html<Self> {
        Html::textarea(
            Attributes::new()
                .value(description)
                .class(Self::class("tab0-textarea")),
            Events::new().on_input(|desc| Msg::SetDescriptionViewAsEdit(Some(desc))),
            vec![],
        )
    }

    fn render_tab0_description_view(
        &self,
        description: &block::chat_message::Message,
    ) -> Html<Self> {
        chat_message::div(Attributes::new(), Events::new(), description)
    }

    fn render_tab0_textures(&self, character: &block::Character) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("tab0-content")),
            Events::new(),
            vec![
                Html::fragment(
                    character
                        .textures()
                        .iter()
                        .enumerate()
                        .map(|(tex_idx, texture)| self.render_tab0_texture(texture, tex_idx))
                        .collect(),
                ),
                self.render_tab0_new_texture(),
            ],
        )
    }
    fn render_tab0_texture(
        &self,
        texture: &block::character::StandingTexture,
        tex_idx: usize,
    ) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("tab0-texture")),
            Events::new(),
            vec![
                Html::input(
                    Attributes::new().value(texture.name()),
                    Events::new().on_input(move |name| Msg::SetTextureName(tex_idx, name)),
                    vec![],
                ),
                self.render_tab0_texture_image(texture.image(), tex_idx),
            ],
        )
    }

    fn render_tab0_texture_image(
        &self,
        image: Option<&BlockRef<resource::ImageData>>,
        tex_idx: usize,
    ) -> Html<Self> {
        image
            .and_then(|image| {
                image.map(|image| {
                    Html::img(
                        Attributes::new()
                            .draggable(false)
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

    fn render_tab0_new_texture(&self) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("tab0-texture")),
            Events::new(),
            vec![Btn::secondary(
                Attributes::new(),
                Events::new().on_click(|_| Msg::PushTexture),
                vec![Html::text("立ち絵を追加")],
            )],
        )
    }
}
