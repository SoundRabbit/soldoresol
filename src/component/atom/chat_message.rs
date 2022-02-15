use crate::arena::block;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;

pub fn div<C: Component>(
    attrs: Attributes<C>,
    events: Events<C::Msg>,
    message: &block::chat_message::Message,
) -> Html<C> {
    Html::div(
        attrs.class(ChatMessage::class("base")),
        events,
        ChatMessage::render(message),
    )
}

pub struct ChatMessage {}

impl ChatMessage {
    fn render<C: Component>(message: &block::chat_message::Message) -> Vec<Html<C>> {
        ChatMessage::styled(
            message
                .iter()
                .map(|message_token| Self::render_token(message_token))
                .collect(),
        )
    }

    fn render_token<C: Component>(message_token: &block::chat_message::MessageToken) -> Html<C> {
        match message_token {
            block::chat_message::MessageToken::Text(text) => Html::text(text),
            block::chat_message::MessageToken::Refer(text) => Html::text(format!("{{{}}}", text)),
            block::chat_message::MessageToken::CommandBlock(cmd, message) => {
                let cmd_name = cmd.name.to_string();
                if cmd_name == "div"
                    || cmd_name == "grid"
                    || cmd_name == "span"
                    || cmd_name == "box"
                    || cmd_name == ""
                {
                    let mut cmds: Vec<_> = cmd
                        .args
                        .iter()
                        .map(block::chat_message::Message::to_string)
                        .collect();
                    cmds.push(cmd_name.clone());
                    let cmds = cmds.join(" ");
                    Html::span(
                        Attributes::new().string("data-cmd", cmds),
                        Events::new(),
                        Self::render(message),
                    )
                } else if cmd_name == "fas" || cmd_name == "far" || cmd_name == "fab" {
                    let args: Vec<_> = cmd
                        .args
                        .iter()
                        .map(block::chat_message::Message::to_string)
                        .collect();
                    let args = args.join(" ");
                    Html::i(
                        Attributes::new().class(cmd_name).class(args),
                        Events::new(),
                        Self::render(message),
                    )
                } else if cmd_name == "rb" {
                    Html::ruby(
                        Attributes::new(),
                        Events::new(),
                        vec![
                            Html::fragment(Self::render(message)),
                            Html::rp(Attributes::new(), Events::new(), vec![Html::text("《")]),
                            Html::rt(
                                Attributes::new(),
                                Events::new(),
                                cmd.args
                                    .iter()
                                    .map(|msg| Html::fragment(Self::render(msg)))
                                    .collect(),
                            ),
                            Html::rp(Attributes::new(), Events::new(), vec![Html::text("》")]),
                        ],
                    )
                } else {
                    Html::span(
                        Attributes::new().string("data-cmd", cmd_name),
                        Events::new(),
                        Self::render(message),
                    )
                }
            }
        }
    }
}

