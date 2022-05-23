use super::super::organism::{
    room_modeless, room_modeless_chat::ChatUser, tab_modeless_container::TabModelessList,
    table_menu::TableMenu,
};
use super::{Room, ShowingModal};
use crate::arena::{block, user, Arena, ArenaMut, BlockMut};
use crate::table::Table;
use kagura::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

macro_rules! new_channel {
    ($arena:expr,$chat:expr,$name:expr) => {{
        let arena = &mut ($arena);
        let chat = &mut ($chat);

        let mut chat_channel = block::ChatChannel::new();

        chat_channel.name_set(String::from($name));

        let chat_channel = arena.insert(chat_channel);

        chat.update(|chat: &mut block::Chat| {
            chat.channels_push(chat_channel.clone());
        });

        chat_channel
    }};
}

impl Constructor for Room {
    fn constructor(props: Self::Props) -> Self {
        let mut arena = ArenaMut::clone(&props.arena);
        let mut chat = arena.insert(block::Chat::new());

        new_channel!(arena, chat, "メイン");
        new_channel!(arena, chat, "サブ");

        let mut table = block::Table::new();
        let craftboard = block::Craftboard::new(table.default_is_bind_to_grid(), [0.0, 0.0, 0.0]);
        table.push_craftboard(arena.insert(craftboard));

        let mut scene = block::Scene::new();
        scene.tables_push(arena.insert(table));

        let mut world = block::World::new();
        world.push_scenes(arena.insert(scene));
        let world = arena.insert(world);

        let me = arena.insert(user::Player::new());

        let chat_users = vec![ChatUser::Player(BlockMut::clone(&me))];

        let modeless_container = Rc::new(RefCell::new(TabModelessList::new()));
        super::open_modeless(
            &props.client_id,
            &arena,
            &world,
            &modeless_container,
            room_modeless::ContentData::Chat {
                data: BlockMut::clone(&chat),
                user: ChatUser::Player(BlockMut::clone(&me)),
            },
        );

        Self {
            arena: arena,
            local_arena: Arena::new(),
            client_id: props.client_id,
            bcdice_loader: props.bcdice_loader,

            chat: chat,
            world: world,
            me: me,

            table: Rc::new(RefCell::new(Table::new())),
            modeless_container: modeless_container,

            table_tool: TableMenu::initial_selected(),
            ok_to_catch_file: true,
            is_2d_mode: false,
            is_debug_mode: false,

            chat_users: chat_users,

            showing_contextmenu: None,
            showing_modal: ShowingModal::None,
        }
    }
}
