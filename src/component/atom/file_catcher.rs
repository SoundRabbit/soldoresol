use crate::arena::resource::{self, LoadFrom};
use crate::libs::color::color_system;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use std::rc::Rc;

pub struct Props {
    pub attributes: Attributes,
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
}

impl Component for FileCatcher {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for FileCatcher {
    fn constructor(_props: &Props) -> Self {
        Self {
            is_showing_overlay: false,
        }
    }
}

impl Update for FileCatcher {
    fn update(&mut self, _props: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::sub(sub),
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
                            Cmd::task(|resolve| {
                                wasm_bindgen_futures::spawn_local(async move {
                                    if let Some(image_data) =
                                        resource::ImageData::load_from(file).await
                                    {
                                        resolve(Msg::Sub(On::LoadImageData(image_data)));
                                    }
                                });
                            })
                        });
                    }
                }

                Cmd::list(cmds)
            }
        }
    }
}

impl Render for FileCatcher {
    fn render(&self, props: &Props, children: Vec<Html<Self>>) -> Html<Self> {
        let attrs = props.attributes.clone();

        Self::styled(Html::div(
            attrs,
            Events::new()
                .on_dragend(|_| Msg::SetIsShowingOverlay(false))
                .on_dragleave(|_| Msg::SetIsShowingOverlay(false))
                .on_dragover({
                    let ok_to_catch_file = props.ok_to_catch_file;
                    move |e| {
                        if ok_to_catch_file {
                            e.prevent_default();
                            Msg::SetIsShowingOverlay(true)
                        } else {
                            Msg::SetIsShowingOverlay(false)
                        }
                    }
                })
                .on_drop(|e| {
                    let data_transfer = unwrap_or!(e.data_transfer(); Msg::NoOp);
                    let file_list = unwrap_or!(data_transfer.files(); Msg::NoOp);

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
