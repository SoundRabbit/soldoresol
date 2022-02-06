use super::super::atom::btn::{self, Btn};
use super::super::atom::heading::{self, Heading};
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
            vec![self.render_tab1_header(), self.render_tab1_main()],
        )
    }

    fn render_tab1_header(&self) -> Html<Self> {
        Html::div(Attributes::new(), Events::new(), vec![])
    }

    fn render_tab1_main(&self) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("tab1-main")),
            Events::new(),
            vec![
                self.render_tab1_texture_block("PZ（上）", 2),
                self.render_tab1_texture_block("NZ（下）", 5),
                self.render_tab1_texture_block("PY（奥）", 1),
                self.render_tab1_texture_block("NY（前）", 4),
                self.render_tab1_texture_block("PX（右）", 0),
                self.render_tab1_texture_block("NX（左）", 3),
            ],
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
                self.craftboard
                    .map(|data| self.render_tab1_texture(data, tex_idx))
                    .unwrap_or(Common::none()),
            ],
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
                    vec![Html::text("画像を選択")],
                )
            })
    }
}
