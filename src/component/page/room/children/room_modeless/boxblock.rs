use super::super::atom::btn::{self, Btn};
use super::super::atom::text;
use super::super::molecule::color_pallet::{self, ColorPallet};
use super::super::molecule::tab_menu::{self, TabMenu};
use super::super::util::styled::{Style, Styled};
use crate::arena::block::{self, BlockId};
use crate::arena::resource;
use kagura::prelude::*;
use wasm_bindgen::JsCast;

pub struct Props {
    pub block_arena: block::ArenaRef,
    pub resource_arena: resource::ArenaRef,
    pub boxblock_id: block::BlockId,
}

pub enum Msg {
    NoOp,
    Sub(On),
    PackToDownload,
    Download(toml::Value),
}

pub enum On {
    SetCommonProps {
        name: Option<String>,
        display_name: Option<String>,
        size: Option<[f32; 3]>,
        color: Option<crate::libs::color::Pallet>,
    },
}

pub struct Boxblock {
    block_arena: block::ArenaRef,
    resource_arena: resource::ArenaRef,
    boxblock_id: block::BlockId,
    element_id: ElementId,
}

struct ElementId {
    input_display_name: String,
    input_boxblock_name: String,
}

impl Constructor for Boxblock {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            block_arena: props.block_arena,
            resource_arena: props.resource_arena,
            boxblock_id: props.boxblock_id,
            element_id: ElementId {
                input_display_name: format!("{:X}", crate::libs::random_id::u128val()),
                input_boxblock_name: format!("{:X}", crate::libs::random_id::u128val()),
            },
        }
    }
}

impl Component for Boxblock {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.block_arena = props.block_arena;
        self.boxblock_id = props.boxblock_id;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::sub(sub),
            Msg::PackToDownload => {
                let block_ids = vec![BlockId::clone(&self.boxblock_id)];

                let task = self.block_arena.pack_to_toml(block_ids.into_iter());
                Cmd::task(move |resolve| {
                    wasm_bindgen_futures::spawn_local(async move {
                        let packed = task().await;
                        resolve(Msg::Download(packed));
                    })
                })
            }
            Msg::Download(packed) => {
                if let Ok(serialized) = toml::to_string(&packed) {
                    let blob = web_sys::Blob::new_with_str_sequence_and_options(
                        &array![serialized].into(),
                        web_sys::BlobPropertyBag::new().type_("application/toml"),
                    )
                    .unwrap();
                    let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
                    let document = web_sys::window().unwrap().document().unwrap();
                    let a = document.create_element("a").unwrap();
                    let _ = a.set_attribute("href", &url);
                    let boxblock_name = self
                        .block_arena
                        .map(&self.boxblock_id, |boxblock: &block::boxblock::Boxblock| {
                            boxblock.name().clone()
                        })
                        .unwrap_or(String::from("ブロック"));
                    let _ =
                        a.set_attribute("download", &(format!("ブロック_{}.toml", boxblock_name)));
                    let _ = a.set_attribute("style", "display: none");
                    let _ = document.body().unwrap().append_child(&a);
                    a.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
                    let _ = document.body().unwrap().remove_child(&a);
                }
                Cmd::none()
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(
            self.block_arena
                .map(&self.boxblock_id, |boxblock: &block::boxblock::Boxblock| {
                    let prop_names = vec![String::from("[common]")];
                    let prop_tabs = vec![self.render_common(boxblock)];

                    Html::div(
                        Attributes::new().class(Self::class("base")),
                        Events::new(),
                        vec![
                            TabMenu::with_children(
                                tab_menu::Props {
                                    selected: 0,
                                    tabs: prop_names,
                                    controlled: true,
                                },
                                Subscription::none(),
                                prop_tabs,
                            ),
                            self.render_bottom_menu(),
                        ],
                    )
                })
                .unwrap_or(Html::none()),
        )
    }
}

