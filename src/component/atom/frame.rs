use super::util::styled::{Style, Styled};
use kagura::prelude::*;

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

impl Constructor for Frame {
    fn constructor(
        props: Self::Props,
        builder: &mut ComponentBuilder<Self::Msg, Self::Sub>,
    ) -> Self {
        Self {
            resize_height: props.resize_height,
            resize_width: props.resize_width,
            element: None,
            class_name: props.class_name,
        }
    }
}

impl Component for Frame {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, builder: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::SetElement(el) => {
                self.element = Some(el);
                Cmd::none()
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(Html::div(
            {
                let attrs = Attributes::new().class(Self::class("base"));
                if let Some(class_name) = &self.class_name {
                    attrs.class(class_name)
                } else {
                    attrs
                }
            },
            Events::new().rendered(if self.element.is_some() {
                None
            } else {
                Some(|el| Msg::SetElement(el))
            }),
            vec![Html::div(Attributes::new(), Events::new(), vec![])],
        ))
    }
}

impl Styled for Frame {
    fn style() -> Style {
        style! {
            "base" {
                "display": "grid";
            }
        }
    }
}
