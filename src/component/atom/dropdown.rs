use super::atom::btn::{self, Btn};
use super::atom::fa;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::Cmd;
use kagura::prelude::*;
use nusa::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

pub struct Props {
    pub direction: Direction,
    pub variant: btn::Variant,
    pub toggle_type: ToggleType,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            direction: Direction::BottomLeft,
            toggle_type: ToggleType::Click,
            variant: btn::Variant::Primary,
        }
    }
}

pub enum Direction {
    Bottom,
    BottomLeft,
    BottomRight,
    RightBottom,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bottom => write!(f, "bottom"),
            Self::BottomLeft => write!(f, "bottom-left"),
            Self::BottomRight => write!(f, "bottom-right"),
            Self::RightBottom => write!(f, "right-bottom"),
        }
    }
}

impl Direction {
    fn caret(&self) -> Html {
        match self {
            Self::Bottom | Self::BottomLeft | Self::BottomRight => fa::fas_i("fa-caret-down"),
            Self::RightBottom => fa::fas_i("fa-caret-right"),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum ToggleType {
    Click,
    Hover,
    Manual(bool),
}

pub enum Msg {
    NoOp,
    SetRoot(web_sys::Node),
    ToggleTo(bool),
}

pub enum On {}

pub struct Dropdown {
    direction: Direction,
    variant: btn::Variant,
    batch_state: Rc<RefCell<BatchState>>,
    batch: js_sys::Function,
}

struct BatchState {
    is_dropdowned: bool,
    toggle_type: ToggleType,
    root: Option<Rc<web_sys::Node>>,
    handle: Option<Box<dyn FnMut(Cmd<Dropdown>)>>,
}

impl Component for Dropdown {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Dropdown {}

impl Constructor for Dropdown {
    fn constructor(props: Self::Props) -> Self {
        let is_dropdowned = if let ToggleType::Manual(is_dropdowned) = &props.toggle_type {
            *is_dropdowned
        } else {
            false
        };

        let batch_state = Rc::new(RefCell::new(BatchState {
            is_dropdowned,
            toggle_type: props.toggle_type,
            root: None,
            handle: None,
        }));

        let batch: js_sys::Function = Closure::wrap(Box::new({
            let batch_state = Rc::clone(&batch_state);
            move |e: web_sys::Event| {
                let mut batch_state = batch_state.borrow_mut();
                let batch_state: &mut BatchState = &mut batch_state;
                if let (Some(root), Some(handle)) =
                    (batch_state.root.as_ref(), batch_state.handle.as_mut())
                {
                    if batch_state.toggle_type == ToggleType::Click {
                        if batch_state.is_dropdowned {
                            handle(Cmd::chain(Msg::ToggleTo(false)));
                        } else if let Some(target) =
                            e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok())
                        {
                            if root.contains(Some(&target)) {
                                handle(Cmd::chain(Msg::ToggleTo(true)));
                            }
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(_)>)
        .into_js_value()
        .unchecked_into();

        let _ = web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("click", &batch);

        Self {
            direction: props.direction,
            variant: props.variant,
            batch_state,
            batch,
        }
    }
}

impl Update for Dropdown {
    fn on_assemble(self: Pin<&mut Self>) -> Cmd<Self> {
        Cmd::batch(kagura::util::Batch::new({
            let batch_state = Rc::clone(&self.batch_state);
            move |handle| {
                batch_state.borrow_mut().handle = Some(handle);
            }
        }))
    }

    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        if let ToggleType::Manual(is_dropdowned) = &props.toggle_type {
            self.batch_state.borrow_mut().is_dropdowned = *is_dropdowned;
        }

        self.direction = props.direction;
        self.variant = props.variant;
        self.batch_state.borrow_mut().toggle_type = props.toggle_type;

        Cmd::none()
    }

    fn update(self: Pin<&mut Self>, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::SetRoot(root) => {
                self.batch_state.borrow_mut().root = Some(Rc::new(root));
                Cmd::none()
            }
            Msg::ToggleTo(is_dropdowned) => {
                self.batch_state.borrow_mut().is_dropdowned = is_dropdowned;
                Cmd::none()
            }
        }
    }
}

impl Render<Html> for Dropdown {
    type Children = (Vec<Html>, Vec<Html>);
    fn render(&self, (text, children): Self::Children) -> Html {
        Self::styled(match &self.batch_state.borrow().toggle_type {
            ToggleType::Click => self.render_toggle_by_click(text, children),
            ToggleType::Hover => self.render_toggle_by_hover(text, children),
            ToggleType::Manual(_) => self.render_toggle_by_manual(text, children),
        })
    }
}

impl Dropdown {
    fn base_class_option(&self) -> &str {
        match &self.variant {
            btn::Variant::Menu => "base-menu",
            btn::Variant::MenuAsSecondary => "base-menu",
            btn::Variant::DarkLikeMenu => "base-menu",
            btn::Variant::SecondaryLikeMenu => "base-menu",
            _ => "base-default",
        }
    }

    fn render_toggle_by_click(&self, text: Vec<Html>, children: Vec<Html>) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .class(Self::class(self.base_class_option())),
            Events::new()
                .refer(self, |root| Msg::SetRoot(root))
                .on_mousedown(self, |e| {
                    e.stop_propagation();
                    Msg::NoOp
                }),
            vec![self.render_toggle_btn(text), self.render_toggled(children)],
        )
    }

    fn render_toggle_by_hover(&self, text: Vec<Html>, children: Vec<Html>) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .class(Self::class(self.base_class_option())),
            Events::new()
                .on("mouseenter", self, |_| Msg::ToggleTo(true))
                .on("mouseleave", self, |_| Msg::ToggleTo(false)),
            vec![self.render_toggle_btn(text), self.render_toggled(children)],
        )
    }

    fn render_toggle_by_manual(&self, text: Vec<Html>, children: Vec<Html>) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .class(Self::class(self.base_class_option())),
            Events::new(),
            vec![self.render_toggle_btn(text), self.render_toggled(children)],
        )
    }

    fn render_toggle_btn(&self, text: Vec<Html>) -> Html {
        Html::button(
            Attributes::new()
                .class("pure-button")
                .class(Btn::class_name(&self.variant))
                .class(Self::class("root-btn"))
                .string(
                    "data-toggled",
                    self.batch_state.borrow().is_dropdowned.to_string(),
                ),
            Events::new(),
            vec![Html::div(
                Attributes::new().class(Self::class("btn")),
                Events::new(),
                text,
            )],
        )
    }

    fn render_toggled(&self, children: Vec<Html>) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("content"))
                .class(Self::class(&format!("content-{}", &self.direction)))
                .string(
                    "data-toggled",
                    self.batch_state.borrow().is_dropdowned.to_string(),
                ),
            Events::new(),
            if self.batch_state.borrow().is_dropdowned {
                children
            } else {
                vec![]
            },
        )
    }
}

