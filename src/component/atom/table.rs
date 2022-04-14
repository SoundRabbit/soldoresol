use crate::arena::{block, BlockMut};
use crate::table;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Props {
    table: Rc<RefCell<table::Table>>,
    world: BlockMut<block::World>,
}

pub enum Msg {}

pub enum On {}

pub struct Table {
    table: Rc<RefCell<table::Table>>,
    world: BlockMut<block::World>,
}

impl Component for Table {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Table {}

impl Constructor for Table {
    fn constructor(props: &Props) -> Self {
        Self {
            table: props.table,
            world: props.world,
        }
    }
}

impl Update for Table {
    fn on_assemble(self: Pin<&mut Self>) -> Cmd<Self> {
        let table = Rc::clone(&self.table);
        let world = self.world.as_ref();
        Cmd::task(async move {
            table.borrow_mut().render_reserved(world).await;
            Cmd::none()
        })
    }
}

impl Render<Html> for Table {
    type Children = Vec<Html>;
    fn render(&self, children: Self::Children) -> Html {
        let node: &web_sys::Node = self.table.borrow().canvas().as_ref();
        Html::node(node.clone())
    }
}

impl Styled for Table {
    fn style() -> Style {
        style! {}
    }
}
