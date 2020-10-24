use kagura::prelude::*;

pub struct Props {
    pub variant: Variant,
}

pub enum Variant {
    Primary,
    Danger,
}

pub enum Msg {}

pub enum On {}

pub struct Btn {
    variant: Variant,
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primary => write!(f, "primary"),
            Self::Danger => write!(f, "danger"),
        }
    }
}

impl Constructor for Btn {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            variant: props.variant,
        }
    }
}

impl Component for Btn {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

    fn update(&mut self, _: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        Cmd::none()
    }

    fn render(&self, children: Vec<Html>) -> Html {
        Html::button(
            Attributes::new()
                .class("pure-button")
                .string("data-variant", format!("{}", &self.variant)),
            Events::new(),
            children,
        )
    }
}
