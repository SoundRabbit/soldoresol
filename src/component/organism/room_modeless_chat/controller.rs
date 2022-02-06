use super::*;

impl RoomModelessChat {
    pub fn render_controller(&self) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("controller")),
            Events::new(),
            vec![
                Html::textarea(
                    Attributes::new().value(
                        self.inputing_chat_message
                            .as_ref()
                            .map(String::as_str)
                            .unwrap_or(""),
                    ),
                    Events::new()
                        .on("input", {
                            let ignore_intput = self.inputing_chat_message.is_none();
                            move |e| {
                                if let Some(target) = e
                                    .target()
                                    .and_then(|t| t.dyn_into::<web_sys::HtmlTextAreaElement>().ok())
                                {
                                    if ignore_intput {
                                        target.set_value("");
                                    }
                                    Msg::SetInputingChatMessage(target.value())
                                } else {
                                    Msg::NoOp
                                }
                            }
                        })
                        .on_keydown(|e| {
                            if e.key() == "Enter" && !e.shift_key() {
                                Msg::SendInputingChatMessage(true)
                            } else {
                                Msg::NoOp
                            }
                        }),
                    vec![],
                ),
                Html::div(
                    Attributes::new().class(Self::class("controller-guide")),
                    Events::new(),
                    vec![
                        text::span("Shift＋Enterで改行できます。"),
                        Btn::primary(
                            Attributes::new(),
                            Events::new().on_click(|_| Msg::SendInputingChatMessage(false)),
                            vec![Html::text("送信")],
                        ),
                    ],
                ),
            ],
        )
    }
}
