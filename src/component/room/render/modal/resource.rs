use super::super::super::super::modal;
use super::Msg;
use crate::{resource::Data, Resource};
use kagura::prelude::*;

mod common {
    pub use super::super::common::*;
}

pub fn render(resource: &Resource) -> Html<Msg> {
    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            12,
            Attributes::new(),
            Events::new(),
            vec![
                common::header("画像"),
                modal::body(
                    Attributes::new()
                        .class("scroll-v grid container")
                        .style("min-height", "50vh"),
                    Events::new(),
                    resource
                        .all()
                        .map(|(_, data)| data)
                        .filter_map(|data| {
                            if let Data::Image { url, .. } = data {
                                Some(url)
                            } else {
                                None
                            }
                        })
                        .map(|img_url| {
                            Html::img(
                                Attributes::new()
                                    .class("grid-w-2")
                                    .class("pure-img")
                                    .string("src", img_url.as_str()),
                                Events::new(),
                                vec![],
                            )
                        })
                        .collect(),
                ),
                modal::footer(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text("ファイルはドラッグ & ドロップで追加できます。")],
                ),
            ],
        )],
    )
}
