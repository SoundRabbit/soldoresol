use crate::arena::resource::{self, LoadFrom};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;

pub struct Props {
    pub ok_to_catch_file: bool,
}

pub enum Msg {
    NoOp,
    Sub(On),
    SetIsShowingOverlay(bool),
    LoadFiles(Vec<web_sys::File>),
}

pub enum On {
    LoadImageData(resource::ImageData),
}

pub struct FileCatcher {
    is_showing_overlay: bool,
    ok_to_catch_file: bool,
}

impl Component for FileCatcher {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for FileCatcher {}

impl Constructor for FileCatcher {
    fn constructor(props: Props) -> Self {
        Self {
            is_showing_overlay: false,
            ok_to_catch_file: props.ok_to_catch_file,
        }
    }
}

impl Update for FileCatcher {
    fn on_load(self: Pin<&mut Self>, props: Props) -> Cmd<Self> {
        self.ok_to_catch_file = props.ok_to_catch_file;
        Cmd::none()
    }

    fn update(self: Pin<&mut Self>, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::submit(sub),
            Msg::SetIsShowingOverlay(is_showing_overlay) => {
                self.is_showing_overlay = is_showing_overlay;
                Cmd::none()
            }
            Msg::LoadFiles(files) => {
                self.is_showing_overlay = false;
                let mut cmds = vec![];
                for file in files {
                    let file_type = file.type_();
                    let splited_file_type: Vec<&str> = file_type.split('/').collect();
                    let file_type_prefix: &str = splited_file_type.get(0).unwrap_or(&"");

                    let file = Rc::new(file);

                    if file_type_prefix == "image" {
                        cmds.push({
                            let file = Rc::clone(&file);
                            Cmd::task(async move {
                                if let Some(image_data) = resource::ImageData::load_from(file).await
                                {
                                    Cmd::submit(On::LoadImageData(image_data))
                                } else {
                                    Cmd::none()
                                }
                            })
                        });
                    }
                }

                Cmd::list(cmds)
            }
        }
    }
}

impl Render<Html> for FileCatcher {
    type Children = (Attributes, Events, Vec<Html>);
    fn render(&self, (attrs, events, children): Self::Children) -> Html {
        Self::styled(Html::div(
            attrs,
            events
                .on_dragend(self, |_| Msg::SetIsShowingOverlay(false))
                .on_dragleave(self, |_| Msg::SetIsShowingOverlay(false))
                .on_dragover(self, {
                    let ok_to_catch_file = self.ok_to_catch_file;
                    move |e| {
                        if ok_to_catch_file {
                            e.prevent_default();
                            Msg::SetIsShowingOverlay(true)
                        } else {
                            Msg::SetIsShowingOverlay(false)
                        }
                    }
                })
                .on_drop(self, |e| {
                    let e = unwrap!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
                    let data_transfer = unwrap!(e.data_transfer(); Msg::NoOp);
                    let file_list = unwrap!(data_transfer.files(); Msg::NoOp);

                    e.prevent_default();

                    let mut files = vec![];
                    for i in 0..(file_list.length()) {
                        if let Some(file) = file_list.get(i) {
                            files.push(file);
                        }
                    }
                    Msg::LoadFiles(files)
                }),
            vec![
                Html::fragment(children),
                if self.is_showing_overlay {
                    Html::div(
                        Attributes::new().class(Self::class("overlay")),
                        Events::new(),
                        vec![],
                    )
                } else {
                    Html::none()
                },
            ],
        ))
    }
}

impl Styled for FileCatcher {
    fn style() -> Style {
        style! {
            ".overlay" {
                "position": "fixed";
                "top": "0";
                "height": "0";
                "width": "100vw";
                "height": "100vh";
                "background-color": crate::libs::color::Pallet::gray(9).a(30);
                "z-index": super::constant::z_index::OVERLAY;
            }
        }
    }
}
