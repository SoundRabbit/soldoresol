use super::*;

impl WorldView {
    pub fn render_prefab(&self) -> Vec<Html<Self>> {
        vec![self.render_prefab_characters(), self.render_prefab_scenes()]
    }

    fn render_prefab_characters(&self) -> Html<Self> {
        Collapse::with_children(
            collapse::Props { is_collapsed: true },
            Sub::none(),
            vec![
                Html::text("キャラクター"),
                self.world
                    .map(|world: &block::World| {
                        Html::fragment(
                            world
                                .prefab_characters()
                                .iter()
                                .map(|character| self.render_prefab_character(character))
                                .collect(),
                        )
                    })
                    .unwrap_or(Html::none()),
            ],
        )
    }

    fn render_prefab_character(&self, character: &BlockMut) -> Html<Self> {
        let character_id = character.id();
        character
            .map(|character: &block::PrefabCharacter| {
                Collapse::with_children(
                    collapse::Props { is_collapsed: true },
                    Sub::none(),
                    vec![Html::div(
                        Attributes::new().class(Self::class("item")),
                        Events::new(),
                        vec![
                            Html::div(
                                Attributes::new().class(Common::keyvalue()),
                                Events::new(),
                                vec![
                                    Btn::primary(
                                        Attributes::new(),
                                        Events::new(),
                                        vec![Html::text("編集")],
                                    ),
                                    Html::input(
                                        Attributes::new().value(character.name()),
                                        Events::new(),
                                        vec![],
                                    ),
                                ],
                            ),
                            Html::div(
                                Attributes::new()
                                    .class(Common::keyvalue())
                                    .class(Self::class("item-content")),
                                Events::new(),
                                vec![
                                    text::span("データID"),
                                    Html::input(
                                        Attributes::new()
                                            .flag("readonly")
                                            .value(character_id.to_string()),
                                        Events::new(),
                                        vec![],
                                    ),
                                    text::span("表示名"),
                                    Html::ruby(
                                        Attributes::new().class(Common::selectable()),
                                        Events::new(),
                                        {
                                            let display_name = character.display_name();
                                            vec![
                                                Html::text(&display_name.0),
                                                Html::rb(
                                                    Attributes::new(),
                                                    Events::new(),
                                                    vec![Html::text(&display_name.1)],
                                                ),
                                            ]
                                        },
                                    ),
                                ],
                            ),
                        ],
                    )],
                )
            })
            .unwrap_or(Html::none())
    }

    fn render_prefab_scenes(&self) -> Html<Self> {
        Collapse::with_children(
            collapse::Props { is_collapsed: true },
            Sub::none(),
            vec![
                Html::text("sシーン"),
                self.world
                    .map(|world: &block::World| {
                        Html::fragment(
                            world
                                .prefab_scenes()
                                .iter()
                                .map(|scene| self.render_prefab_scene(scene))
                                .collect(),
                        )
                    })
                    .unwrap_or(Html::none()),
            ],
        )
    }

    fn render_prefab_scene(&self, scene: &BlockMut) -> Html<Self> {
        let scene_id = scene.id();
        scene
            .map(|scene: &block::PrefabScene| {
                Collapse::with_children(
                    collapse::Props { is_collapsed: true },
                    Sub::none(),
                    vec![Html::div(
                        Attributes::new().class(Self::class("item")),
                        Events::new(),
                        vec![
                            Html::div(
                                Attributes::new().class(Common::keyvalue()),
                                Events::new(),
                                vec![
                                    Btn::primary(
                                        Attributes::new(),
                                        Events::new(),
                                        vec![Html::text("編集")],
                                    ),
                                    Html::input(
                                        Attributes::new().value(scene.name()),
                                        Events::new(),
                                        vec![],
                                    ),
                                ],
                            ),
                            Html::div(
                                Attributes::new()
                                    .class(Common::keyvalue())
                                    .class(Self::class("item-content")),
                                Events::new(),
                                vec![
                                    text::span("データID"),
                                    Html::input(
                                        Attributes::new()
                                            .flag("readonly")
                                            .value(scene_id.to_string()),
                                        Events::new(),
                                        vec![],
                                    ),
                                ],
                            ),
                        ],
                    )],
                )
            })
            .unwrap_or(Html::none())
    }
}