impl Boxblock {
    fn render_common(&self, boxblock: &block::boxblock::Boxblock) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("content-base"))
                .class("pure-form"),
            Events::new(),
            vec![Html::div(
                Attributes::new().class(Self::class("common")),
                Events::new(),
                vec![
                    Html::div(
                        Attributes::new().class(Self::class("common-props")),
                        Events::new(),
                        vec![Html::div(
                            Attributes::new().class(Self::class("key-value")),
                            Events::new(),
                            vec![
                                text::label("表示名", &self.element_id.input_display_name),
                                Html::input(
                                    Attributes::new()
                                        .value(boxblock.display_name())
                                        .id(&self.element_id.input_display_name),
                                    Events::new().on_input(|display_name| {
                                        Msg::Sub(On::SetCommonProps {
                                            name: None,
                                            display_name: Some(display_name),
                                            size: None,
                                            color: None,
                                        })
                                    }),
                                    vec![],
                                ),
                                text::label("ブロック名", &self.element_id.input_boxblock_name),
                                Html::input(
                                    Attributes::new()
                                        .value(boxblock.name())
                                        .id(&self.element_id.input_boxblock_name),
                                    Events::new().on_input(|name| {
                                        Msg::Sub(On::SetCommonProps {
                                            name: Some(name),
                                            display_name: None,
                                            size: None,
                                            color: None,
                                        })
                                    }),
                                    vec![],
                                ),
                            ],
                        )],
                    ),
                    Html::div(
                        Attributes::new(),
                        Events::new(),
                        vec![ColorPallet::empty(
                            color_pallet::Props {
                                default_selected: boxblock.color().clone(),
                                title: Some(String::from("ブロック色")),
                            },
                            Subscription::new(move |sub| match sub {
                                color_pallet::On::SelectColor(name_color) => {
                                    Msg::Sub(On::SetCommonProps {
                                        name: None,
                                        display_name: None,
                                        size: None,
                                        color: Some(name_color),
                                    })
                                }
                            }),
                        )],
                    ),
                ],
            )],
        )
    }

    fn render_bottom_menu(&self) -> Html {
        Html::div(
            Attributes::new().class(Self::class("bottom-menu")),
            Events::new(),
            vec![
                Html::div(Attributes::new(), Events::new(), vec![]),
                Html::div(
                    Attributes::new(),
                    Events::new(),
                    vec![Btn::primary(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::PackToDownload),
                        vec![Html::text("ダウンロード")],
                    )],
                ),
            ],
        )
    }
}

impl Styled for Boxblock {
    fn style() -> Style {
        style! {
            "base" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-template-rows": "1fr max-content";
                "height": "100%";
            }

            "content-base" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-auto-rows": "max-content";
                "grid-auto-flow": "row";
                "row-gap": ".65em";
                "overflow-y": "scroll";
                "overflow-x": "hidden";
                "max-height": "100%";
                "padding": "1.2ch 0 1.2ch 1.2ch";
            }

            "base textarea" {
                "resize": "none";
            }

            "common" {
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "grid-template-rows": "20rem";
                "column-gap": ".35em";
                "row-gap": ".65em";
            }

            "common-props" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-template-rows": "max-content 1fr";
                "grid-auto-flow": "row";
                "column-gap": ".35em";
                "row-gap": ".65em";
            }

            "common-imgs" {
                "display": "grid";
                "grid-template-rows": "max-content max-content 1fr max-content";
                "row-gap": "0.35em";
            }

            "common-imgs-container" {
                "overflow": "hidden";
            }

            "common-imgs-img" {
                "height": "100%";
                "width": "100%";
                "object-fit": "contain";
            }

            "common-imgs-list-item" {
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "column-gap": ".15em";
            }

            "common-imgs-list-btn" {
                "display": "grid";
            }

            "key-value" {
                "display": "grid";
                "column-gap": ".35em";
                "row-gap": ".65em";
                "grid-template-columns": "max-content 1fr";
            }

            "root-prop" {
                "display": "grid";
                "row-gap": ".65em";
                "grid-template-columns": "1fr";
            }

            "bottom-menu" {
                "background-color": crate::libs::color::color_system::gray(100, 0).to_string();
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "padding": "0.6ch 1.2ch";
            }
        }
    }
}
