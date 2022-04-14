use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use wasm_bindgen::JsCast;

pub struct Props {
    resize_height: bool,
    resize_width: bool,
    class_name: Option<String>,
}

pub enum Msg {
    SetElement(web_sys::Node),
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
    type Event = On;
}

impl HtmlComponent for Frame {}

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
    fn update(self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::SetElement(node) => {
                self.element = node.dyn_into::<web_sys::Element>().ok();
                Cmd::none()
            }
        }
    }
}

impl Render<Html> for Frame {
    type Children = ();
    fn render(&self, _children: ()) -> Html {
        Self::styled(Html::div(
            {
                let attrs = Attributes::new().class(Self::class("base"));
                if let Some(class_name) = &self.class_name {
                    attrs.class(class_name)
                } else {
                    attrs
                }
            },
            Events::new().refer(self, |node| Msg::SetElement(node)),
            vec![Html::div(Attributes::new(), Events::new(), vec![])],
        ))
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
