use kagura::prelude::*;

pub fn span<Msg>(text: impl Into<String>) -> Html<Msg> {
    Html::span(Attributes::new(), Events::new(), vec![Html::text(text)])
}

pub fn div<Msg>(text: impl Into<String>) -> Html<Msg> {
    Html::div(Attributes::new(), Events::new(), vec![Html::text(text)])
}
