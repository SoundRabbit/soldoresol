use kagura::prelude::*;

pub struct State;

pub struct Msg;

pub struct Sub;

pub fn new() -> Component<Msg, State, Sub> {
    Component::new(init, update, render)
}

fn init() -> (State, Cmd<Msg, Sub>) {
    (State, Cmd::none())
}

fn update(_: &mut State, _: Msg) -> Cmd<Msg, Sub> {
    Cmd::none()
}

fn render(_: &State) -> Html<Msg> {
    Html::div(
        Attributes::new().id("app"),
        Events::new(),
        vec![render_side_menu()],
    )
}

fn render_side_menu() -> Html<Msg> {
    Html::div(
        Attributes::new().id("app-side-menu"),
        Events::new(),
        vec![
            render_side_menu_item("テーブル"),
            render_side_menu_item("画像"),
            render_side_menu_item("音楽"),
            render_side_menu_item("キャラクター"),
            render_side_menu_item("オブジェクト"),
            render_side_menu_item("資料"),
            render_side_menu_item("チャット"),
        ],
    )
}

fn render_side_menu_item(text: impl Into<String>) -> Html<Msg> {
    Html::div(
        Attributes::new().class("app-side-menu-item"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("app-side-menu-item-content-wrapper"),
                Events::new(),
                vec![Html::div(
                    Attributes::new().class("app-side-menu-item-text"),
                    Events::new(),
                    vec![Html::text(text)],
                )],
            ),
            Html::div(
                Attributes::new().class("app-side-menu-item-content-wrapper"),
                Events::new(),
                vec![Html::div(
                    Attributes::new().class("app-side-menu-item-frame"),
                    Events::new(),
                    vec![],
                )],
            ),
        ],
    )
}
