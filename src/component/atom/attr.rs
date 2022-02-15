use kagura::prelude::*;

pub fn span<C: Component>(attrs: Attributes<C>, text: impl Into<String>) -> Html<C> {
    Html::span(attrs, Events::new(), vec![Html::text(text)])
}

pub fn div<C: Component>(attrs: Attributes<C>, text: impl Into<String>) -> Html<C> {
    Html::div(attrs, Events::new(), vec![Html::text(text)])
}
