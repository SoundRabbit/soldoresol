use crate::arena::{block, BlockMut};
use crate::libs::random_id::U128Id;
use crate::table;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

pub struct Props {
    pub table: Rc<RefCell<table::Table>>,
    pub world: BlockMut<block::World>,
}

pub enum Msg {
    ResizeCanvas,
}

pub enum On {
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
}

pub struct Table {
    table: Rc<RefCell<table::Table>>,
    world: BlockMut<block::World>,
    canvas: Rc<web_sys::HtmlCanvasElement>,
}

impl Component for Table {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Table {}

impl Constructor for Table {
    fn constructor(props: Self::Props) -> Self {
        let canvas = props.table.borrow().canvas();
        Self::set_attribute(&canvas);
        Self {
            table: props.table,
            world: props.world,
            canvas,
        }
    }
}

impl Update for Table {
    fn on_assemble(mut self: Pin<&mut Self>) -> Cmd<Self> {
        Cmd::list(vec![
            self.as_mut().cmds(),
            Cmd::task(async move { Cmd::chain(Msg::ResizeCanvas) }),
            Cmd::batch(kagura::util::Batch::new(|mut handle| {
                let a = Closure::wrap(
                    Box::new(move || handle(Cmd::chain(Msg::ResizeCanvas))) as Box<dyn FnMut()>
                );
                let _ = web_sys::window()
                    .unwrap()
                    .add_event_listener_with_callback("resize", a.as_ref().unchecked_ref());
                a.forget();
            })),
        ])
    }

    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.table = props.table;
        self.world = props.world;

        let canvas = self.table.borrow().canvas();

        if !self.canvas.is_same_node(Some(canvas.as_ref())) {
            self.canvas = canvas;
            Self::set_attribute(&self.canvas);
        }

        self.table.borrow_mut().render_reserved(self.world.as_ref());
        self.cmds()
    }

    fn update(self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::ResizeCanvas => {
                self.table.borrow_mut().reset_size();
                self.table.borrow_mut().render_reserved(self.world.as_ref());
                Cmd::none()
            }
        }
    }
}

impl Table {
    fn cmds(self: Pin<&mut Self>) -> Cmd<Self> {
        let updates = self.table.borrow_mut().take_updated();

        if !updates.insert.is_empty() || !updates.update.is_empty() {
            Cmd::submit(On::UpdateBlocks {
                insert: updates.insert,
                update: updates.update,
            })
        } else {
            Cmd::none()
        }
    }
}

impl Table {
    fn set_attribute(canvas: &web_sys::HtmlCanvasElement) {
        let _ = canvas.set_attribute("class", Self::class("canvas").as_str());
    }
}

impl Render<Html> for Table {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        let node = self.table.borrow().canvas().as_ref().clone();
        Self::styled(Html::node(node.into()))
    }
}

impl Styled for Table {
    fn style() -> Style {
        style! {
            ".canvas" {
                "width": "100%";
                "height": "100%";
            }
        }
    }
}