impl Styled for ChatMessage {
    fn style() -> Style {
        style! {
            ".base" {
                "width": "100%";
                "height": "100%";
                "font-family": "sans-serif";
            }

            ".base [data-cmd~='box']" {
                "display": "block";
                "overflow-y": "scroll";
                "max-height": "15em";
                "padding-left": ".35rem";
                "border-left": format!(".35rem solid {}", crate::libs::color::Pallet::gray(3));
            }

            ".base [data-cmd~='div']" {
                "display": "block";
            }

            ".base [data-cmd~='span']" {
                "display": "inline-block";
            }

            ".base [data-cmd~='grid']" {
                "display": "grid";
                "grid-template-columns": "repeat(12, 1fr)";
                "grid-auto-rows": "max-content";
                "column-gap": "0.25ch";
                "row-gap": "0.375em";
            }

            ".base [data-cmd~='1fr']" {
                "grid-area": "auto / span 1";
            }

            ".base [data-cmd~='2fr']" {
                "grid-area": "auto / span 2";
            }

            ".base [data-cmd~='3fr']" {
                "grid-area": "auto / span 3";
            }

            ".base [data-cmd~='4fr']" {
                "grid-area": "auto / span 4";
            }

            ".base [data-cmd~='5fr']" {
                "grid-area": "auto / span 5";
            }

            ".base [data-cmd~='6fr']" {
                "grid-area": "auto / span 6";
            }

            ".base [data-cmd~='7fr']" {
                "grid-area": "auto / span 7";
            }

            ".base [data-cmd~='8fr']" {
                "grid-area": "auto / span 8";
            }

            ".base [data-cmd~='9fr']" {
                "grid-area": "auto / span 9";
            }

            ".base [data-cmd~='10fr']" {
                "grid-area": "auto / span 10";
            }

            ".base [data-cmd~='11fr']" {
                "grid-area": "auto / span 11";
            }

            ".base [data-cmd~='12fr']" {
                "grid-area": "auto / span 12";
            }

            ".base [data-cmd~='1vfr']" {
                "grid-area": "span 1";
            }

            ".base [data-cmd~='2vfr']" {
                "grid-area": "span 2";
            }

            ".base [data-cmd~='3vfr']" {
                "grid-area": "span 3";
            }

            ".base [data-cmd~='4vfr']" {
                "grid-area": "span 4";
            }

            ".base [data-cmd~='sans-serif']" {
                "font-family": "sans-serif";
            }

            ".base [data-cmd~='serif']" {
                "font-family": "serif";
            }

            ".base [data-cmd~='mono']" {
                "font-family": "monospace";
            }

            ".base [data-cmd~='bold']" {
                "font-weight": "bold";
            }

            ".base [data-cmd~='nb']" {
                "word-break": "keep-all";
                "white-space": "nowrap";
            }

            ".base [data-cmd~='left']" {
                "text-align": "left";
            }

            ".base [data-cmd~='right']" {
                "text-align": "right";
            }

            ".base [data-cmd~='center']" {
                "text-align": "center";
            }

            ".base [data-cmd~='large']" {
                "font-size": "1.25em";
            }

            ".base [data-cmd~='huge']" {
                "font-size": "1.5em";
            }

            ".base [data-cmd~='light']" {
                "color": format!("{} !important",crate::libs::color::Pallet::gray(0));
                "background-color": crate::libs::color::Pallet::gray(9);
            }

            ".base [data-cmd~='dark']" {
                "color": format!("{} !important",crate::libs::color::Pallet::gray(9));
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".base [data-cmd~='red']" {
                "color": format!("{} !important",crate::libs::color::Pallet::red(7));
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".base [data-cmd~='orange']" {
                "color":format!("{} !important", crate::libs::color::Pallet::orange(5));
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".base [data-cmd~='yellow']" {
                "color":format!("{} !important", crate::libs::color::Pallet::yellow(8));
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".base [data-cmd~='green']" {
                "color": format!("{} !important",crate::libs::color::Pallet::green(7));
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".base [data-cmd~='blue']" {
                "color":format!("{} !important", crate::libs::color::Pallet::blue(5));
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".base [data-cmd~='purple']" {
                "color": format!("{} !important",crate::libs::color::Pallet::purple(5));
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".base [data-cmd~='pink']" {
                "color": format!("{} !important",crate::libs::color::Pallet::pink(7));
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".base [data-cmd~='bg-light']" {
                "color": crate::libs::color::Pallet::gray(9);
                "background-color": format!("{} !important", crate::libs::color::Pallet::gray(0));
            }

            ".base [data-cmd~='bg-dark']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": format!("{} !important",crate::libs::color::Pallet::gray(9));
            }

            ".base [data-cmd~='bg-red']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": format!("{} !important",crate::libs::color::Pallet::red(7));
            }

            ".base [data-cmd~='bg-orange']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": format!("{} !important",crate::libs::color::Pallet::orange(5));
            }

            ".base [data-cmd~='bg-yellow']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": format!("{} !important",crate::libs::color::Pallet::yellow(8));
            }

            ".base [data-cmd~='bg-green']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": format!("{} !important",crate::libs::color::Pallet::green(7));
            }

            ".base [data-cmd~='bg-blue']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": format!("{} !important",crate::libs::color::Pallet::blue(5));
            }

            ".base [data-cmd~='bg-purple']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": crate::libs::color::Pallet::purple(5);
            }

            ".base [data-cmd~='bg-pink']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": crate::libs::color::Pallet::pink(7);
            }
        }
    }
}
