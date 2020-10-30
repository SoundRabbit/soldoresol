use super::util::{Prop, State};
use crate::skyway::Peer;
use crate::Config;
use kagura::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

pub struct Props {
    pub peer: Prop<Peer>,
}

pub enum Msg {
    SetPeerId(String),
}

pub enum On {
    Open(String),
}

pub struct PeerConnection {}

impl Constructor for PeerConnection {
    fn constructor(
        props: Self::Props,
        builder: &mut ComponentBuilder<Self::Msg, Self::Sub>,
    ) -> Self {
        let peer = props.peer;

        builder.add_batch(move |mut handler| {
            let a = Closure::wrap(Box::new({
                let peer = peer.clone();
                move || handler(Msg::SetPeerId(peer.id()))
            }) as Box<dyn FnMut()>);
            peer.on("open", Some(a.as_ref().unchecked_ref()));
            a.forget();
        });

        Self {}
    }
}

impl Component for PeerConnection {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::SetPeerId(peer_id) => Cmd::Sub(On::Open(peer_id)),
        }
    }

    fn render(&self, mut children: Vec<Html>) -> Html {
        Html::fragment(children)
    }
}
