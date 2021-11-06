use kagura::prelude::*;

pub fn div<C: Component>() -> Html<C> {
    Html::div(Attributes::new(), Events::new(), vec![])
}
