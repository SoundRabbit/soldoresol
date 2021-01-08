use super::atom::{
    btn::{self, Btn},
    fa,
};
use super::constant;
use super::util::styled::{Style, Styled};
use crate::libs::color::color_system;
use crate::libs::select_list::SelectList;
use kagura::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Props {
    pub size: [f32; 2],
    pub loc: [f32; 2],
    pub z_index: usize,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            size: [30.0, 30.0],
            loc: [0.0, 0.0],
            z_index: 0,
        }
    }
}

pub enum Msg {
    CloseSelf,
}

pub enum On {
    Close,
}

pub struct Modeless {
    size: [f32; 2],
    loc: [f32; 2],
    z_index: usize,
}

impl Constructor for Modeless {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            z_index: props.z_index,
            loc: props.loc,
            size: props.size,
        }
    }
}

impl Component for Modeless {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.z_index = props.z_index;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::CloseSelf => Cmd::sub(On::Close),
        }
    }

    fn render(&self, mut children: Vec<Html>) -> Html {
        children.push(self.render_rsz("rsz-t"));
        children.push(self.render_rsz("rsz-tl"));
        children.push(self.render_rsz("rsz-l"));
        children.push(self.render_rsz("rsz-bl"));
        children.push(self.render_rsz("rsz-b"));
        children.push(self.render_rsz("rsz-br"));
        children.push(self.render_rsz("rsz-r"));
        children.push(self.render_rsz("rsz-tr"));
        Self::styled(Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .style("z-index", format!("{}", self.z_index))
                .style("left", format!("{}%", self.loc[0]))
                .style("top", format!("{}%", self.loc[1]))
                .style("width", format!("{}%", self.size[0]))
                .style("height", format!("{}%", self.size[1])),
            Events::new(),
            children,
        ))
    }
}

impl Modeless {
    fn render_rsz(&self, name: &str) -> Html {
        Html::div(
            Attributes::new().class(Self::class(name)),
            Events::new(),
            vec![],
        )
    }
}

impl Styled for Modeless {
    fn style() -> Style {
        style! {
            "base" {
                "position": "absolute";
                "overflow": "hidden";
                "border-radius": "2px";
                "border": format!("1px solid {}", color_system::gray(255, 9));
            }

            "rsz-t" {
                "position": "absolute";
                "top": "-0.5rem";
                "left": "0.5rem";
                "width": "calc(100% - 1rem)";
                "height": "1rem";
            }

            "rsz-tl" {
                "position": "absolute";
                "top": "-0.5rem";
                "left": "-0.5rem";
                "width": "1rem";
                "height": "1rem";
            }

            "rsz-l" {
                "position": "absolute";
                "top": "0.5rem";
                "left": "-0.5rem";
                "width": "1rem";
                "height": "calc(100% - 1rem)";
            }

            "rsz-bl" {
                "position": "absolute";
                "top": "calc(100% - 0.5rem)";
                "left": "-0.5rem";
                "width": "1rem";
                "height": "1rem";
            }

            "rsz-b" {
                "position": "absolute";
                "top": "calc(100% - 0.5rem)";
                "left": "0.5rem";
                "width": "calc(100% - 1rem)";
                "height": "1rem";
            }

            "rsz-br" {
                "position": "absolute";
                "top": "calc(100% - 0.5rem)";
                "left": "calc(100% - 0.5rem)";
                "width": "1rem";
                "height": "1rem";
            }

            "rsz-r" {
                "position": "absolute";
                "top": "0.5rem";
                "left": "calc(100% - 0.5rem)";
                "width": "1rem";
                "height": "calc(100% - 1rem)";
            }

            "rsz-tr" {
                "position": "absolute";
                "top": "-0.5rem";
                "left": "calc(100% - 0.5rem)";
                "width": "1rem";
                "height": "1rem";
            }
        }
    }
}
