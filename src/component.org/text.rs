use kagura::prelude::*;

pub fn span(text: impl Into<String>) -> Html {
    Html::span(Attributes::new(), Events::new(), vec![Html::text(text)])
}

pub fn div(text: impl Into<String>) -> Html {
    Html::div(Attributes::new(), Events::new(), vec![Html::text(text)])
}