impl std::ops::Drop for Dropdown {
    fn drop(&mut self) {
        let _ = web_sys::window()
            .unwrap()
            .remove_event_listener_with_callback("click", &self.batch);
    }
}

impl Styled for Dropdown {
    fn style() -> Style {
        style! {
            ".base" {
                "position": "relative";
                "overflow": "visible !important";
            }

            ".base-menu" {
                "justify-self": "stretch";
                "display": "grid";
            }

            ".base-default" {
                "max-width": "max-content";
            }

            ".root-btn" {
                "height": "100%";
            }

            r#".root-btn[data-toggled="false"]"# {
                "z-index": "auto";
            }

            r#".root-btn[data-toggled="true"]"# {
                "z-index": format!("{}", super::constant::z_index::MASK + 1);
            }

            ".btn" {
                "display": "flex";
                "align-items": "center";
            }

            ".btn > *:not(:first-child)" {
                "margin-right": "1ch";
            }

            ".btn > *:not(:last-child)" {
                "flex-gorw": "1";
            }

            ".content" {
                "position": "absolute";
                "z-index": format!("{}", super::constant::z_index::MASK + 1);
                "grid-auto-rows": "max-content";
                "grid-auto-flow": "rows";
                "row-gap": "0.05rem";
                "padding-top": "0.05rem";
                "padding-bottom": "0.05rem";
                "justify-items": "stretch";
                "background-color": crate::libs::color::color_system::gray(100, 0).to_string();
                "border-radius": "2px";
                "display": "grid";
            }

            r#".content[data-toggled="false"]"# {
                "display": "none";
            }
            r#".content[data-toggled="true"]"# {
                "display": "grid";
            }

            ".content-bottom" {
                "top": "100%";
                "left": "0";
                "right": "0";
                "grid-template-columns": "1fr";
            }

            ".content-bottom-left" {
                "top": "100%";
                "right": "0";
                "grid-template-columns": "max-content";
            }

            ".content-bottom-right" {
                "top": "100%";
                "left": "0";
                "grid-template-columns": "max-content";

            }

            ".content-right-bottom" {
                "top": "0";
                "left": "100%";
                "grid-template-columns": "max-content";
            }

            ".menu-heading" {
                "padding": ".5em .5em";
                "line-height": "1.5";
                "align-items": "center";
                "display": "grid";
                "grid-template-columns": "1fr max-content 1fr";
                "column-gap": ".25em";
            }

            ".menu-heading:before, .menu-heading:after" {
                "content": "''";
                "height": ".15rem";
                "background-color": crate::libs::color::Pallet::gray(1);
            }
        }
    }
}
