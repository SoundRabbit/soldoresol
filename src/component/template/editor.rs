use super::util::{Prop, State};
use kagura::prelude::*;

pub struct Props {
    pub enable_mouse_event: bool,
}

pub enum Msg {
    SetPressedBtn { btn: u16 },
    SetPressedBtnAndPosition { btn: u16, position: [i32; 2] },
}

pub enum On {
    ChangeMouseState(Prop<MouseState>),
}

pub struct MouseState {
    pub position: [i32; 2],
    pub last_position: [i32; 2],
    pub btn_1: MouseBtnState,
    pub btn_2: MouseBtnState,
    pub btn_3: MouseBtnState,
}

pub struct MouseBtnState {
    pub is_pressed: bool,
    pub is_changed: bool,
    pub last_changed_position: [i32; 2],
}

pub struct Editor {
    is_enable_mouse_event: bool,
    mouse_state: State<MouseState>,
}

impl MouseState {
    fn new() -> Self {
        Self {
            position: [-1, -1],
            last_position: [-1, -1],
            btn_1: MouseBtnState::new(),
            btn_2: MouseBtnState::new(),
            btn_3: MouseBtnState::new(),
        }
    }
}

impl MouseBtnState {
    fn new() -> Self {
        Self {
            is_pressed: false,
            is_changed: false,
            last_changed_position: [-1, -1],
        }
    }
}

impl Constructor for Editor {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            is_enable_mouse_event: props.enable_mouse_event,
            mouse_state: State::new(MouseState::new()),
        }
    }
}

impl Component for Editor {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.is_enable_mouse_event = props.enable_mouse_event;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::SetPressedBtn { btn } => {
                self.set_pressed_btn(btn);
                Cmd::sub(On::ChangeMouseState(self.mouse_state.as_prop()))
            }
            Msg::SetPressedBtnAndPosition { btn, position } => {
                self.set_pressed_btn(btn);
                self.set_position(position);
                Cmd::sub(On::ChangeMouseState(self.mouse_state.as_prop()))
            }
        }
    }

    fn render(&self, children: Vec<Html>) -> Html {
        Html::div(
            Attributes::new().class("template--editor"),
            if self.is_enable_mouse_event {
                Events::new()
                    .on_mousedown(|e| Msg::SetPressedBtn { btn: e.buttons() })
                    .on_mouseup(|e| Msg::SetPressedBtn { btn: e.buttons() })
                    .on_mousemove(|e| Msg::SetPressedBtnAndPosition {
                        btn: e.buttons(),
                        position: [e.page_x(), e.page_y()],
                    })
            } else {
                Events::new()
            },
            children,
        )
    }
}

impl Editor {
    fn set_pressed_btn(&mut self, btn: u16) {
        let btn_1 = btn & 1 != 0;
        if btn_1 != self.mouse_state.btn_1.is_pressed {
            self.mouse_state.btn_1.is_pressed = btn_1;
            self.mouse_state.btn_1.is_changed = true;
            self.mouse_state.btn_1.last_changed_position = self.mouse_state.position.clone();
        } else {
            self.mouse_state.btn_1.is_changed = false;
        }

        let btn_2 = btn & 1 != 0;
        if btn_2 != self.mouse_state.btn_2.is_pressed {
            self.mouse_state.btn_2.is_pressed = btn_2;
            self.mouse_state.btn_2.is_changed = true;
            self.mouse_state.btn_2.last_changed_position = self.mouse_state.position.clone();
        } else {
            self.mouse_state.btn_2.is_changed = false;
        }

        let btn_3 = btn & 1 != 0;
        if btn_3 != self.mouse_state.btn_3.is_pressed {
            self.mouse_state.btn_3.is_pressed = btn_3;
            self.mouse_state.btn_3.is_changed = true;
            self.mouse_state.btn_3.last_changed_position = self.mouse_state.position.clone();
        } else {
            self.mouse_state.btn_3.is_changed = false;
        }
    }

    fn set_position(&mut self, position: [i32; 2]) {
        self.mouse_state.last_position = self.mouse_state.position.clone();
        self.mouse_state.position = position;
    }
}
