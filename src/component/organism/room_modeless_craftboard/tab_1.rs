use super::super::atom::btn::{self, Btn};
use super::super::atom::text;
use super::super::organism::room_modeless::RoomModeless;
use super::*;

impl RoomModelessCraftboard {
    pub fn render_tab1(&self) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(RoomModeless::class("common-base"))
                .class("pure-form"),
            Events::new(),
            vec![Html::div(
                Attributes::new()
                    .class(Common::keyvalue())
                    .class(Self::class("tab1-main")),
                Events::new(),
                vec![
                    text::span("PZ（上）"),
                    self.craftboard
                        .map(|data| self.render_tab1_texture(data, 2))
                        .unwrap_or(Common::none()),
                    text::span("NZ（下）"),
                    self.craftboard
                        .map(|data| self.render_tab1_texture(data, 5))
                        .unwrap_or(Common::none()),
                    text::span("PY（奥）"),
                    self.craftboard
                        .map(|data| self.render_tab1_texture(data, 1))
                        .unwrap_or(Common::none()),
                    text::span("NY（前）"),
                    self.craftboard
                        .map(|data| self.render_tab1_texture(data, 4))
                        .unwrap_or(Common::none()),
                    text::span("PX（右）"),
                    self.craftboard
                        .map(|data| self.render_tab1_texture(data, 0))
                        .unwrap_or(Common::none()),
                    text::span("NX（左）"),
                    self.craftboard
                        .map(|data| self.render_tab1_texture(data, 3))
                        .unwrap_or(Common::none()),
                ],
            )],
        )
    }

    fn render_tab1_texture(&self, craftboard: &block::Craftboard, tex_idx: usize) -> Html<Self> {
        craftboard.textures()[tex_idx]
            .as_ref()
            .map(|texture| {
                texture.map(|texture| {
                    Html::img(
                        Attributes::new()
                            .src(texture.url().to_string())
                            .class(Common::bg_transparent()),
                        Events::new().on_click(move |_| {
                            Msg::SetShowingModal(ShowingModal::SelectTexture(tex_idx))
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
                        Msg::SetShowingModal(ShowingModal::SelectTexture(tex_idx))
                    }),
                    vec![Html::text("立ち絵を選択")],
                )
            })
    }
}
