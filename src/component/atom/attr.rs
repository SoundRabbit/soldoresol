use kagura::prelude::*;

pub fn span<C: Component>(attrs: Attributes, text: impl Into<String>) -> Html<C> {
    Html::span(attrs, Events::new(), vec![Html::text(text)])
}
