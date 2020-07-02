use kagura::prelude::*;

pub fn span<Msg>(text: impl Into<String>) -> Html<Msg> {
    Html::span(Attributes::new(), Events::new(), vec![Html::text(text)])
}
