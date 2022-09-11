use super::page::room::{self, Room};
use crate::arena::{block, Arena, BlockKind, BlockMut, Pack, PackDepth};
use crate::libs::bcdice::js::DynamicLoader;
use crate::libs::js_object::Object;
use crate::libs::random_id::U128Id;
use crate::libs::skyway::{self, DataConnection, MeshRoom, Peer};
use kagura::prelude::*;
use nusa::prelude::*;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

mod task;

pub struct Props {
    pub peer: Rc<Peer>,
    pub peer_id: Rc<String>,
    pub room: Rc<MeshRoom>,
    pub room_id: Rc<String>,
    pub client_id: Rc<String>,
    pub bcdice_loader: Rc<DynamicLoader>,
}

pub enum Msg {
    AddConnection {
        peer_id: Rc<String>,
        connection: Rc<DataConnection>,
        send_arena: bool,
    },
    SendGetBlockResponse {
        connection: Rc<DataConnection>,
        block_id: U128Id,
    },
}

pub enum On {}

pub struct SkywayConnecter {
    arena: Arena,
    client_id: Rc<String>,
    bcdice_loader: Rc<DynamicLoader>,

    peer: Rc<Peer>,
    room: Rc<MeshRoom>,
    connections: HashMap<Rc<String>, Rc<DataConnection>>,
    rquesting_arena_blocks: Rc<RefCell<HashSet<U128Id>>>,

    world: Option<BlockMut<block::World>>,
    chat: Option<BlockMut<block::Chat>>,
}

impl Component for SkywayConnecter {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for SkywayConnecter {}

impl Constructor for SkywayConnecter {
    fn constructor(props: Self::Props) -> Self {
        Self {
            arena: Arena::new(),
            client_id: props.client_id,
            bcdice_loader: props.bcdice_loader,

            peer: props.peer,
            room: props.room,
            connections: HashMap::new(),
            rquesting_arena_blocks: Rc::new(RefCell::new(HashSet::new())),

            world: None,
            chat: None,
        }
    }
}

impl Update for SkywayConnecter {
    fn on_assemble(self: Pin<&mut Self>) -> Cmd<Self> {
        Cmd::list(vec![
            Cmd::batch(kagura::util::Batch::new(|resolve| {
                let resolve = Rc::new(RefCell::new(resolve));
                let a = Closure::wrap(Box::new({
                    let resolve = Rc::clone(&resolve);
                    let peer = Rc::clone(&self.peer);
                    move |peer_id: JsValue| {
                        if let Some(peer_id) = peer_id.as_string() {
                            let peer_id = Rc::new(peer_id);
                            crate::debug::log_2("peerJoin", peer_id.as_str());
                            let connection = Rc::new(peer.connect(peer_id.as_str()));

                            let a = Closure::wrap(Box::new({
                                let resolve = Rc::clone(&resolve);
                                let connection = Rc::clone(&connection);
                                move |data| {
                                    crate::debug::log_2("open", peer_id.as_str());
                                    resolve.borrow_mut()(Cmd::chain(Msg::AddConnection {
                                        peer_id: Rc::clone(&peer_id),
                                        connection: Rc::clone(&connection),
                                        send_arena: true,
                                    }));
                                }
                            })
                                as Box<dyn FnMut(JsValue)>);
                            connection.on("open", Some(a.as_ref().unchecked_ref()));
                            a.forget();

                            let a = Closure::wrap(Box::new(move |error| {
                                crate::debug::log_2("error", &error);
                            })
                                as Box<dyn FnMut(JsValue)>);
                            connection.on("error", Some(a.as_ref().unchecked_ref()));
                            a.forget();
                        }
                    }
                }) as Box<dyn FnMut(JsValue)>);
                self.room.on("peerJoin", Some(a.as_ref().unchecked_ref()));
                a.forget();
            })),
            Cmd::batch(kagura::util::Batch::new(|mut resolve| {
                let a = Closure::wrap(Box::new(move |connection: JsValue| {
                    crate::debug::log_2("connection", &connection);
                    let connection = connection.unchecked_into::<DataConnection>();
                    let connection = Rc::new(connection);
                    let peer_id = Rc::new(connection.id());
                    resolve(Cmd::chain(Msg::AddConnection {
                        peer_id: Rc::clone(&peer_id),
                        connection: Rc::clone(&connection),
                        send_arena: false,
                    }));
                }) as Box<dyn FnMut(JsValue)>);
                self.peer.on("connection", Some(a.as_ref().unchecked_ref()));
                a.forget();
            })),
        ])
    }

    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.client_id = props.client_id;
        Cmd::none()
    }

