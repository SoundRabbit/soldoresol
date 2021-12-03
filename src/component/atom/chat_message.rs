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
                if cmd_name == "gr" {
                    let mut cols = vec![];
                    for col in &cmd.args {
                        let col = col.to_string();
                        if col == "k" {
                            cols.push(String::from("max-content"));
                        } else {
                            cols.push(String::from("1fr"));
                        }
                    }
                    Html::span(
                        Attributes::new()
                            .string("data-cmd", cmd_name)
                            .style("grid-template-columns", cols.join(" ")),
                        Events::new(),
                        Self::render(message),
                    )
                } else if cmd_name == "block" {
                    let mut cmds: Vec<_> = cmd
                        .args
                        .iter()
                        .map(block::chat_message::Message::to_string)
                        .collect();
                    cmds.push(String::from("block"));
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
            }

            ".base *[data-cmd~='nb']" {
                "word-break": "keep-all";
                "white-space": "nowrap";
            }

            ".base *[data-cmd~='left']" {
                "text-align": "left";
            }

            ".base *[data-cmd~='right']" {
                "text-align": "right";
            }

            ".base *[data-cmd~='center']" {
                "text-align": "center";
            }

            ".base *[data-cmd~='gr']" {
                "display": "grid";
            }

            ".base *[data-cmd~='box']" {
                "display": "block";
                "overflow-y": "scroll";
                "max-height": "10rem";
                "padding-left": ".35rem";
                "border-left": format!(".35rem solid {}", crate::libs::color::Pallet::gray(3));
            }

            ".base *[data-cmd~='block']" {
                "display": "block";
            }

            ".base *[data-cmd~='large']" {
                "font-size": "1.25em";
            }

            ".base *[data-cmd~='huge']" {
                "font-size": "1.5em";
            }

            ".base *[data-cmd~='red']" {
                "color": crate::libs::color::Pallet::red(7);
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".base *[data-cmd~='orange']" {
                "color": crate::libs::color::Pallet::orange(5);
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".base *[data-cmd~='yellow']" {
                "color": crate::libs::color::Pallet::yellow(8);
            }

            ".base *[data-cmd~='green']" {
                "color": crate::libs::color::Pallet::green(7);
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".base *[data-cmd~='blue']" {
                "color": crate::libs::color::Pallet::blue(5);
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".base *[data-cmd~='purple']" {
                "color": crate::libs::color::Pallet::purple(5);
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".base *[data-cmd~='pink']" {
                "color": crate::libs::color::Pallet::pink(7);
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".base *[data-cmd~='bg-dark']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": crate::libs::color::Pallet::gray(9);
            }

            ".base *[data-cmd~='bg-red']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": crate::libs::color::Pallet::red(7);
            }

            ".base *[data-cmd~='bg-orange']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": crate::libs::color::Pallet::orange(5);
            }

            ".base *[data-cmd~='bg-yellow']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": crate::libs::color::Pallet::yellow(8);
            }

            ".base *[data-cmd~='bg-green']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": crate::libs::color::Pallet::green(7);
            }

            ".base *[data-cmd~='bg-blue']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": crate::libs::color::Pallet::blue(5);
            }

            ".base *[data-cmd~='bg-purple']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": crate::libs::color::Pallet::purple(5);
            }

            ".base *[data-cmd~='bg-pink']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": crate::libs::color::Pallet::pink(7);
            }
        }
    }
}
