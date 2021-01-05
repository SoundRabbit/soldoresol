use super::super::{btn, text};
use crate::{
    block::{self, BlockId},
    idb, model,
    random_id::U128Id,
    resource::Resource,
    JsObject, Promise, Timestamp,
};
use kagura::prelude::*;
use std::{collections::HashMap, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};

pub type SaveTable = Component<Props, Sub>;

pub struct Props {
    pub table_db: Rc<web_sys::IdbDatabase>,
    pub common_db: Rc<web_sys::IdbDatabase>,
    pub block_field: block::Field,
    pub resource: Resource,
    pub table_id: BlockId,
}

pub struct State {
    finished: bool,
    cmd_queue: model::CmdQueue<Msg, Sub>,
}

pub enum Msg {
    Close,
    Finish(Option<Rc<web_sys::IdbDatabase>>),
}

pub enum Sub {
    Close,
    DbVersionIsUpdated(Rc<web_sys::IdbDatabase>),
}

pub fn new() -> SaveTable {
    Component::new(init, update, render)
}

fn init(state: Option<State>, props: Props) -> (State, Cmd<Msg, Sub>, Vec<Batch<Msg>>) {
    if let Some(state) = state {
        return (state, Cmd::none(), vec![]);
    }

    let state = State {
        finished: false,
        cmd_queue: model::CmdQueue::new(),
    };

    let table_db = props.table_db;
    let common_db = props.common_db;
    let block_field = props.block_field;
    let resource = props.resource;
    let table_id = props.table_id;

    let blocks = {
        let mut blocks = block_field.dependents_of::<block::Table>(&table_id);
        blocks.insert(table_id.clone());
        blocks
    };
    let resources = block_field.resources_of::<block::Table>(&table_id);
    let table_name = block_field
        .get::<block::Table>(&table_id)
        .map(|t| t.name().clone())
        .unwrap_or(String::new());

    let blocks = block_field.pack_listed(blocks.into_iter().collect());
    let resources = resource.pack_listed(resources.into_iter().collect());

    let cmd = Cmd::task(move |resolve| {
        blocks
            .and_then(move |blocks| {
                resources.map(move |resources| {
                    if let (Some(b), Some(r)) = (blocks, resources) {
                        Some((b, r))
                    } else {
                        None
                    }
                })
            })
            .then(move |x| {
                if let Some((blocks, resources)) = x {
                    let table_id = Rc::new(table_id.to_id().to_u128().to_string());
                    let names = table_db.object_store_names();
                    let mut has_table_object = false;
                    for i in 0..names.length() {
                        if let Some(name) = names.item(i) {
                            if name == table_id.as_str() {
                                has_table_object = true;
                            }
                        }
                    }
                    crate::debug::log_1(format!("has_table_object:{}", has_table_object));
                    if has_table_object {
                        save_blocks(
                            Rc::clone(&table_db),
                            Rc::clone(&common_db),
                            Rc::clone(&table_id),
                            table_name,
                            blocks,
                            resources,
                        )
                        .then(move |_| resolve(Msg::Finish(None)));
                    } else {
                        idb::create_object_strage(&table_db, table_id.as_ref()).then(
                            move |database| {
                                crate::debug::log_1("save_blocks");
                                let database = Rc::new(database.unwrap());
                                save_blocks(
                                    Rc::clone(&database),
                                    Rc::clone(&common_db),
                                    Rc::clone(&table_id),
                                    table_name,
                                    blocks,
                                    resources,
                                )
                                .then(move |_| resolve(Msg::Finish(Some(database))))
                            },
                        );
                    }
                }
            });
    });

    (state, cmd, vec![])
}

fn save_blocks(
    table_db: Rc<web_sys::IdbDatabase>,
    common_db: Rc<web_sys::IdbDatabase>,
    table_id: Rc<String>,
    table_name: String,
    blocks: Vec<(BlockId, JsValue)>,
    resources: Vec<(U128Id, JsValue)>,
) -> Promise<JsValue> {
    let mut transaction = Promise::none();
    for (block_id, block) in blocks {
        let table_db = Rc::clone(&table_db);
        let table_id = Rc::clone(&table_id);
        if block_id.to_id().to_string() == *table_id {
            transaction = transaction
                .and_then(move |_| idb::assign(table_db, table_id, JsValue::from("data"), block));
        } else {
            transaction = transaction
                .and_then(move |_| idb::assign(table_db, table_id, block_id.to_jsvalue(), block));
        }
    }
    let r_ids = js_sys::Array::new();
    for (r_id, data) in resources {
        let common_db = Rc::clone(&common_db);
        r_ids.push(&r_id.to_jsvalue());
        transaction = transaction.and_then(move |_| {
            idb::assign(
                common_db,
                Rc::new(String::from("resources")),
                r_id.to_jsvalue(),
                data,
            )
        });
    }
    {
        let common_db = Rc::clone(&common_db);
        let table_id = table_id.clone();
        let value: js_sys::Object = object! {
            name: table_name.as_str(),
            timestamp: js_sys::Date::now(),
            resources: r_ids
        }
        .into();
        transaction = transaction.and_then(move |_| {
            idb::assign(
                common_db,
                Rc::new(String::from("tables")),
                JsValue::from(table_id.as_str()),
                value.into(),
            )
        });
    }
    transaction
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::Close => {
            state.cmd_queue.enqueue(Cmd::sub(Sub::Close));
            state.cmd_queue.dequeue()
        }
        Msg::Finish(table_db) => {
            state.finished = true;
            if let Some(table_db) = table_db {
                state
                    .cmd_queue
                    .enqueue(Cmd::sub(Sub::DbVersionIsUpdated(table_db)));
            }
            state.cmd_queue.dequeue()
        }
    }
}

fn render(state: &State, _: Vec<Html>) -> Html {
    super::container(
        Attributes::new(),
        Events::new(),
        vec![super::frame(
            6,
            Attributes::new(),
            Events::new(),
            vec![
                super::header(
                    Attributes::new().class("keyvalue").class("keyvalue-rev"),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new().class("text-label"),
                            Events::new(),
                            vec![Html::text("")],
                        ),
                        Html::div(
                            Attributes::new().class("linear-h"),
                            Events::new(),
                            vec![btn::close(
                                Attributes::new(),
                                Events::new().on_click(move |_| Msg::Close),
                            )],
                        ),
                    ],
                ),
                super::body(
                    Attributes::new().class("linear-v"),
                    Events::new(),
                    if state.finished {
                        vec![
                            text::div("保存が完了しました。"),
                            btn::primary(
                                Attributes::new(),
                                Events::new().on_click(|_| Msg::Close),
                                vec![Html::text("OK")],
                            ),
                        ]
                    } else {
                        vec![text::div("保存中…")]
                    },
                ),
                super::footer(Attributes::new(), Events::new(), vec![]),
            ],
        )],
    )
}