    fn update(mut self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::AddConnection {
                peer_id,
                connection,
                send_arena,
            } => {
                self.connections
                    .insert(Rc::clone(&peer_id), Rc::clone(&connection));
                Cmd::list(vec![
                    self.batch_connection(peer_id, Rc::clone(&connection)),
                    if send_arena {
                        self.post_arena_ids(Rc::clone(&connection))
                    } else {
                        Cmd::none()
                    },
                ])
            }

            Msg::SendGetBlockResponse {
                connection,
                block_id,
            } => {
                if let Some(block) = self.arena.get_untyped(&block_id) {
                    Cmd::task(async move {
                        let block = block.pack(PackDepth::FirstBlock).await;
                        connection.send_msg(skyway::Msg::GetBlockResponse(block));
                        Cmd::none()
                    })
                } else {
                    Cmd::none()
                }
            }
        }
    }
}

impl Render<Html> for SkywayConnecter {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Room::empty(
            self,
            None,
            room::Props {
                arena: self.arena.as_mut(),
                client_id: Rc::clone(&self.client_id),
                bcdice_loader: Rc::clone(&self.bcdice_loader),

                world: self.world.as_ref().map(|world| BlockMut::clone(&world)),
                chat: self.chat.as_ref().map(|chat| BlockMut::clone(&chat)),
            },
            Sub::none(),
        )
    }
}

impl SkywayConnecter {
    fn batch_connection(&self, peer_id: Rc<String>, connection: Rc<DataConnection>) -> Cmd<Self> {
        Cmd::batch(kagura::util::Batch::new(|resolve| {
            let resolve = Rc::new(RefCell::new(resolve));

            let a = Closure::wrap(Box::new({
                let rquesting_arena_blocks = Rc::clone(&self.rquesting_arena_blocks);
                let arena = self.arena.as_ref();
                let connection = Rc::clone(&connection);
                let resolve = Rc::clone(&resolve);
                move |data: JsValue| match skyway::Msg::from(&data) {
                    skyway::Msg::PostArenaIds {
                        world: world_id,
                        chat: chat_id,
                        mut blocks,
                    } => {
                        crate::debug::log_1("PostArenaIds");
                        blocks.insert(world_id);
                        blocks.insert(chat_id);
                        for block_id in blocks {
                            if !rquesting_arena_blocks.borrow().contains(&block_id)
                                && arena.kind_of(&block_id) == BlockKind::None
                            {
                                rquesting_arena_blocks
                                    .borrow_mut()
                                    .insert(U128Id::clone(&block_id));
                                connection.send_msg(skyway::Msg::GetBlock(block_id));
                            }
                        }
                    }
                    skyway::Msg::PostBlock(data) => {
                        crate::debug::log_2("PostBlock", &data);
                    }
                    skyway::Msg::GetBlock(block_id) => {
                        crate::debug::log_2("GetBlock", &block_id.to_jsvalue());
                        resolve.borrow_mut()(Cmd::chain(Msg::SendGetBlockResponse {
                            connection: Rc::clone(&connection),
                            block_id,
                        }));
                    }
                    skyway::Msg::GetBlockResponse(data) => {
                        crate::debug::log_2("GetBlockResponse", &data);
                    }
                    skyway::Msg::None => {}
                }
            }) as Box<dyn FnMut(JsValue)>);
            connection.on("data", Some(a.as_ref().unchecked_ref()));
            a.forget();
        }))
    }

    fn post_arena_ids(&self, connection: Rc<DataConnection>) -> Cmd<Self> {
        let blocks = self.arena.ids().collect::<HashSet<_>>();
        let mut world = self.world.as_ref().map(|world| world.id());
        let mut chat = self.world.as_ref().map(|chat| chat.id());
        for block_id in &blocks {
            if world.is_none() {
                if let Some(block) = self.arena.get::<block::World>(&block_id) {
                    world = Some(block.id());
                }
            }

            if chat.is_none() {
                if let Some(block) = self.arena.get::<block::Chat>(&block_id) {
                    chat = Some(block.id());
                }
            }
        }

        if let Some((world, chat)) = join_some!(world, chat) {
            connection.send_msg(skyway::Msg::PostArenaIds {
                world,
                chat,
                blocks,
            });
        }

        Cmd::none()
    }
}
