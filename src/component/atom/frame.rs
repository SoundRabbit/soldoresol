use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use wasm_bindgen::JsCast;

pub struct Props {
    resize_height: bool,
    resize_width: bool,
    class_name: Option<String>,
}

pub enum Msg {
    SetElement(web_sys::Element),
}

pub enum On {}

pub struct Frame {
    resize_height: bool,
    resize_width: bool,
    element: Option<web_sys::Element>,
    class_name: Option<String>,
}

impl Props {
    pub fn new() -> Self {
        Self {
            resize_height: true,
            resize_width: true,
            class_name: None,
        }
    }

    pub fn resize_height(mut self, flag: bool) -> Self {
        self.resize_height = flag;
        self
    }

    pub fn resize_width(mut self, flag: bool) -> Self {
        self.resize_width = flag;
        self
    }

    pub fn class(mut self, name: impl Into<String>) -> Self {
        self.class_name = Some(name.into());
        self
    }
}

impl Component for Frame {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for Frame {
    fn constructor(props: &Props) -> Self {
        Self {
            resize_height: props.resize_height,
            resize_width: props.resize_width,
            element: None,
            class_name: props.class_name.clone(),
        }
    }
}

impl Update for Frame {
    fn ref_node(&mut self, _: &Props, node_name: String, node: web_sys::Node) -> Cmd<Self> {
        if node_name == "frame" && self.element.is_none() {
            if let Ok(element) = node.dyn_into::<web_sys::Element>() {
                self.element = Some(element);
            }
        }
        Cmd::none()
    }
}

impl Render for Frame {
    fn render(&self, _: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(
            Html::div(
                {
                    let attrs = Attributes::new().class(Self::class("base"));
                    if let Some(class_name) = &self.class_name {
                        attrs.class(class_name)
                    } else {
                        attrs
                    }
                },
                Events::new(),
                vec![Html::div(Attributes::new(), Events::new(), vec![])],
            )
            .ref_name("frame"),
        )
    }
}

impl Styled for Frame {
    fn style() -> Style {
        style! {
            ".base" {
                "display": "grid";
            }
        }
    }
}
